//! 静态资源（前端构建产物）服务
//!
//! 对应 Java: Spring Boot 的静态资源映射（`classpath:/static/`）+ 前端 history
//! 路由兜底。前端工程 `mrecord-vue` 执行 `yarn build` 后，产物由 vite 直接输出到
//! 本 crate 的 `static/` 目录（见 `mrecord-vue/vite.config.ts` 的 `outDir`）。
//!
//! ## 为什么内嵌而不是运行时读盘
//!
//! 早期实现是 `ServeDir::new("static")`，该相对路径在**处理请求时**才按进程 cwd
//! 解析，存在三个稳健性缺陷：
//!
//! 1. **依赖 cwd**：容器之外（systemd 未设 `WorkingDirectory`、
//!    `cd / && /app/mrecord-rust`、脚本/包管理器包装启动等）会让前端整体 404，
//!    且启动期无任何报错——API 正常、页面空白，极难排查；
//! 2. **命名数据卷导致前端过期**：原 Dockerfile 声明 `VOLUME ["/app"]`，首次
//!    挂命名卷时 Docker 把镜像里的 `static/` 拷进卷，此后镜像升级**也不会**更新
//!    已有卷内容——升级后仍服务旧前端，甚至出现旧前端 + 新 API 的版本错配；
//! 3. **前后端版本无绑定**：`static/` 由独立的构建步骤产出、再单独打进镜像，
//!    与二进制之间没有任何版本关联机制。
//!
//! 与 `schema.sql`、邮件模板（`include_str!`）的既有约定一致，这里用
//! [`include_dir!`] 把整个 `static/` 在**编译期**内嵌进二进制：单文件制品、
//! 零运行时文件依赖，上述三个问题一并消除。
//!
//! ## 覆盖入口
//!
//! 设置环境变量 `MRECORD_STATIC_DIR=/path/to/static` 可切回运行时读盘（内部仍用
//! [`tower_http::services::ServeDir`]），便于**不重编译 Rust** 的前提下替换前端
//! （联调、热修复、灰度对比）。未设置时一律走内嵌产物。

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::http::{Request, Response, StatusCode, header};
use axum::response::IntoResponse;
use include_dir::{Dir, File, include_dir};
use tower::Service;
use tower_http::services::{ServeDir, ServeFile};

/// 编译期内嵌的前端产物根目录。
///
/// `$CARGO_MANIFEST_DIR` 保证无论以何种 cwd 调用 cargo（workspace 根目录、
/// `-p mrecord-rust`、CI 脚本等），都定位到本 crate 的 `static/`。目录由
/// `build.rs` 保证存在（缺失时生成占位页并打印 `cargo:warning`）。
static EMBEDDED: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

/// 静态资源服务
///
/// 一次构造、克隆进 Axum 路由树（[`axum::Router::fallback_service`] 要求 `Clone`）。
/// 变体由构造时的 `MRECORD_STATIC_DIR` 环境变量决定，整个进程生命周期内固定。
#[derive(Clone)]
pub enum StaticService {
    /// 编译期内嵌（默认）：零文件依赖，任何 cwd 下都可用
    Embedded,
    /// `MRECORD_STATIC_DIR` 覆盖：运行时按磁盘目录读，沿用 `ServeDir` 全部行为
    /// （percent-decode、Range 请求、目录索引等）
    Disk(ServeDir<ServeFile>),
}

/// 构造静态资源服务
///
/// `MRECORD_STATIC_DIR` 非空则走磁盘目录，否则走编译期内嵌。磁盘目录不存在时
/// 打印告警（不阻止启动，静态资源会 404，与原 `ServeDir` 行为一致）。
pub fn static_service() -> StaticService {
    match std::env::var("MRECORD_STATIC_DIR") {
        Ok(dir) if !dir.trim().is_empty() => {
            let base = PathBuf::from(dir);
            if !base.exists() {
                tracing::warn!(
                    dir = %base.display(),
                    "MRECORD_STATIC_DIR 指向的目录不存在，静态资源将全部 404（内嵌产物不会被使用）"
                );
            }
            StaticService::Disk(
                ServeDir::new(base.clone()).fallback(ServeFile::new(base.join("index.html"))),
            )
        }
        _ => StaticService::Embedded,
    }
}

impl Service<Request<Body>> for StaticService {
    type Response = Response<Body>;
    type Error = std::convert::Infallible;
    // `Service::Future` 的输出必然是 `Result<Response, Error>`；这里 Error 为
    // `Infallible`，运行时不存在失败分支
    type Future =
        Pin<Box<dyn std::future::Future<Output = Result<Response<Body>, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self {
            Self::Embedded => Poll::Ready(Ok(())),
            // ServeDir 内部会继续询问其 fallback（ServeFile）的就绪状态。
            // 全限定路径是必须的：`ServeDir<F>` 对所有 `Request<ReqBody>` 都实现了
            // `Service`，裸方法调用会让编译器无法确定该用哪个 impl
            Self::Disk(inner) => {
                <ServeDir<ServeFile> as Service<Request<Body>>>::poll_ready(inner, cx)
            }
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match self {
            Self::Embedded => Box::pin(async move { Ok(serve_embedded(req).await) }),
            Self::Disk(inner) => {
                // clone 出独立所有权，避免 async 块借用 `&mut self`（否则 future
                // 不是 'static，无法装进 Box<dyn Future + Send>）
                let mut inner = inner.clone();
                Box::pin(async move {
                    // `ServeDir` 的 Error 亦是 Infallible，body 统一转成 axum 的 Body
                    inner.call(req).await.map(|resp| resp.into_response())
                })
            }
        }
    }
}

