#!/usr/bin/env python3
"""保留游戏截图的原生分辨率，只裁掉 4:3 黑边；--wide 用于 16:9 截图。

大厅原图不裁剪。游戏现在按屏幕像素渲染，禁止再降采样回 240×180。
"""
import sys
from pathlib import Path
from PIL import Image


def recover(path: Path, wide: bool = False) -> Path:
    im = Image.open(path).convert("RGB")
    w, h = im.size
    ratio = 16 / 9 if wide else 4 / 3
    content_w = min(w, round(h * ratio))
    content_h = min(h, round(content_w / ratio))
    left, top = (w - content_w) // 2, (h - content_h) // 2
    out = path.with_name(path.name.removesuffix("_win.png") + "_canvas.png")
    im.crop((left, top, left + content_w, top + content_h)).save(out)
    return out


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        if arg == "--wide" or Path(arg).name.startswith("menu_"):
            continue
        print(recover(Path(arg), "--wide" in sys.argv or "_wide_" in Path(arg).name))
