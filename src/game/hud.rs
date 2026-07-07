//! 共享 HUD 原语。
//!
//! 每局开始时 `setup_game` 在相机下挂一个 HUD 根实体，并把它传给各游戏的
//! `setup_stage`；HUD 元素统一作为它的子节点：相机空间坐标 = 静止相机游戏的
//! 世界坐标，而魂斗罗 / 超级玛丽滚屏时 HUD 自动跟随，无需手工同步。

use bevy::prelude::*;

use crate::common::render::UiFont;

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
            TextFont::from_font_size(size).with_font(font.0.clone()),
            TextColor(color),
            Transform::from_translation(pos.extend(1.0)),
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
    commands.spawn((
        Sprite::from_color(border, size),
        Transform::from_translation(pos.extend(0.0)),
        GameEntity,
        ChildOf(root),
    ));
    commands.spawn((
        Sprite::from_color(fill, size - Vec2::splat(4.0)),
        Transform::from_translation(pos.extend(0.1)),
        GameEntity,
        ChildOf(root),
    ));
}
