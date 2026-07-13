use bevy::prelude::*;

use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;

pub fn spawn_background_decor(commands: &mut Commands) {
    let cloud_positions: [(f32, f32); 5] = [
        (180.0, 180.0),
        (820.0, 220.0),
        (1500.0, 170.0),
        (2400.0, 200.0),
        (3500.0, 190.0),
    ];
    for (x, y) in cloud_positions {
        spawn_cloud(commands, Vec2::new(x, y));
    }
    let hill_positions: [(f32, bool); 4] = [
        (520.0, true),
        (1900.0, false),
        (2700.0, true),
        (4100.0, false),
    ];
    for (x, big) in hill_positions {
        spawn_hill(commands, x, big);
    }
    let bush_positions: [(f32, i32); 5] = [
        (300.0, 1),
        (1200.0, 2),
        (2200.0, 1),
        (3100.0, 3),
        (3900.0, 2),
    ];
    for (x, w) in bush_positions {
        spawn_bush(commands, x, w);
    }
}

pub fn spawn_cloud(commands: &mut Commands, center: Vec2) {
    let parts: [(f32, f32, f32, f32); 5] = [
        (-26.0, 0.0, 22.0, 16.0),
        (0.0, 0.0, 30.0, 20.0),
        (26.0, 0.0, 22.0, 16.0),
        (-12.0, 12.0, 18.0, 12.0),
        (12.0, 12.0, 18.0, 12.0),
    ];
    for (dx, dy, w, h) in parts {
        commands.spawn((
            Sprite::from_color(COLOR_CLOUD, Vec2::new(w, h)),
            Transform::from_translation((center + Vec2::new(dx, dy)).extend(Z_BG_MID)),
            GameEntity,
        ));
    }
}

pub fn spawn_hill(commands: &mut Commands, x_center: f32, big: bool) {
    let base_y = FLOOR_Y + TILE * 0.5;
    let scale = if big { 1.4 } else { 1.0 };
    let layers = if big { 6 } else { 4 };
    for i in 0..layers {
        let w = ((layers - i) as f32) * 28.0 * scale;
        let y = base_y + (i as f32) * 18.0 * scale;
        let color = if i == layers - 1 {
            COLOR_HILL_DARK
        } else {
            COLOR_HILL
        };
        commands.spawn((
            Sprite::from_color(color, Vec2::new(w, 18.0 * scale + 1.0)),
            Transform::from_translation(Vec3::new(x_center, y, Z_BG_MID + 0.2)),
            GameEntity,
        ));
    }
    let tip_y = base_y + (layers as f32) * 18.0 * scale - 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_CLOUD, Vec2::new(8.0, 6.0)),
        Transform::from_translation(Vec3::new(x_center - 6.0, tip_y, Z_BG_MID + 0.3)),
        GameEntity,
    ));
}

pub fn spawn_bush(commands: &mut Commands, x_center: f32, segments: i32) {
    let base_y = FLOOR_Y + TILE * 0.5 + 6.0;
    let total_w = (segments as f32) * 32.0;
    commands.spawn((
        Sprite::from_color(COLOR_BUSH, Vec2::new(total_w, 18.0)),
        Transform::from_translation(Vec3::new(x_center, base_y, Z_BG_MID + 0.4)),
        GameEntity,
    ));
    for i in 0..segments {
        let dx = -total_w * 0.5 + 16.0 + (i as f32) * 32.0;
        commands.spawn((
            Sprite::from_color(COLOR_BUSH, Vec2::new(22.0, 14.0)),
            Transform::from_translation(Vec3::new(x_center + dx, base_y + 14.0, Z_BG_MID + 0.4)),
            GameEntity,
        ));
    }
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_LIGHT, Vec2::new(total_w * 0.7, 3.0)),
        Transform::from_translation(Vec3::new(x_center - 4.0, base_y + 5.0, Z_BG_MID + 0.5)),
        GameEntity,
    ));
}

