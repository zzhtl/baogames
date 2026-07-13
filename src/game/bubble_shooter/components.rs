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
    pub spin: f32,
}

#[derive(Component)]
pub struct PoppingBubble {
    pub life: f32,
}

#[derive(Component)]
pub struct FallingBubble {
    pub vy: f32,
    pub vx: f32,
    pub angular_speed: f32,
}

#[derive(Component)]
pub struct SettlingBubble {
    pub life: f32,
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
pub struct AimLanding;

#[derive(Component)]
pub struct BubbleCeiling;

#[derive(Component)]
pub struct BubbleDeadLine;

#[derive(Component)]
pub struct BubbleHud;

#[derive(Component)]
pub struct BubbleMessage;

pub fn palette(idx: u8) -> Color {
    // 更明快、更适合儿童的糖果色
    match idx {
        0 => Color::srgb(1.00, 0.42, 0.50), // 草莓粉
        1 => Color::srgb(0.40, 0.74, 1.00), // 天空蓝
        2 => Color::srgb(0.50, 0.92, 0.55), // 嫩绿
        3 => Color::srgb(1.00, 0.86, 0.38), // 蜂蜜黄
        4 => Color::srgb(0.78, 0.58, 1.00), // 葡萄紫
        _ => Color::srgb(1.00, 0.68, 0.36), // 蜜橘
    }
}
