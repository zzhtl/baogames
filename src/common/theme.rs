//! 界面设计 token：语义色、间距、描边宽度。
//!
//! 所有尺寸都用 [`crate::common::px::px`] 表达成画布像素 —— 240×180 的画布上
//! 「2 世界单位」这种写法只会得到 0.67 像素的描边，时有时无。
//!
//! 每个游戏的主色不在这里，放在 `GameKind::accent()`：`common/` 不该认识 `GameKind`。

use bevy::prelude::Color;

use super::px::px;

// ---- 尺寸 ----

/// 面板描边：1 画布像素。
pub const BORDER: f32 = px(1.0);
/// 强调描边（选中态）：2 画布像素。
pub const BORDER_THICK: f32 = px(2.0);

pub const GAP_XS: f32 = px(2.0);
pub const GAP_S: f32 = px(4.0);
pub const GAP_M: f32 = px(6.0);
pub const GAP_L: f32 = px(10.0);

/// 12px 正文的行距：字形 12 像素 + 1 像素行间。
pub const LINE: f32 = px(13.0);

// ---- 语义色 ----

/// 页面底色。
pub const BG_DEEP: Color = Color::srgb(0.039, 0.051, 0.078);
/// 卡片 / 面板底色。
pub const SURFACE: Color = Color::srgb(0.071, 0.086, 0.122);
/// 选中态底色。
pub const SURFACE_SEL: Color = Color::srgb(0.118, 0.149, 0.204);
/// 未选中元素的描边。
pub const BORDER_DIM: Color = Color::srgb(0.165, 0.196, 0.259);

pub const TEXT_PRIMARY: Color = Color::srgb(0.902, 0.925, 0.961);
pub const TEXT_MUTED: Color = Color::srgb(0.580, 0.639, 0.741);
pub const TEXT_DIM: Color = Color::srgb(0.361, 0.408, 0.490);

/// 选中 / 高亮的统一强调色。
pub const ACCENT: Color = Color::srgb(1.0, 0.827, 0.302);
pub const DANGER: Color = Color::srgb(1.0, 0.353, 0.302);
pub const SUCCESS: Color = Color::srgb(0.420, 0.851, 0.478);

/// 覆盖层遮罩。
pub const SCRIM: Color = Color::srgba(0.0, 0.0, 0.0, 0.62);
