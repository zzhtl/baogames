//! 画布像素 ↔ 世界单位。
//!
//! 游戏世界高 540 单位，渲染到高 180 的画布纹理，所以 **1 画布像素 = 3 世界单位**
//! （见 [`crate::common::pixel_canvas`] 的模块说明）。
//!
//! 所有「我想要 N 个屏幕像素」的场合都应该走 [`px`]，所有落到屏幕上的坐标都应该走
//! [`snap`] —— 否则会画出 0.67 像素宽的描边（时有时无）和亚像素抖动的滚屏。

use bevy::math::Vec2;

/// 一个画布像素对应的世界单位数。
///
/// 与 [`crate::common::settings::DisplayMode::canvas_size`] 强耦合，
/// 由 `world_unit_per_pixel_matches_canvas` 单测锁死。
pub const WORLD_PER_PX: f32 = 3.0;

/// 画布像素 → 世界单位。
#[inline]
pub const fn px(n: f32) -> f32 {
    n * WORLD_PER_PX
}

/// 吸附到画布像素网格。
#[inline]
pub fn snap(v: f32) -> f32 {
    (v / WORLD_PER_PX).round() * WORLD_PER_PX
}

/// 二维版本的 [`snap`]。
#[inline]
pub fn snap2(v: Vec2) -> Vec2 {
    Vec2::new(snap(v.x), snap(v.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::constants::ARENA_H;
    use crate::common::settings::DisplayMode;

    #[test]
    fn world_unit_per_pixel_matches_canvas() {
        for mode in [DisplayMode::Classic4x3, DisplayMode::Widescreen16x9] {
            let (w, h) = mode.canvas_size();
            assert_eq!(h as f32 * WORLD_PER_PX, ARENA_H);
            assert_eq!(w as f32 * WORLD_PER_PX, mode.world_width());
        }
    }

    #[test]
    fn px_converts_screen_pixels_to_world_units() {
        assert_eq!(px(1.0), 3.0);
        assert_eq!(px(12.0), 36.0); // 点阵字体原生字号
        assert_eq!(px(24.0), 72.0);
    }

    #[test]
    fn snap_rounds_to_the_pixel_grid() {
        assert_eq!(snap(0.0), 0.0);
        assert_eq!(snap(1.4), 0.0);
        assert_eq!(snap(1.6), 3.0);
        assert_eq!(snap(-1.6), -3.0);
        assert_eq!(snap(42.0), 42.0);
        // 相机滚屏的典型值：任意浮点必须落回 3 的倍数
        assert_eq!(snap(123.456) % WORLD_PER_PX, 0.0);
    }
}
