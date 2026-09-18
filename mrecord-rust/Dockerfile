# =====================================================================
# Mrecord-Rust 容器化 —— 多阶段构建
#
# 对应 Java 版根目录 `dockerfile`。构建上下文为本目录（mrecord-rust/）：
#   docker build -t mrecord-rust ./mrecord-rust
#
# 阶段划分：
#   deps    —— 只拷 Cargo.toml/Cargo.lock + 空壳 src，缓存依赖编译产物
#   builder —— 拷真实源码 + 前端产物，只编译业务代码（依赖命中 deps 缓存）
#   runtime —— alpine 瘦身运行镜像，仅含一个自包含二进制（前端已内嵌）
#
# 运行时数据落工作目录 /app：data.db（SQLite，首次启动自动建表）与
# exports/（导出 Excel），两者随 /app 数据卷持久化。
# =====================================================================

# ----------------------------- 编译阶段 -----------------------------
FROM rust:alpine AS deps

# libsqlite3-sys 默认从源码编译 SQLite，需要 C 编译器与 musl 头文件；
# 仅存在于编译镜像，不进入运行镜像
RUN apk add --no-cache gcc musl-dev

WORKDIR /app

# 先拷依赖描述文件，构造空壳工程编译全部第三方依赖
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() { println!(\"stub\"); }" > src/main.rs && \
    # 预热 release 依赖缓存
    cargo build --release

# ----------------------------- 构建阶段 -----------------------------
FROM rust:alpine AS builder

RUN apk add --no-cache gcc musl-dev

WORKDIR /app

# 复用 deps 阶段编译好的依赖产物，源码变更时只需重编译业务代码
COPY --from=deps /app/target ./target
COPY --from=deps /usr/local/cargo /usr/local/cargo

COPY Cargo.toml Cargo.lock ./
# include_str! / include_dir! 在编译期读取，schema.sql、邮件模板与前端产物
# 必须随源码一起拷入 builder 阶段；运行阶段不再需要任何静态资源文件
COPY schema.sql ./
COPY src/ ./src/
COPY resources/ ./resources/
# 前端构建产物：内嵌进二进制，运行镜像里不再保留 static/ 目录
COPY static/ ./static/

RUN cargo build --release --bin mrecord-rust

# ----------------------------- 运行阶段 -----------------------------
FROM alpine:latest AS runtime

# lettre 通过 rustls 发 SMTP 时需要 CA 证书；时区数据供 chrono 的 Local 使用
RUN apk add --no-cache ca-certificates tzdata && \
    update-ca-certificates && \
    # 非 root 用户运行容器
    addgroup -S app && adduser -S app -G app

WORKDIR /app

# 二进制（musl 静态链接），前端产物已在编译期内嵌，运行镜像无需任何静态资源文件
COPY --from=builder /app/target/release/mrecord-rust ./

# 预建导出目录并移交属主；首次挂载命名数据卷时 Docker 会把 /app 内容
# （二进制、exports/）复制进卷，故需提前赋权。由于前端已内嵌进二进制，
# 卷内容再也无法用旧前端「遮蔽」镜像里的新前端（升级镜像即升级前端）
RUN mkdir -p exports && chown -R app:app /app

USER app

# /app 为数据卷：持久化 data.db + exports/
VOLUME ["/app"]

# 容器内必须监听 0.0.0.0，否则宿主机映射 2333 端口无法访问
ENV MRECORD_HOST=0.0.0.0 \
    MRECORD_PORT=2333 \
    RUST_LOG="mrecord_rust=info,tower_http=info" \
    TZ=Asia/Shanghai

EXPOSE 2333

ENTRYPOINT ["/app/mrecord-rust"]
