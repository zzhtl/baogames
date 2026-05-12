use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct BubbleCell {
    pub col: i32,
    pub row: i32,
}

#[derive(Component, Clone, Copy)]
pub struct BubbleColor(pub u8);

#[derive(Component)]
pub struct GridBubble;

#[derive(Component)]
pub struct FlyingBubble {
    pub vel: Vec2,
}

#[derive(Component)]
pub struct PoppingBubble {
    pub life: f32,
}

#[derive(Component)]
pub struct FallingBubble {
    pub vy: f32,
}

#[derive(Component)]
pub struct CannonBarrel;

#[derive(Component)]
pub struct LoadedBubble;

#[derive(Component)]
pub struct NextBubbleSprite;

#[derive(Component)]
pub struct AimDot {
    pub idx: usize,
}

#[derive(Component)]
pub struct BubbleHud;

#[derive(Component)]
pub struct BubbleMessage;

pub fn palette(idx: u8) -> Color {
    match idx {
        0 => Color::srgb(0.94, 0.32, 0.32),
        1 => Color::srgb(0.32, 0.55, 0.96),
        2 => Color::srgb(0.34, 0.82, 0.42),
        3 => Color::srgb(0.98, 0.88, 0.28),
        4 => Color::srgb(0.82, 0.42, 0.96),
        _ => Color::srgb(0.96, 0.6, 0.24),
    }
}

pub fn palette_dark(idx: u8) -> Color {
    let c = palette(idx).to_srgba();
    Color::srgb(c.red * 0.55, c.green * 0.55, c.blue * 0.55)
}