/// 从内嵌目录中查找并返回静态文件；未命中时按 SPA 规则兜底到 `index.html`
///
/// 对应原 `ServeDir::new("static").fallback(ServeFile::new("static/index.html"))`：
/// 浏览器刷新 `/login` 这类前端路由地址时，磁盘上没有对应文件，必须回发
/// `index.html` 由前端 vue-router 接管。
async fn serve_embedded(req: Request<Body>) -> Response<Body> {
    // 只取路径部分（`uri().path()` 已剥离 query string）；前置 `/` 去掉后即
    // 内嵌目录内的相对 key。内嵌查找是纯内存 HashMap 匹配，`..` 之类的路径
    // 绝无可能命中真实文件，故不存在目录穿越风险（根本不碰文件系统）。
    let rel = req.uri().path().trim_start_matches('/');

    if let Some(file) = EMBEDDED.get_file(rel) {
        return file_to_response(file);
    }

    index_response()
}

/// 返回内嵌的 `index.html`（SPA 入口）
///
/// 供 [`serve_embedded`] 的 SPA 兜底与飞牛统一网关的「精确前缀根」路由共用。
/// 飞牛桌面入口（`app/ui/config` 的 `url`）打开的是 `/app/mrecord-fnos`
/// 本身，而 axum 嵌套路由的 fallback 只覆盖 `prefix/*`，前缀根必须显式
/// 挂一个返回首页的路由，否则用户点桌面图标会拿到 404 白屏。
fn index_response() -> Response<Body> {
    match EMBEDDED.get_file("index.html") {
        Some(file) => file_to_response(file),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(
                "静态资源未内嵌：缺少 static/index.html。请先在 mrecord-vue 执行 `yarn build`。",
            ))
            .expect("构造 404 响应不会失败"),
    }
}

/// 返回内嵌的 `index.html`（SPA 入口）——飞牛统一网关「精确前缀根」路由使用
///
/// 见 [`index_response`] 的说明：飞牛桌面入口打开 `/app/mrecord-fnos` 本身时，
/// 需要一个显式 handler 把 SPA 首页交给浏览器。
pub async fn serve_index() -> Response<Body> {
    index_response()
}

/// 把内嵌文件转成 HTTP 响应（200 + 按扩展名推断的 Content-Type）
///
/// 内嵌数据存活于二进制的数据段，生命周期为 `'static`，故直接零拷贝塞进
/// `Body`（`Body: From<&'static [u8]>`）。
fn file_to_response(file: &'static File<'static>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_for(file.path()))
        .body(Body::from(file.contents()))
        .expect("构造静态文件响应不会失败")
}

/// 按文件扩展名返回 Content-Type
///
/// 覆盖 vite 产物会出现的全部类型；未知类型退回 `application/octet-stream`
/// （浏览器走下载而非执行，更安全）。
fn mime_for(path: &Path) -> header::HeaderValue {
    let mime: &'static str = match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "map" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        "wasm" => "application/wasm",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    };
    header::HeaderValue::from_static(mime)
}

#[cfg(test)]
mod tests {
    //! 静态资源服务的单元测试。
    //!
    //! 不依赖真实前端构建产物——`build.rs` 保证 `static/index.html` 一定存在
    //! （真实产物或占位页），故内嵌查找与 SPA 兜底逻辑始终可测。

    use super::*;

    #[test]
    fn mime_covers_frontend_asset_types() {
        assert_eq!(
            mime_for(Path::new("index.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            mime_for(Path::new("assets/index-AbCd123.js")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(
            mime_for(Path::new("assets/style.9f8e7d.css")),
            "text/css; charset=utf-8"
        );
        assert_eq!(mime_for(Path::new("favicon.svg")), "image/svg+xml");
        assert_eq!(mime_for(Path::new("app-icon-1024.png")), "image/png");
        assert_eq!(
            mime_for(Path::new("manifest.json")),
            "application/json; charset=utf-8"
        );
        // 大写扩展名同样识别
        assert_eq!(
            mime_for(Path::new("LEGACY.HTML")),
            "text/html; charset=utf-8"
        );
        // 无扩展名退回二进制流
        assert_eq!(
            mime_for(Path::new("no-extension")),
            "application/octet-stream"
        );
    }

    fn request_for(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("构造测试请求不会失败")
    }

    #[tokio::test]
    async fn unknown_route_falls_back_to_index_html() {
        // 前端路由地址（磁盘上不存在对应文件）必须回发 index.html，
        // 否则浏览器刷新 SPA 页面会白屏
        let resp = serve_embedded(request_for("/some/spa/route")).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .expect("缺 Content-Type")
                .to_str()
                .unwrap()
                .contains("text/html"),
            "SPA 兜底响应必须是 HTML"
        );
    }

    #[tokio::test]
    async fn root_serves_index_html() {
        let resp = serve_embedded(request_for("/")).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn query_string_does_not_break_lookup() {
        // query string 不参与内嵌查找
        let resp = serve_embedded(request_for("/login?redirect=/home")).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
