//! 共享 HUD 原语。
//!
//! 每局开始时 `setup_game` 在相机下挂一个 HUD 根实体，并把它传给各游戏的
//! `setup_stage`；HUD 元素统一作为它的子节点：相机空间坐标 = 静止相机游戏的
//! 世界坐标，而魂斗罗 / 超级玛丽滚屏时 HUD 自动跟随，无需手工同步。

use bevy::prelude::*;

use bevy::sprite::Anchor;
use bevy::text::FontHinting;

use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, pixel_font};
use crate::common::theme::BORDER;

use super::model::GameEntity;

/// 在 HUD 根下放一段文本；`extra` 用于挂各游戏自己的更新 marker。
#[allow(clippy::too_many_arguments)]
pub(super) fn hud_text(
    commands: &mut Commands,
    font: &UiFont,
    root: Entity,
    value: &str,
    pos: Vec2,
    size: f32,
    color: Color,
    extra: impl Bundle,
) -> Entity {
    commands
        .spawn((
            Text2d::new(value),
            pixel_font(font, size),
            FontHinting::Enabled,
            TextColor(color),
            Transform::from_translation(snap2(pos).extend(1.0)),
            GameEntity,
            ChildOf(root),
            extra,
        ))
        .id()
}

/// 在 HUD 根下放一个双层面板（描边 + 深色填充）。
pub(super) fn hud_panel(
    commands: &mut Commands,
    root: Entity,
    pos: Vec2,
    size: Vec2,
    fill: Color,
    border: Color,
) {
    let pos = snap2(pos);
    commands.spawn((
        Sprite::from_color(border, size),
        Transform::from_translation(pos.extend(0.0)),
        GameEntity,
        ChildOf(root),
    ));
    commands.spawn((
        // 描边必须正好 1 画布像素：原来的 -4.0 是 0.67 像素，粗细会时有时无
        Sprite::from_color(fill, size - Vec2::splat(BORDER * 2.0)),
        Transform::from_translation(pos.extend(0.1)),
        GameEntity,
        ChildOf(root),
    ));
}

/// HUD 顶栏 / 底栏：一条通栏面板 + 左右两块对齐的文本。
///
/// 记忆翻翻乐与推箱子的版式完全一样，太空射击 / 泡泡龙的消息条也复用它。
pub(super) fn hud_bar(
    commands: &mut Commands,
    root: Entity,
    y_px: f32,
    fill: Color,
    border: Color,
) {
    hud_panel(
        commands,
        root,
        Vec2::new(0.0, px(y_px)),
        Vec2::new(px(236.0), px(15.0)),
        fill,
        border,
    );
}

/// 带锚点的 HUD 文本：左对齐的标签列 / 右对齐的数值列靠它才真的对齐。
#[allow(clippy::too_many_arguments)]
pub(super) fn hud_text_anchored(
    commands: &mut Commands,
    font: &UiFont,
    root: Entity,
    value: &str,
    pos: Vec2,
    size: f32,
    color: Color,
    anchor: Anchor,
    extra: impl Bundle,
) -> Entity {
    commands
        .spawn((
            Text2d::new(value),
            pixel_font(font, size),
            FontHinting::Enabled,
            anchor,
            TextColor(color),
            Transform::from_translation(snap2(pos).extend(1.0)),
            GameEntity,
            ChildOf(root),
            extra,
        ))
        .id()
}
