#!/bin/bash
# ============================================================================
# Mrecord 飞牛 fnOS 应用打包脚本
#
# 按架构产出两个符合 fnOS 规范的独立包：
#   mrecord-fnos-x86.fpk  （platform=x86，  含 x86_64 二进制）
#   mrecord-fnos-arm.fpk  （platform=arm，  含 aarch64  二进制）
#
# 流程：
#   1. 前置工具链检查（fnpack / cargo-zigbuild / zig / node / yarn）
#   2. 以前端网关前缀构建 mrecord-vue（base=/app/mrecord-fnos/，
#      VITE_API_BASE_URL=/app/mrecord-fnos/api/v2）→ 输出到 mrecord-rust/static/
#   3. 交叉编译 x86_64 / aarch64 两个 musl 静态二进制（前端经 include_dir! 内嵌）
#   4. 逐架构：拷对应二进制到 app/、改 manifest 的 platform 字段、fnpack 打包、
#      改名输出、还原 manifest
#   5. 恢复 mrecord-rust/static/ 为默认 base 构建，避免污染主分支产物
#
# 用法：
#   ./scripts/build.sh              # 双架构全流程
#   ./scripts/build.sh x86_64       # 只出 x86 包
#   ./scripts/build.sh --no-restore # 跳过第 5 步（保留网关版前端产物）
#
# 兼容 macOS 自带 bash 3.2：不使用关联数组、mapfile 等 4.x+ 语法。
# ============================================================================

set -euo pipefail

# ------------------------------- 路径与常量 ---------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PKG_DIR="$(dirname "$SCRIPT_DIR")"                    # mrecord-fnos/
ROOT_DIR="$(dirname "$PKG_DIR")"                       # 仓库根
VUE_DIR="$ROOT_DIR/mrecord-vue"
RUST_DIR="$ROOT_DIR/mrecord-rust"
MANIFEST="$PKG_DIR/manifest"

APP_NAME="mrecord-fnos"
BIN_NAME="mrecord-rust"
GATEWAY_PREFIX="/app/${APP_NAME}"

# 常见安装位置先塞进 PATH（提权 / 非交互 shell 的 PATH 常不含 cargo 与 homebrew）
for _d in "${HOME}/.cargo/bin" "${CARGO_HOME:-}/bin" /opt/homebrew/bin /usr/local/bin "${HOME}/.local/bin"; do
  [ -d "$_d" ] && PATH="${_d}:${PATH}"
done
export PATH

ARCHS="x86_64 aarch64"
RESTORE_FRONTEND=1

# ------------------------------- 参数解析 ------------------------------------
for arg in "$@"; do
  case "$arg" in
    --no-restore) RESTORE_FRONTEND=0 ;;
    x86_64 | aarch64) ARCHS="$arg" ;;
    *)
      echo "未知参数: $arg"
      echo "用法: $0 [x86_64|aarch64] [--no-restore]"
      exit 1
      ;;
  esac
done

# ------------------------------- 工具函数 ------------------------------------
log() { echo "==> $*"; }
fail() { printf '错误: %b\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "未找到 $1。$2"; }

# bash 架构名 → rust target（case 查找，兼容 bash 3.2）
target_for() {
  case "$1" in
    x86_64) echo "x86_64-unknown-linux-musl" ;;
    aarch64) echo "aarch64-unknown-linux-musl" ;;
    *) fail "未知架构: $1（支持 x86_64 / aarch64）" ;;
  esac
}

# bash 架构名 → manifest 的 platform 字段
platform_for() {
  case "$1" in
    x86_64) echo "x86" ;;
    aarch64) echo "arm" ;;
    *) fail "未知架构: $1（支持 x86_64 / aarch64）" ;;
  esac
}

# 改 manifest 的 platform 字段（awk + 临时文件，兼容 macOS / Linux 的 sed 差异）
set_platform() {
  awk -v v="$1" '/^platform=/{print "platform=" v; next} 1' "$MANIFEST" > "$MANIFEST.tmp"
  mv "$MANIFEST.tmp" "$MANIFEST"
}

