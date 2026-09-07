#!/usr/bin/env python3
"""把 1280×720 的窗口截图还原成 240×180 的真实画布像素。

窗口是画布的整数倍最近邻放大（4:3 下 4×，左右各 160px 黑边），所以降采样是无损的。
脚本会校验每个放大块内颜色一致 —— 一旦画布尺寸或缩放倍数变了就报错，而不是
悄悄产出一张糊图。
"""
import sys
from pathlib import Path
from PIL import Image

CANVAS_W, CANVAS_H = 240, 180  # DisplayMode::Classic4x3


def recover(path: Path) -> Path:
    im = Image.open(path).convert("RGB")
    w, h = im.size
    scale = h // CANVAS_H
    if scale < 1 or h % CANVAS_H:
        raise SystemExit(f"{path}: 高度 {h} 不是画布高 {CANVAS_H} 的整数倍")
    content_w = CANVAS_W * scale
    left = (w - content_w) // 2
    crop = im.crop((left, 0, left + content_w, h))

    px = crop.load()
    for by in range(0, h, scale):
        for bx in range(0, content_w, scale):
            c = px[bx, by]
            for dy in range(scale):
                for dx in range(scale):
                    if px[bx + dx, by + dy] != c:
                        raise SystemExit(
                            f"{path}: ({bx + dx},{by + dy}) 处放大块不均匀，"
                            f"窗口不是画布的 {scale}× 最近邻放大"
                        )

    out = path.with_name(path.name.removesuffix("_win.png") + "_canvas.png")
    crop.resize((CANVAS_W, CANVAS_H), Image.NEAREST).save(out)
    return out


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        print(recover(Path(arg)))
