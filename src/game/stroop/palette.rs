//! 色字大挑战配色。
//!
//! 六个色相在 12px 点阵下必须两两可分：相邻色相的明度也拉开，避免红/橙在
//! CRT 扫描线开启时糊成一团。

use bevy::prelude::Color;

/// 题面用的六种颜色，下标即「颜色编号」。
pub const INK: [Color; 6] = [
    Color::srgb(0.98, 0.29, 0.29), // 红
    Color::srgb(1.00, 0.84, 0.20), // 黄
    Color::srgb(0.31, 0.60, 1.00), // 蓝
    Color::srgb(0.36, 0.86, 0.42), // 绿
    Color::srgb(0.76, 0.47, 0.98), // 紫
    Color::srgb(1.00, 0.58, 0.20), // 橙
];

/// 与 [`INK`] 一一对应的汉字。
pub const NAMES: [&str; 6] = ["红", "黄", "蓝", "绿", "紫", "橙"];

pub const SWATCH_BORDER: Color = Color::srgb(0.165, 0.196, 0.259);
pub const SWATCH_SELECTED: Color = Color::srgb(0.361, 0.937, 0.702);
pub const PROMPT_PLAIN: Color = Color::srgb(0.878, 0.918, 0.961);
