use bevy::prelude::*;
use bevy::text::Font;

use super::constants::{Z_BACKGROUND, Z_SPRITE, Z_TEXT};
use super::sprite_def::SpriteDef;

#[derive(Resource, Clone)]
pub struct UiFont(pub Handle<Font>);

impl FromWorld for UiFont {
    fn from_world(world: &mut World) -> Self {
        let font = Font::try_from_bytes(
            include_bytes!("../../assets/fonts/NotoSansCJK-Regular.ttc").to_vec(),
        )
        .expect("embedded CJK font must be valid");
        let mut fonts = world.resource_mut::<Assets<Font>>();
        UiFont(fonts.add(font))
    }
}

pub fn rect<'a, M: Component>(
    commands: &'a mut Commands,
    pos: Vec2,
    size: Vec2,
    color: Color,
    marker: M,
) -> EntityCommands<'a> {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(pos.extend(Z_SPRITE)),
        marker,
    ))
}

pub fn background_rect<'a, M: Component>(
    commands: &'a mut Commands,
    pos: Vec2,
    size: Vec2,
    color: Color,
    marker: M,
) -> EntityCommands<'a> {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(pos.extend(Z_BACKGROUND)),
        marker,
    ))
}

pub fn panel<'a, M: Component + Clone>(
    commands: &'a mut Commands,
    pos: Vec2,
    size: Vec2,
    fill: Color,
    border: Color,
    marker: M,
) -> EntityCommands<'a> {
    rect(commands, pos, size, border, marker.clone());
    rect(commands, pos, size - Vec2::splat(4.0), fill, marker)
}

pub fn text<'a, M: Component>(
    commands: &'a mut Commands,
    font: &UiFont,
    value: &str,
    pos: Vec2,
    size: f32,
    color: Color,
    marker: M,
) -> EntityCommands<'a> {
    commands.spawn((
        Text2d::new(value),
        TextFont::from_font_size(size).with_font(font.0.clone()),
        TextColor(color),
        Transform::from_translation(pos.extend(Z_TEXT)),
        marker,
    ))
}

/// 按 [`SpriteDef`] 生成一个精灵：透明父实体(custom_size=def.size) + 各子矩形。
///
/// - `child_marker`：打到每个子矩形上的组件（通常是 `GameEntity`，便于统一清理）。
/// - `parent_extra`：父实体的额外组件（如玩家/敌人逻辑组件，外加 `GameEntity`）。
///
/// 返回父实体 id，方便调用方继续挂子节点或记录。子矩形按各自 `base_z + dz` 叠加，
/// 与离线预览光栅器的画家算法一致。
pub fn spawn_sprite_def(
    commands: &mut Commands,
    def: &SpriteDef,
    pos: Vec2,
    base_z: f32,
    child_marker: impl Component + Clone,
    parent_extra: impl Bundle,
) -> Entity {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), def.size),
            Transform::from_translation(pos.extend(base_z)),
            parent_extra,
        ))
        .id();
    for p in def.parts {
        commands
            .spawn((
                Sprite::from_color(p.color, Vec2::new(p.w, p.h)),
                Transform::from_translation(Vec3::new(p.dx, p.dy, base_z + p.dz)),
                child_marker.clone(),
            ))
            .insert(ChildOf(parent));
    }
    parent
}
