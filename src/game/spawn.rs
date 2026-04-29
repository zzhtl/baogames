use bevy::prelude::*;
use std::time::Duration;

use crate::common::render::{panel, rect};

use super::model::*;

pub(super) fn spawn_platforms(commands: &mut Commands) {
    let platforms = [
        (Vec2::new(0.0, -220.0), Vec2::new(920.0, 34.0)),
        (Vec2::new(-230.0, -90.0), Vec2::new(190.0, 24.0)),
        (Vec2::new(60.0, 10.0), Vec2::new(210.0, 24.0)),
        (Vec2::new(305.0, 105.0), Vec2::new(210.0, 24.0)),
    ];
    for (pos, size) in platforms {
        spawn_wall(commands, pos, size, Color::srgb(0.42, 0.5, 0.56));
    }
}

pub(super) fn spawn_player(commands: &mut Commands, id: usize, pos: Vec2, color: Color) {
    rect(commands, pos, Vec2::new(32.0, 32.0), color, GameEntity).insert((
        Player { id },
        Collider {
            size: Vec2::new(30.0, 30.0),
        },
    ));
}

pub(super) fn spawn_runner(commands: &mut Commands, id: usize, pos: Vec2, color: Color) {
    rect(commands, pos, Vec2::new(30.0, 38.0), color, GameEntity).insert((
        Player { id },
        Collider {
            size: Vec2::new(30.0, 38.0),
        },
        Velocity(Vec2::ZERO),
        Grounded(false),
    ));
}

pub(super) fn spawn_enemy(commands: &mut Commands, pos: Vec2, hp: i32, points: u32, color: Color) {
    rect(commands, pos, Vec2::new(34.0, 34.0), color, GameEntity).insert((
        Enemy { hp, points },
        Collider {
            size: Vec2::new(32.0, 32.0),
        },
        Velocity(Vec2::ZERO),
    ));
}

pub(super) fn spawn_wall(commands: &mut Commands, pos: Vec2, size: Vec2, color: Color) {
    panel(
        commands,
        pos,
        size,
        color,
        Color::srgb(0.1, 0.11, 0.13),
        GameEntity,
    )
    .insert((Wall, Collider { size }));
}

pub(super) fn spawn_goal(commands: &mut Commands, pos: Vec2, size: Vec2, color: Color) {
    panel(
        commands,
        pos,
        size,
        color,
        Color::srgb(1.0, 0.96, 0.62),
        GameEntity,
    )
    .insert((Goal, Collider { size }));
}

pub(super) fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    rect(
        commands,
        pos,
        Vec2::splat(28.0),
        Color::srgb(0.08, 0.08, 0.09),
        GameEntity,
    )
    .insert((Bomb {
        timer: Timer::new(Duration::from_millis(1200), TimerMode::Once),
    },));
}

pub(super) fn spawn_projectile(
    commands: &mut Commands,
    pos: Vec2,
    velocity: Vec2,
    owner: ProjectileOwner,
    color: Color,
) {
    rect(commands, pos, Vec2::splat(12.0), color, GameEntity).insert((
        Projectile { owner, radius: 8.0 },
        Velocity(velocity),
        Lifetime(Timer::new(Duration::from_secs(3), TimerMode::Once)),
    ));
}

pub(super) fn spawn_bubble_projectile(
    commands: &mut Commands,
    pos: Vec2,
    velocity: Vec2,
    color_id: usize,
) {
    rect(
        commands,
        pos,
        Vec2::splat(28.0),
        bubble_color(color_id),
        GameEntity,
    )
    .insert((
        Projectile {
            owner: ProjectileOwner::Player,
            radius: 15.0,
        },
        Bubble { color_id },
        Velocity(velocity),
        Lifetime(Timer::new(Duration::from_secs(4), TimerMode::Once)),
    ));
}

pub(super) fn spawn_bubble(commands: &mut Commands, pos: Vec2, color_id: usize) {
    rect(
        commands,
        pos,
        Vec2::splat(30.0),
        bubble_color(color_id),
        GameEntity,
    )
    .insert((
        Bubble { color_id },
        Collider {
            size: Vec2::splat(30.0),
        },
    ));
}

fn bubble_color(color_id: usize) -> Color {
    match color_id % 4 {
        0 => Color::srgb(0.95, 0.34, 0.45),
        1 => Color::srgb(0.35, 0.67, 0.98),
        2 => Color::srgb(0.34, 0.85, 0.48),
        _ => Color::srgb(0.98, 0.82, 0.3),
    }
}