# 记录原始 platform，退出时无条件还原（避免中途失败留下被改过的 manifest）
ORIGINAL_PLATFORM="$(awk -F= '/^platform=/{print $2}' "$MANIFEST")"
restore_manifest() { set_platform "$ORIGINAL_PLATFORM"; }
trap restore_manifest EXIT

# ------------------------------- 第 1 步：工具链 -----------------------------
log "检查工具链"
need node "请先安装 Node.js（https://nodejs.org）"
need yarn "请先安装 yarn（npm i -g yarn）"
need cargo "请先安装 Rust（https://rustup.rs）"
need cargo-zigbuild "安装：cargo install cargo-zigbuild"

# zig：cargo-zigbuild 需要在 PATH 上找到 zig
if ! command -v zig >/dev/null 2>&1; then
  fail "未找到 zig（cargo-zigbuild 的交叉链接器）。安装示例（macOS ARM）：\n  curl -fSL -o /tmp/zig.tar.xz https://ziglang.org/download/0.15.2/zig-aarch64-macos-0.15.2.tar.xz\n  tar -xf /tmp/zig.tar.xz -C ~/.local && ln -sf ~/.local/zig-aarch64-macos-0.15.2/zig ~/.local/bin/zig"
fi

FNPACK="${FNPACK:-fnpack}"
if ! command -v "$FNPACK" >/dev/null 2>&1; then
  fail "未找到 fnpack。下载对应平台版本：\n  https://developer.fnnas.com/docs/cli/fnpack/#下载\n例如 macOS Apple Silicon：\n  curl -L -o fnpack https://static2.fnnas.com/fnpack/fnpack-1.2.3-darwin-arm64\n  chmod +x fnpack && sudo mv fnpack /usr/local/bin/"
fi

# 确保目标架构已安装
for arch in $ARCHS; do
  target="$(target_for "$arch")"
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
for arch in $ARCHS; do
  target="$(target_for "$arch")"
  log "交叉编译 ${arch}（${target}）"
  (
    cd "$RUST_DIR"
    cargo zigbuild --release --target "$target" --bin "$BIN_NAME"
  )
done

# ------------------------ 第 4 步：逐架构打包 --------------------------------
for arch in $ARCHS; do
  target="$(target_for "$arch")"
  platform="$(platform_for "$arch")"
  src="$RUST_DIR/target/${target}/release/${BIN_NAME}"
  [ -f "$src" ] || fail "编译产物不存在: $src"

  log "打包 ${arch} → ${APP_NAME}-${platform}.fpk"

  # 每个包只含该架构的二进制：先清空 app/ 下所有旧二进制，再拷入当前架构
  rm -f "$PKG_DIR/app/${BIN_NAME}-x86_64" "$PKG_DIR/app/${BIN_NAME}-aarch64"
  cp "$src" "$PKG_DIR/app/${BIN_NAME}-${arch}"

  # manifest 的 platform 字段改为当前架构（脚本退出时由 trap 还原）
  set_platform "$platform"

  (
    cd "$PKG_DIR"
    "$FNPACK" build
  )

  # fnpack 输出固定为 <appname>.fpk，按架构改名
  out="$PKG_DIR/${APP_NAME}.fpk"
  [ -f "$out" ] || fail "fnpack 未生成 ${out}"
  final="$PKG_DIR/${APP_NAME}-${platform}.fpk"
  mv "$out" "$final"
  echo "    ${APP_NAME}-${platform}.fpk（$(du -h "$final" | cut -f1)，含 ${arch} 二进制 $(du -h "$src" | cut -f1)）"
done

# ------------------- 第 5 步：恢复默认前端产物（避免污染主构建） -------------
if [ "$RESTORE_FRONTEND" -eq 1 ]; then
  log "恢复 mrecord-rust/static/ 为默认 base 构建"
  (
    cd "$VUE_DIR"
    yarn build
  )
fi

# ------------------------------- 汇总 ----------------------------------------
log "完成"
echo "  产物目录：${PKG_DIR}/"
for arch in $ARCHS; do
  platform="$(platform_for "$arch")"
  echo "  - ${APP_NAME}-${platform}.fpk"
done
echo ""
echo "下一步：把对应架构的 .fpk 拷到飞牛 fnOS 设备上安装验证（x86 设备用 x86 包，ARM 设备用 arm 包）。"
