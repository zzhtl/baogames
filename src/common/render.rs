use bevy::prelude::*;
use bevy::text::Font;

use super::constants::{Z_BACKGROUND, Z_SPRITE, Z_TEXT};

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
