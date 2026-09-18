#!/bin/bash
# ============================================================================
# Mrecord 飞牛 fnOS 应用打包脚本
#
# 流程：
#   1. 前置工具链检查（fnpack / cargo-zigbuild / node / yarn）
#   2. 以前端网关前缀构建 mrecord-vue（base=/app/mrecord-fnos/，
#      VITE_API_BASE_URL=/app/mrecord-fnos/api/v2）→ 输出到 mrecord-rust/static/
#   3. 交叉编译 x86_64 / aarch64 两个 musl 静态二进制（前端经 include_dir! 内嵌）
#   4. 拷贝二进制到 mrecord-fnos/app/
#   5. 恢复 mrecord-rust/static/ 为默认 base 构建，避免污染主分支产物
#   6. fnpack build 生成 mrecord-fnos.fpk
#
# 用法：
#   ./scripts/build.sh              # 编译双架构并打包
#   ./scripts/build.sh x86_64       # 只编译 x86_64
#   ./scripts/build.sh --no-restore # 跳过第 5 步（保留网关版前端产物）
# ============================================================================

set -euo pipefail

# ------------------------------- 路径与常量 ---------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PKG_DIR="$(dirname "$SCRIPT_DIR")"                    # mrecord-fnos/
ROOT_DIR="$(dirname "$PKG_DIR")"                       # 仓库根
VUE_DIR="$ROOT_DIR/mrecord-vue"
RUST_DIR="$ROOT_DIR/mrecord-rust"

APP_NAME="mrecord-fnos"
GATEWAY_PREFIX="/app/${APP_NAME}"
BIN_NAME="mrecord-rust"

# 支持的目标架构：bash 名 → rust target
declare -A TARGET_MAP=(
  ["x86_64"]="x86_64-unknown-linux-musl"
  ["aarch64"]="aarch64-unknown-linux-musl"
)

ARCHS=("x86_64" "aarch64")
RESTORE_FRONTEND=1

# ------------------------------- 参数解析 ------------------------------------
for arg in "$@"; do
  case "$arg" in
    --no-restore) RESTORE_FRONTEND=0 ;;
    x86_64 | aarch64) ARCHS=("$arg") ;;
    *)
      echo "未知参数: $arg"
      echo "用法: $0 [x86_64|aarch64|all] [--no-restore]"
      exit 1
      ;;
  esac
done

# ------------------------------- 工具函数 ------------------------------------
log() { echo "==> $*"; }
fail() { echo "错误: $*" >&2; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || fail "未找到 $1。$2"; }

# ------------------------------- 第 1 步：工具链 -----------------------------
log "检查工具链"
need node "请先安装 Node.js（https://nodejs.org）"
need yarn "请先安装 yarn（npm i -g yarn）"
need cargo "请先安装 Rust（https://rustup.rs）"

if ! command -v cargo-zigbuild >/dev/null 2>&1; then
  fail "未找到 cargo-zigbuild（musl 交叉编译工具）。安装：\n  cargo install cargo-zigbuild\n并按其文档安装 zig。"
fi

FNPACK="${FNPACK:-fnpack}"
if ! command -v "$FNPACK" >/dev/null 2>&1; then
  fail "未找到 fnpack。下载对应平台版本：\n  https://developer.fnnas.com/docs/cli/fnpack/#下载\n例如 macOS Apple Silicon：\n  curl -L -o fnpack https://static2.fnnas.com/fnpack/fnpack-1.2.3-darwin-arm64\n  chmod +x fnpack && sudo mv fnpack /usr/local/bin/"
fi

# 确保目标架构已安装
for arch in "${ARCHS[@]}"; do
  target="${TARGET_MAP[$arch]}"
  if ! rustup target list --installed 2>/dev/null | grep -q "$target"; then
    log "安装 rust target: $target"
    rustup target add "$target"
  fi
done

# ------------------------- 第 2 步：构建网关版前端 ---------------------------
# base 与 VITE_API_BASE_URL 都是构建期参数，不需要改任何前端源码：
#   - base: 资产 URL 前缀，vite.config 里硬编码的 '/' 会被 CLI --base 覆盖
#   - VITE_API_BASE_URL: src/utils/request.ts 的 axios baseURL，默认 '/api/v2'
# outDir 固定为 ../mrecord-rust/static（vite.config 配置），会被 emptyOutDir 清空
log "构建网关版前端（base=${GATEWAY_PREFIX}/）"
(
  cd "$VUE_DIR"
  yarn install --silent
  VITE_API_BASE_URL="${GATEWAY_PREFIX}/api/v2" yarn build --base="${GATEWAY_PREFIX}/"
)

# -------------------- 第 3 步：交叉编译 musl 静态二进制 ----------------------
# 前端经 include_dir! 在编译期内嵌，所以每个架构都要在前端就绪后编译。
for arch in "${ARCHS[@]}"; do
  target="${TARGET_MAP[$arch]}"
  log "交叉编译 ${arch}（${target}）"
  (
    cd "$RUST_DIR"
    cargo zigbuild --release --target "$target" --bin "$BIN_NAME"
  )
done

# ------------------------ 第 4 步：拷贝二进制到打包目录 ----------------------
log "拷贝二进制到 ${PKG_DIR}/app/"
for arch in "${ARCHS[@]}"; do
  target="${TARGET_MAP[$arch]}"
  src="$RUST_DIR/target/${target}/release/${BIN_NAME}"
  [ -f "$src" ] || fail "编译产物不存在: $src"
  cp "$src" "$PKG_DIR/app/${BIN_NAME}-${arch}"
  echo "    ${BIN_NAME}-${arch}（$(du -h "$src" | cut -f1)）"
done

# ------------------- 第 5 步：恢复默认前端产物（避免污染主构建） -------------
if [ "$RESTORE_FRONTEND" -eq 1 ]; then
  log "恢复 mrecord-rust/static/ 为默认 base 构建"
  (
    cd "$VUE_DIR"
    yarn build
  )
fi

# ------------------------------- 第 6 步：打包 -------------------------------
log "执行 fnpack build"
(
  cd "$PKG_DIR"
  "$FNPACK" build
)

FPK="$PKG_DIR/${APP_NAME}.fpk"
if [ -f "$FPK" ]; then
  log "完成: ${FPK}（$(du -h "$FPK" | cut -f1)）"
  echo ""
  echo "下一步：把 .fpk 拷到飞牛 fnOS 设备上安装验证。"
else
  fail "未找到生成的 ${FPK}，请检查 fnpack 输出"
fi
