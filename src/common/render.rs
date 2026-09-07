use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{Font, FontHinting, FontSmoothing};

use super::constants::{Z_BACKGROUND, Z_SPRITE, Z_TEXT};
use super::px::snap2;
use super::sprite_def::SpriteDef;

#[derive(Resource, Clone)]
pub struct UiFont(pub Handle<Font>);

impl FromWorld for UiFont {
    fn from_world(world: &mut World) -> Self {
        let font = Font::try_from_bytes(
            include_bytes!("../../assets/fonts/fusion-pixel-12px-proportional-zh_hans.ttf").to_vec(),
        )
        .expect("embedded Fusion Pixel CJK font must be valid");
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

/// 仅当内容变化时才写入 `Text2d`。
///
/// HUD 每帧无条件覆写文本会触发 Bevy 对该实体重新排版（CJK 字形 reshape），
/// 即使字符串没变；先比较再写可让变更检测在值未变时完全跳过。
pub fn set_text(dst: &mut Mut<Text2d>, value: &str) {
    if dst.0 != value {
        value.clone_into(&mut dst.0);
    }
}

/// 点阵字体专用的 `TextFont`：关掉灰度抗锯齿。
///
/// 配合渲染目标的 `scale_factor`（见 [`crate::common::pixel_canvas`]），字形按画布
/// 像素原生光栅化；再关掉抗锯齿，才能拿到干净的点阵边缘而不是一圈灰边。
pub fn pixel_font(font: &UiFont, size: f32) -> TextFont {
    TextFont::from_font_size(size)
        .with_font(font.0.clone())
        .with_font_smoothing(FontSmoothing::None)
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
    text_anchored(commands, font, value, pos, size, color, Anchor::CENTER, marker)
}

/// 带锚点的文本。左对齐的标签列 / 右对齐的数值列靠它才能真正对齐，
/// 而不是靠「猜这串字有多宽」去凑中心坐标。位置吸附到画布像素网格。
#[allow(clippy::too_many_arguments)]
pub fn text_anchored<'a, M: Component>(
    commands: &'a mut Commands,
    font: &UiFont,
    value: &str,
    pos: Vec2,
    size: f32,
    color: Color,
    anchor: Anchor,
    marker: M,
) -> EntityCommands<'a> {
    commands.spawn((
        Text2d::new(value),
        pixel_font(font, size),
        // 字形吸附到整数 X 坐标，避免半像素起笔把竖笔画糊成两列
        FontHinting::Enabled,
        anchor,
        TextColor(color),
        Transform::from_translation(snap2(pos).extend(Z_TEXT)),
        marker,
    ))
}

/// 按 [`SpriteDef`] 生成一个精灵：透明父实体(custom_size=def.size) + 各子矩形。
///
/// - `child_marker`：打到每个子矩形上的组件（通常是 `GameEntity`，便于统一清理）。
/// - `parent_extra`：父实体的额外组件（如玩家/敌人逻辑组件，外加 `GameEntity`）。
///
/// 返回父实体 id，方便调用方继续挂子节点或记录。父实体位于 `base_z`，子矩形
/// 使用局部 `dz`，因此最终世界层级为 `base_z + dz`，与离线预览的画家算法一致。
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
                Transform::from_translation(Vec3::new(p.dx, p.dy, p.dz)),
                child_marker.clone(),
            ))
            .insert(ChildOf(parent));
    }
    parent
}

/// 给已有父实体按 [`SpriteDef`] 挂子矩形（与离线预览同源）。
///
/// 与 [`spawn_sprite_def`] 的区别：本函数不建父实体，且子块 z 直接用 part 的
/// `dz`（相对父），适合父实体自带 z 的场景；后者子块 z 为 `base_z + dz`。
pub fn attach_sprite_parts(
    commands: &mut Commands,
    parent: Entity,
    def: &SpriteDef,
    marker: impl Component + Clone,
) {
    for p in def.parts {
        commands
            .spawn((
                Sprite::from_color(p.color, Vec2::new(p.w, p.h)),
                Transform::from_translation(Vec3::new(p.dx, p.dy, p.dz)),
                marker.clone(),
            ))
            .insert(ChildOf(parent));
    }
}
