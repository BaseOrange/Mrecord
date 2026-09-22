//! 静态资源构建守卫
//!
//! 前端产物 `static/` 由 `mrecord-vue` 的 `yarn build` 生成（vite `outDir` 指向
//! 本目录），且被 `.gitignore` 忽略，克隆仓库后默认不存在。而
//! [`static_files`] 模块通过 `include_dir!` 在**编译期**把整个 `static/` 内嵌
//! 进二进制——目录若缺失，编译会直接失败。
//!
//! 本脚本在 `static/index.html` 缺失时自动生成一份占位页并打印 `cargo:warning`：
//! - `cargo check` / `cargo test` 在未构建前端的情况下仍可正常通过；
//! - 占位页是肉眼可见的「前端未构建」提示，不会被误当作成品页面发布。
//!
//! 生产与容器构建都会先执行前端构建（见 Dockerfile builder 阶段），占位页
//! 永远不会进入正式制品。

use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR 未设置");
    let static_dir = Path::new(&manifest_dir).join("static");
    let index = static_dir.join("index.html");

    if !index.exists() {
        std::fs::create_dir_all(&static_dir).expect("创建 static/ 目录失败");
        std::fs::write(&index, STUB_INDEX_HTML).expect("写入占位 index.html 失败");
        println!("cargo:warning=未检测到前端构建产物 static/index.html，已生成占位页。");
        println!(
            "cargo:warning=请在 mrecord-vue 目录执行 `yarn build`（vite outDir 指向 mrecord-rust/static）后再发布，"
        );
        println!(
            "cargo:warning=否则二进制只会提供占位页，`cargo run` 打开的是「前端未构建」提示。"
        );
    }

    // 前端重新构建后必须重新编译本 crate 才能重新内嵌
    println!("cargo:rerun-if-changed=static");

    // ==================== 诊断信息所需的编译期元数据 ====================
    // `handler::diagnostic` 通过 `env!` 读取这两项，随二进制固化，运行时不可伪造。
    // 获取失败时给 "unknown"，绝不阻断构建（交叉编译环境可能没有 rustc 在 PATH）。
    println!(
        "cargo:rustc-env=RUSTC_VERSION={}",
        rustc_version().unwrap_or_else(|| "unknown".to_string())
    );
    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
}

/// 获取工具链版本字符串（`rustc --version` 的首行输出）
fn rustc_version() -> Option<String> {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .lines()
        .next()
        .map(|line| line.trim().to_string())
}

/// 占位首页：一眼可见的「前端未构建」提示页
const STUB_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mrecord-Rust｜前端未构建</title>
    <style>
        body {
            font-family: -apple-system, "Segoe UI", Roboto, "PingFang SC", "Microsoft YaHei", sans-serif;
            display: flex; align-items: center; justify-content: center;
            min-height: 100vh; margin: 0;
            background: #f5f5f5; color: #333;
        }
        .card {
            max-width: 560px; padding: 40px; border-radius: 12px;
            background: #fff; box-shadow: 0 2px 12px rgba(0, 0, 0, .08);
        }
        h1 { font-size: 20px; margin: 0 0 12px; }
        p { line-height: 1.7; margin: 8px 0; color: #666; }
        code {
            display: inline-block; padding: 2px 8px; border-radius: 6px;
            background: #f0f0f0; font-size: 14px;
        }
        .warn { color: #d48806; font-weight: 600; }
    </style>
</head>
<body>
    <div class="card">
        <h1>⚠️ 前端尚未构建</h1>
        <p>这是由 <code>build.rs</code> 自动生成的占位页——后端二进制内嵌的
            <code>static/</code> 目录中没有真实前端产物。</p>
        <p>请在 <code>mrecord-vue</code> 目录执行：</p>
        <p><code>yarn install &amp;&amp; yarn build</code></p>
        <p>（vite 的 <code>outDir</code> 已指向 <code>../mrecord-rust/static</code>）</p>
        <p>然后重新 <code>cargo build</code> / <code>cargo run</code>。</p>
        <p class="warn">正式制品（含 Docker 镜像）不应出现本页面。</p>
    </div>
</body>
</html>
"#;