pub fn spawn_ground(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_TOP, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        Solid { size: Vec2::splat(TILE) },
        GroundTile,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_LIGHT, Vec2::new(TILE, 4.0)),
        Transform::from_translation((center + Vec2::new(0.0, TILE * 0.5 - 2.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_DARK, Vec2::new(6.0, 6.0)),
        Transform::from_translation((center + Vec2::new(-8.0, -2.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_DARK, Vec2::new(6.0, 6.0)),
        Transform::from_translation((center + Vec2::new(8.0, -10.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

pub fn spawn_brick(commands: &mut Commands, center: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(COLOR_BRICK, Vec2::splat(TILE)),
            Transform::from_translation(center.extend(Z_TILE)),
            Solid { size: Vec2::splat(TILE) },
            BrickTile,
            GameEntity,
        ))
        .id();
    for i in 0..2 {
        let y_off = -TILE * 0.5 + 8.0 + (i as f32) * 16.0;
        commands
            .spawn((
                Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(TILE, 2.0)),
                Transform::from_translation(Vec3::new(0.0, y_off, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
    commands
        .spawn((
            Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(2.0, 14.0)),
            Transform::from_translation(Vec3::new(-9.0, 9.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    commands
        .spawn((
            Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(2.0, 14.0)),
            Transform::from_translation(Vec3::new(9.0, -9.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
}

pub fn spawn_question(commands: &mut Commands, center: Vec2, content: QuestionContent) {
    let entity = commands
        .spawn((
            Sprite::from_color(COLOR_QUESTION, Vec2::splat(TILE)),
            Transform::from_translation(center.extend(Z_TILE)),
            Solid { size: Vec2::splat(TILE) },
            QuestionBlock {
                used: false,
                bump_t: 0.0,
                base_y: center.y,
                content,
                spawn_done: false,
            },
            GameEntity,
        ))
        .id();
    commands
        .spawn((
            Sprite::from_color(COLOR_QUESTION_DARK, Vec2::new(TILE, 4.0)),
            Transform::from_translation(Vec3::new(0.0, TILE * 0.5 - 2.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    commands
        .spawn((
            Sprite::from_color(COLOR_QUESTION_DARK, Vec2::new(TILE, 4.0)),
            Transform::from_translation(Vec3::new(0.0, -TILE * 0.5 + 2.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    let q_parts: [(f32, f32, f32, f32); 6] = [
        (0.0, 8.0, 10.0, 4.0),
        (5.0, 4.0, 4.0, 4.0),
        (0.0, 0.0, 4.0, 4.0),
        (0.0, -4.0, 4.0, 4.0),
        (-5.0, 8.0, 4.0, 4.0),
        (0.0, -10.0, 4.0, 4.0),
    ];
    for (dx, dy, w, h) in q_parts {
        commands
            .spawn((
                Sprite::from_color(COLOR_M_BLACK, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.06)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

pub fn spawn_stone(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_TOP, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        Solid { size: Vec2::splat(TILE) },
        StoneTile,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_LIGHT, Vec2::new(TILE - 4.0, 3.0)),
        Transform::from_translation((center + Vec2::new(0.0, TILE * 0.5 - 3.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_DARK, Vec2::new(TILE - 4.0, 3.0)),
        Transform::from_translation((center + Vec2::new(0.0, -TILE * 0.5 + 3.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

pub fn spawn_pipe_tile(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_PIPE, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_PIPE)),
        Solid { size: Vec2::splat(TILE) },
        PipeTile,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_LIGHT, Vec2::new(4.0, TILE - 4.0)),
        Transform::from_translation((center + Vec2::new(-TILE * 0.5 + 6.0, 0.0)).extend(Z_PIPE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_DARK, Vec2::new(4.0, TILE - 4.0)),
        Transform::from_translation((center + Vec2::new(TILE * 0.5 - 6.0, 0.0)).extend(Z_PIPE + 0.05)),
        GameEntity,
    ));
}

pub fn spawn_castle(commands: &mut Commands, base_center: Vec2) {
    let cx = base_center.x;
    let cy = base_center.y + 32.0;
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE, Vec2::new(112.0, 96.0)),
        Transform::from_translation(Vec3::new(cx, cy, Z_DECOR)),
        GameEntity,
    ));
    for i in 0..5 {
        let dx = -48.0 + (i as f32) * 24.0;
        commands.spawn((
            Sprite::from_color(COLOR_CASTLE, Vec2::new(16.0, 12.0)),
            Transform::from_translation(Vec3::new(cx + dx, cy + 54.0, Z_DECOR)),
            GameEntity,
        ));
    }
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE, Vec2::new(36.0, 32.0)),
        Transform::from_translation(Vec3::new(cx, cy + 64.0, Z_DECOR)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE, Vec2::new(20.0, 16.0)),
        Transform::from_translation(Vec3::new(cx, cy + 86.0, Z_DECOR)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE_DARK, Vec2::new(24.0, 36.0)),
        Transform::from_translation(Vec3::new(cx, cy - 30.0, Z_DECOR + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE_DARK, Vec2::new(24.0, 6.0)),
        Transform::from_translation(Vec3::new(cx, cy - 8.0, Z_DECOR + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE_DARK, Vec2::new(8.0, 10.0)),
        Transform::from_translation(Vec3::new(cx - 32.0, cy + 12.0, Z_DECOR + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE_DARK, Vec2::new(8.0, 10.0)),
        Transform::from_translation(Vec3::new(cx + 32.0, cy + 12.0, Z_DECOR + 0.05)),
        GameEntity,
    ));
}

pub fn spawn_flag(commands: &mut Commands, col: i32) {
    let pole_x = col as f32 * TILE + TILE * 0.5;
    let pole_bottom_y = FLOOR_Y + TILE;
    let pole_top_y = FLOOR_Y + 10.0 * TILE;
    let pole_h = pole_top_y - pole_bottom_y;
    let pole_cy = (pole_top_y + pole_bottom_y) * 0.5;
    commands.spawn((
        Sprite::from_color(COLOR_FLAG_POLE, Vec2::new(4.0, pole_h)),
        Transform::from_translation(Vec3::new(pole_x, pole_cy, Z_FLAG)),
        FlagPole,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_FLAG, Vec2::new(12.0, 12.0)),
        Transform::from_translation(Vec3::new(pole_x, pole_top_y + 6.0, Z_FLAG + 0.05)),
        GameEntity,
    ));
    let flag_y = pole_top_y - 12.0;
    commands.spawn((
        Sprite::from_color(COLOR_FLAG, Vec2::new(22.0, 16.0)),
        Transform::from_translation(Vec3::new(pole_x - 13.0, flag_y, Z_FLAG + 0.1)),
        FlagBanner {
            y_target: flag_y,
            speed: 0.0,
        },
        GameEntity,
    ));
}

pub fn spawn_dark_block(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_DARK_BRICK, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        Solid { size: Vec2::splat(TILE) },
        DarkBrick,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_DARK_BRICK_LIGHT, Vec2::new(TILE - 4.0, 3.0)),
        Transform::from_translation((center + Vec2::new(0.0, TILE * 0.5 - 3.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_DARK_BRICK_DARK, Vec2::new(TILE - 4.0, 3.0)),
        Transform::from_translation((center + Vec2::new(0.0, -TILE * 0.5 + 3.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

pub fn spawn_lava(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_LAVA, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        LavaTile,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_LAVA_BRIGHT, Vec2::new(TILE, 4.0)),
        Transform::from_translation((center + Vec2::new(0.0, TILE * 0.4)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_LAVA_BRIGHT, Vec2::new(TILE * 0.4, 3.0)),
        Transform::from_translation((center + Vec2::new(-6.0, TILE * 0.2)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    let _ = LAVA_DAMAGE;
}

pub fn spawn_moving_platform(commands: &mut Commands, center: Vec2, vertical: bool) {
    let size = Vec2::new(PLATFORM_W, PLATFORM_H);
    let initial_vel = if vertical {
        Vec2::new(0.0, PLATFORM_SPEED)
    } else {
        Vec2::new(PLATFORM_SPEED, 0.0)
    };
    let entity = commands
        .spawn((
            Sprite::from_color(COLOR_PLATFORM, size),
            Transform::from_translation(center.extend(Z_TILE)),
            Solid { size },
            MovingPlatform {
                vertical,
                center,
                vel: initial_vel,
                last_dx: 0.0,
                last_dy: 0.0,
            },
            GameEntity,
        ))
        .id();
    commands
        .spawn((
            Sprite::from_color(COLOR_PLATFORM_DARK, Vec2::new(PLATFORM_W, 2.0)),
            Transform::from_translation(Vec3::new(0.0, -PLATFORM_H * 0.5 + 1.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    for i in 0..3 {
        commands
            .spawn((
                Sprite::from_color(COLOR_PLATFORM_DARK, Vec2::new(2.0, 4.0)),
                Transform::from_translation(Vec3::new(
                    -20.0 + (i as f32) * 20.0,
                    PLATFORM_H * 0.5 + 8.0,
                    0.05,
                )),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

pub fn spawn_high_sky_decor(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(COLOR_MOON, Vec2::splat(40.0)),
        Transform::from_translation(Vec3::new(160.0, 200.0, Z_BG_MID)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.6, 0.4), Vec2::splat(8.0)),
        Transform::from_translation(Vec3::new(166.0, 206.0, Z_BG_MID + 0.05)),
        GameEntity,
    ));
    let cloud_positions = [(420.0, 180.0), (1000.0, 220.0), (1700.0, 170.0)];
    for (x, y) in cloud_positions {
        spawn_cloud(commands, Vec2::new(x, y));
    }
}
