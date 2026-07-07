//! 推箱子配色（sprites.rs 与游戏渲染、离线预览共用）。

use bevy::prelude::Color;

// 砖墙（5 阶明暗）
pub const WALL_OUTLINE: Color = Color::srgb(0.22, 0.14, 0.09);
pub const WALL_MORTAR: Color = Color::srgb(0.34, 0.22, 0.14);
pub const WALL_BASE: Color = Color::srgb(0.56, 0.38, 0.24);
pub const WALL_HI: Color = Color::srgb(0.72, 0.52, 0.34);
pub const WALL_TOP: Color = Color::srgb(0.82, 0.63, 0.42);
pub const WALL_DK: Color = Color::srgb(0.42, 0.27, 0.17);

// 地板（3 阶——背景保持低对比）
pub const FLOOR_EDGE: Color = Color::srgb(0.12, 0.15, 0.22);
pub const FLOOR_BASE: Color = Color::srgb(0.16, 0.20, 0.28);
pub const FLOOR_HI: Color = Color::srgb(0.20, 0.25, 0.34);

// 目标点（靶心 + 外发光）
pub const GOAL_FAINT: Color = Color::srgba(0.95, 0.52, 0.28, 0.30);
pub const GOAL_RING: Color = Color::srgb(0.95, 0.52, 0.28);
pub const GOAL_INNER: Color = Color::srgb(0.36, 0.20, 0.16);
pub const GOAL_GLOW: Color = Color::srgb(1.0, 0.80, 0.42);

// 木箱（5 阶 + 包角；木纹直接用 DK 阶上色保证可读）
pub const BOX_OUTLINE: Color = Color::srgb(0.40, 0.26, 0.10);
pub const BOX_EDGE: Color = Color::srgb(0.74, 0.52, 0.22);
pub const BOX_FACE: Color = Color::srgb(0.92, 0.72, 0.34);
pub const BOX_HI: Color = Color::srgb(1.0, 0.85, 0.50);
pub const BOX_DK: Color = Color::srgb(0.68, 0.46, 0.19);
pub const BOX_CORNER: Color = Color::srgb(0.52, 0.34, 0.14);

// 完成箱（绿色，同构）
pub const DONE_OUTLINE: Color = Color::srgb(0.10, 0.36, 0.18);
pub const DONE_EDGE: Color = Color::srgb(0.36, 0.78, 0.42);
pub const DONE_FACE: Color = Color::srgb(0.56, 0.92, 0.46);
pub const DONE_HI: Color = Color::srgb(0.78, 1.0, 0.62);
pub const DONE_DK: Color = Color::srgb(0.30, 0.66, 0.30);
pub const DONE_CORNER: Color = Color::srgb(0.18, 0.50, 0.24);

// 工帽小人
pub const P_OUTLINE: Color = Color::srgb(0.10, 0.16, 0.30);
pub const HAT: Color = Color::srgb(0.98, 0.78, 0.22);
pub const HAT_HI: Color = Color::srgb(1.0, 0.92, 0.55);
pub const HAT_DK: Color = Color::srgb(0.82, 0.60, 0.14);
pub const SKIN: Color = Color::srgb(0.99, 0.85, 0.68);
pub const EYE: Color = Color::srgb(0.10, 0.12, 0.20);
pub const MOUTH: Color = Color::srgb(0.85, 0.45, 0.35);
pub const SUIT: Color = Color::srgb(0.28, 0.52, 0.86);
pub const SUIT_HI: Color = Color::srgb(0.44, 0.68, 0.96);
pub const SUIT_DK: Color = Color::srgb(0.18, 0.36, 0.64);
