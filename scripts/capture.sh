#!/usr/bin/env bash
# 无头截图。开发机没有显示器（XDG_SESSION_TYPE=tty），所以用 Xvfb 提供 X server、
# lavapipe 提供软件 Vulkan，把游戏真的跑起来再抓画面。
#
#   ./scripts/capture.sh            # 全部 16 个场景（单进程跑完）
#   ./scripts/capture.sh menu_library
#
# 产物写 preview_out/capture/，.gitignore 已忽略 preview_out。
# 存档重定向到同一目录，避免污染入库的 baogames.save。
set -euo pipefail
cd "$(dirname "$0")/.."
OUT=preview_out/capture
mkdir -p "$OUT"

xvfb-run -a -s "-screen 0 1280x720x24" \
  env \
    VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
    VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.json \
    WGPU_BACKEND=vulkan \
    LIBGL_ALWAYS_SOFTWARE=1 \
    BAOGAMES_SAVE_PATH="$OUT/baogames.save" \
    RUST_LOG="${RUST_LOG:-warn}" \
  cargo run --features devtools -- --capture "${1:-all}" --out "$OUT"

# 窗口图是画布的整数倍最近邻放大，降采样回 240×180 拿到逐像素精确的画布。
python3 scripts/win_to_canvas.py "$OUT"/*_win.png
