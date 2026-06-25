#!/usr/bin/env bash
#
# BaoGames · macOS 一键打包脚本
# 在 macOS 上把游戏编译并组装成可双击运行的 BaoGames.app（可选 .dmg）。
#
# 用法：
#   bash scripts/build-macos.sh                # 默认 Intel(x86_64) 包
#   bash scripts/build-macos.sh --universal    # Intel + Apple 芯片通用二进制
#   bash scripts/build-macos.sh --dmg          # 额外产出 dist/BaoGames.dmg
#   bash scripts/build-macos.sh --target aarch64-apple-darwin   # 指定目标三元组
#   bash scripts/build-macos.sh --no-sign      # 不做 ad-hoc 签名
#
# 字体已用 include_bytes! 内嵌进二进制，故 .app 自包含，无需附带 assets 目录。

set -euo pipefail

# ---- 配置 ----
APP_NAME="BaoGames"
BIN_NAME="baogames"
BUNDLE_ID="com.baogames.app"
VERSION="0.1.0"
MIN_MACOS="10.13"

# ---- 默认参数 ----
TARGET="x86_64-apple-darwin"   # 默认 Intel
DO_UNIVERSAL=0
DO_DMG=0
DO_SIGN=1

while [[ $# -gt 0 ]]; do
  case "$1" in
    --universal) DO_UNIVERSAL=1; shift ;;
    --dmg)       DO_DMG=1; shift ;;
    --no-sign)   DO_SIGN=0; shift ;;
    --target)    TARGET="$2"; shift 2 ;;
    -h|--help)   sed -n '2,18p' "$0"; exit 0 ;;
    *) echo "未知参数: $1（-h 查看用法）" >&2; exit 1 ;;
  esac
done

# ---- 定位项目根目录（脚本在 scripts/ 下）----
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "⚠️  本脚本需在 macOS 上运行（用到 cargo --target apple-darwin / sips / iconutil / codesign）。" >&2
  exit 1
fi

DIST="$ROOT/dist"
APP_DIR="$DIST/$APP_NAME.app"
MACOS_DIR="$APP_DIR/Contents/MacOS"
RES_DIR="$APP_DIR/Contents/Resources"

ensure_target() {
  local t="$1"
  if ! rustup target list --installed 2>/dev/null | grep -qx "$t"; then
    echo "→ 安装 rust target: $t"
    rustup target add "$t"
  fi
}

# ---- 编译 ----
echo "==> 编译 release 二进制"
if [[ "$DO_UNIVERSAL" -eq 1 ]]; then
  ensure_target "x86_64-apple-darwin"
  ensure_target "aarch64-apple-darwin"
  cargo build --release --target x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin
  BUILT_BIN="$ROOT/target/${BIN_NAME}-universal"
  lipo -create -output "$BUILT_BIN" \
    "$ROOT/target/x86_64-apple-darwin/release/$BIN_NAME" \
    "$ROOT/target/aarch64-apple-darwin/release/$BIN_NAME"
  echo "→ 已合成通用二进制: $(lipo -archs "$BUILT_BIN")"
else
  ensure_target "$TARGET"
  cargo build --release --target "$TARGET"
  BUILT_BIN="$ROOT/target/$TARGET/release/$BIN_NAME"
fi

# ---- 组装 .app ----
echo "==> 组装 $APP_NAME.app"
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RES_DIR"
cp "$BUILT_BIN" "$MACOS_DIR/$BIN_NAME"
chmod +x "$MACOS_DIR/$BIN_NAME"

# ---- 图标（可选）----
ICON_SRC="$ROOT/assets/icon/icon_1024.png"
ICON_REF=""
if [[ -f "$ICON_SRC" ]]; then
  echo "==> 生成 AppIcon.icns"
  ICONSET="$(mktemp -d)/AppIcon.iconset"
  mkdir -p "$ICONSET"
  for s in 16 32 128 256 512; do
    sips -z "$s" "$s"           "$ICON_SRC" --out "$ICONSET/icon_${s}x${s}.png"      >/dev/null
    sips -z "$((s*2))" "$((s*2))" "$ICON_SRC" --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
  done
  iconutil -c icns "$ICONSET" -o "$RES_DIR/AppIcon.icns"
  ICON_REF="<key>CFBundleIconFile</key><string>AppIcon</string>"
else
  echo "ℹ️  未找到 $ICON_SRC，使用系统默认图标（可后补 1024×1024 PNG 到该路径再打包）。"
fi

# ---- Info.plist ----
cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>$APP_NAME</string>
  <key>CFBundleDisplayName</key><string>$APP_NAME</string>
  <key>CFBundleIdentifier</key><string>$BUNDLE_ID</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleExecutable</key><string>$BIN_NAME</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
  <key>LSMinimumSystemVersion</key><string>$MIN_MACOS</string>
  <key>NSHighResolutionCapable</key><true/>
  $ICON_REF
</dict>
</plist>
PLIST

printf 'APPL????' > "$APP_DIR/Contents/PkgInfo"

# ---- ad-hoc 签名 ----
if [[ "$DO_SIGN" -eq 1 ]]; then
  echo "==> ad-hoc 签名"
  codesign --force --deep --sign - "$APP_DIR"
fi

# ---- 可选 DMG ----
if [[ "$DO_DMG" -eq 1 ]]; then
  echo "==> 生成 DMG"
  DMG="$DIST/$APP_NAME.dmg"
  rm -f "$DMG"
  hdiutil create -volname "$APP_NAME" -srcfolder "$APP_DIR" -ov -format UDZO "$DMG" >/dev/null
  echo "→ $DMG"
fi

echo ""
echo "✅ 完成：$APP_DIR"
du -sh "$APP_DIR" | awk '{print "   体积: "$1}'
echo "   双击运行；若 Gatekeeper 拦截（未公证），右键→打开，或执行："
echo "     xattr -dr com.apple.quarantine \"$APP_DIR\""
