//! 软件光栅器：把 `SpriteDef`（一组彩色矩形）忠实画成 PNG，复刻 Bevy 2D 渲染语义。
//!
//! - 中心锚点：子矩形覆盖 `[dx-w/2, dx+w/2] × [dy-h/2, dy+h/2]`。
//! - Y 轴翻转：Bevy Y 向上，图片 Y 向下。
//! - 画家算法：按最终 `z = base_z + dz` 升序绘制，大 z 覆盖小 z。
//! - 颜色：用 `Color::to_linear()` 在 linear 空间 alpha 混合，输出再编码回 sRGB —— 与 GPU 一致。

use bevy::color::{Color, ColorToPacked, LinearRgba, Srgba};
use bevy::math::Vec2;
use image::{Rgba, RgbaImage};

use baogames::common::sprite_def::SpriteDef;

/// 逻辑视野矩形（逻辑坐标，Y 向上）。
#[derive(Clone, Copy)]
pub struct View {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl View {
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }
    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }
}

/// linear-RGBA 浮点画布。采样分辨率 = `scale`（像素/逻辑单位）。
pub struct Canvas {
    buf: Vec<[f32; 4]>,
    w: usize,
    h: usize,
    view: View,
    scale: f32,
}

impl Canvas {
    /// 以背景色铺底新建画布。`scale` 是采样分辨率（像素/逻辑单位）。
    pub fn new(view: View, scale: f32, bg: Color) -> Self {
        let w = (view.width() * scale).round().max(1.0) as usize;
        let h = (view.height() * scale).round().max(1.0) as usize;
        Canvas {
            buf: vec![to_linear(bg); w * h],
            w,
            h,
            view,
            scale,
        }
    }

    /// 逻辑矩形 → 像素，alpha over 混合（linear 空间）。
    fn fill_logical(&mut self, cx: f32, cy: f32, w: f32, h: f32, color: Color) {
        let (l, r) = (cx - w * 0.5, cx + w * 0.5);
        let (b, t) = (cy - h * 0.5, cy + h * 0.5);
        // 逻辑 → 像素；Y 翻转：图片顶端对应 view.max_y
        let px_l = ((l - self.view.min_x) * self.scale).round() as i32;
        let px_r = ((r - self.view.min_x) * self.scale).round() as i32;
        let py_t = ((self.view.max_y - t) * self.scale).round() as i32;
        let py_b = ((self.view.max_y - b) * self.scale).round() as i32;
        let src = to_linear(color);
        for py in py_t.max(0)..py_b.min(self.h as i32) {
            for px in px_l.max(0)..px_r.min(self.w as i32) {
                blend_over(&mut self.buf[py as usize * self.w + px as usize], src);
            }
        }
    }

    /// 把一个精灵画到画布上，`origin` 是精灵父原点在逻辑坐标里的位置。
    pub fn draw_sprite(&mut self, def: &SpriteDef, origin: Vec2) {
        // 画家算法：按 dz 升序（base_z 对所有 part 相同，排序只看 dz）
        let mut order: Vec<usize> = (0..def.parts.len()).collect();
        order.sort_by(|&a, &b| def.parts[a].dz.total_cmp(&def.parts[b].dz));
        for &i in &order {
            let p = def.parts[i];
            self.fill_logical(origin.x + p.dx, origin.y + p.dy, p.w, p.h, p.color);
        }
    }

    /// 在逻辑坐标里画一条实色横带（地面等场景装饰）。
    pub fn fill_band(&mut self, cy: f32, h: f32, color: Color) {
        let cx = (self.view.min_x + self.view.max_x) * 0.5;
        self.fill_logical(cx, cy, self.view.width(), h, color);
    }

    /// 最近邻整数放大 `zoom` 倍后存为 PNG。zoom 用于把"实机小尺寸"放大到肉眼可见。
    pub fn save(&self, path: &str, zoom: u32) {
        let zoom = zoom.max(1);
        let (ow, oh) = (self.w as u32 * zoom, self.h as u32 * zoom);
        let mut img = RgbaImage::new(ow, oh);
        for y in 0..oh {
            for x in 0..ow {
                let sx = (x / zoom) as usize;
                let sy = (y / zoom) as usize;
                img.put_pixel(x, y, Rgba(to_u8(self.buf[sy * self.w + sx])));
            }
        }
        img.save(path).unwrap_or_else(|e| panic!("写 PNG 失败 {path}: {e}"));
    }
}

/// 计算精灵所有 part 的逻辑包围盒。
pub fn sprite_bbox(def: &SpriteDef) -> View {
    let mut v = View {
        min_x: f32::MAX,
        min_y: f32::MAX,
        max_x: f32::MIN,
        max_y: f32::MIN,
    };
    for p in def.parts {
        v.min_x = v.min_x.min(p.dx - p.w * 0.5);
        v.max_x = v.max_x.max(p.dx + p.w * 0.5);
        v.min_y = v.min_y.min(p.dy - p.h * 0.5);
        v.max_y = v.max_y.max(p.dy + p.h * 0.5);
    }
    v
}

fn to_linear(c: Color) -> [f32; 4] {
    let l = c.to_linear();
    [l.red, l.green, l.blue, l.alpha]
}

fn to_u8(px: [f32; 4]) -> [u8; 4] {
    Srgba::from(LinearRgba::new(px[0], px[1], px[2], px[3])).to_u8_array()
}

fn blend_over(dst: &mut [f32; 4], src: [f32; 4]) {
    let a = src[3];
    let ia = 1.0 - a;
    dst[0] = src[0] * a + dst[0] * ia;
    dst[1] = src[1] * a + dst[1] * ia;
    dst[2] = src[2] * a + dst[2] * ia;
    dst[3] = a + dst[3] * ia;
}
