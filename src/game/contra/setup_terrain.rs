use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;

// 背景：FC 1-1 纯黑夜空 + 稀疏星点
pub fn spawn_background(commands: &mut Commands) {
    let sky = commands
        .spawn((
            Sprite::from_color(COLOR_SKY, Vec2::new(ARENA_W * 1.4, ARENA_H * 1.4)),
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_BG)),
            ContraBackground,
            GameEntity,
        ))
        .id();
    let stars: [(f32, f32); 22] = [
        (-440.0, 200.0), (-380.0, 175.0), (-320.0, 215.0), (-260.0, 185.0),
        (-200.0, 210.0), (-140.0, 175.0), (-80.0, 220.0), (-20.0, 190.0),
        (40.0, 205.0), (100.0, 175.0), (160.0, 215.0), (220.0, 185.0),
        (280.0, 210.0), (340.0, 175.0), (400.0, 220.0), (440.0, 190.0),
        (-420.0, 145.0), (-180.0, 140.0), (60.0, 145.0), (260.0, 140.0),
        (380.0, 150.0), (-300.0, 150.0),
    ];
    for (x, y) in stars {
        commands
            .spawn((
                Sprite::from_color(COLOR_STAR, Vec2::splat(2.0)),
                Transform::from_translation(Vec3::new(x, y, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(sky));
    }
}

// 远景装饰：FC 1-1 白色雪山带
pub fn spawn_decor(commands: &mut Commands) {
    let base_y = GROUND_TOP + 32.0;
    let mountains: [(f32, f32, f32); 14] = [
        (160.0, 180.0, 96.0),
        (400.0, 150.0, 78.0),
        (660.0, 200.0, 110.0),
        (920.0, 160.0, 86.0),
        (1180.0, 220.0, 116.0),
        (1460.0, 170.0, 90.0),
        (1740.0, 210.0, 108.0),
        (2020.0, 180.0, 96.0),
        (2320.0, 220.0, 114.0),
        (2620.0, 160.0, 84.0),
        (2900.0, 210.0, 108.0),
        (3200.0, 190.0, 100.0),
        (3520.0, 220.0, 116.0),
        (3860.0, 180.0, 92.0),
    ];
    for (x, w, h) in mountains {
        spawn_mountain(commands, Vec2::new(x, base_y), w, h);
    }
}

fn spawn_mountain(commands: &mut Commands, base: Vec2, base_w: f32, peak_h: f32) {
    let step_h = 4.0_f32;
    let layers = (peak_h / step_h).max(1.0) as i32;
    let red_layers = 4;
    for i in 0..layers {
        let frac = i as f32 / layers as f32;
        let w = (base_w * (1.0 - frac)).max(2.0);
        let y = base.y + i as f32 * step_h + step_h * 0.5;
        let is_top = i >= layers - red_layers;
        let bright_color = if is_top { COLOR_MOUNTAIN_TOP } else { COLOR_MOUNTAIN };
        let shade_color = if is_top { COLOR_MOUNTAIN_TOP } else { COLOR_MOUNTAIN_SHADE };
        let bright_w = w * 0.55;
        commands.spawn((
            Sprite::from_color(bright_color, Vec2::new(bright_w, step_h + 0.5)),
            Transform::from_translation(Vec3::new(base.x - (w - bright_w) * 0.5, y, Z_BG2)),
            GameEntity,
        ));
        let shade_w = w * 0.45;
        commands.spawn((
            Sprite::from_color(shade_color, Vec2::new(shade_w, step_h + 0.5)),
            Transform::from_translation(Vec3::new(base.x + (w - shade_w) * 0.5, y, Z_BG2 + 0.01)),
            GameEntity,
        ));
    }
    commands.spawn((
        Sprite::from_color(COLOR_MOUNTAIN_DARK, Vec2::new(base_w + 2.0, 3.0)),
        Transform::from_translation(Vec3::new(base.x, base.y - 1.5, Z_BG2 + 0.02)),
        GameEntity,
    ));
}

fn spawn_palm_tree(commands: &mut Commands, base_x: f32, ground_y: f32, height: f32, mirror: bool) {
    let trunk_h = height;
    let trunk_w = 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(trunk_w, trunk_h)),
        Transform::from_translation(Vec3::new(base_x, ground_y + trunk_h * 0.5, Z_BG2 + 0.10)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(2.0, trunk_h)),
        Transform::from_translation(Vec3::new(base_x - 2.0, ground_y + trunk_h * 0.5, Z_BG2 + 0.11)),
        GameEntity,
    ));
    let crown_y = ground_y + trunk_h + 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_PALM_DARK, Vec2::new(28.0, 14.0)),
        Transform::from_translation(Vec3::new(base_x, crown_y, Z_BG2 + 0.12)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(6.0, 4.0)),
        Transform::from_translation(Vec3::new(base_x + 4.0, crown_y - 4.0, Z_BG2 + 0.13)),
        GameEntity,
    ));
    let sign = if mirror { -1.0 } else { 1.0 };
    let fronds: [(f32, f32, f32, f32, Color); 6] = [
        (-22.0 * sign, 6.0, 28.0, 5.0, COLOR_PALM_LIGHT),
        (22.0 * sign, 6.0, 28.0, 5.0, COLOR_PALM_DARK),
        (-26.0 * sign, -2.0, 32.0, 5.0, COLOR_PALM_LIGHT),
        (26.0 * sign, -2.0, 32.0, 5.0, COLOR_PALM_DARK),
        (-18.0 * sign, 12.0, 24.0, 5.0, COLOR_PALM_LIGHT),
        (18.0 * sign, 12.0, 24.0, 5.0, COLOR_PALM_DARK),
    ];
    for (dx, dy, w, h, color) in fronds {
        commands.spawn((
            Sprite::from_color(color, Vec2::new(w, h)),
            Transform::from_translation(Vec3::new(base_x + dx, crown_y + dy, Z_BG2 + 0.13)),
            GameEntity,
        ));
    }
    for (dx, dy) in [(-32.0 * sign, 6.0), (32.0 * sign, -2.0), (-26.0 * sign, 12.0)] {
        commands.spawn((
            Sprite::from_color(COLOR_PALM_LIGHT, Vec2::new(4.0, 3.0)),
            Transform::from_translation(Vec3::new(base_x + dx, crown_y + dy, Z_BG2 + 0.14)),
            GameEntity,
        ));
    }
}

fn spawn_canopy(commands: &mut Commands, x_start: f32, x_end: f32, top_y: f32) {
    let mut x = x_start + 38.0;
    let mut idx = 0i32;
    while x < x_end - 32.0 {
        let h = match idx % 3 {
            0 => 38.0,
            1 => 46.0,
            _ => 32.0,
        };
        let mirror = idx % 2 == 1;
        spawn_palm_tree(commands, x, top_y + 2.0, h, mirror);
        x += match idx % 3 {
            0 => 86.0,
            1 => 102.0,
            _ => 78.0,
        };
        idx += 1;
    }
}

pub fn spawn_ground_run(commands: &mut Commands, x_start: f32, x_end: f32, top_y: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;

    spawn_canopy(commands, x_start, x_end, top_y);
    spawn_grass(commands, x_start, x_end, top_y, w, cx);
    spawn_rock_wall(commands, x_start, x_end, top_y, w, cx);

    // 实体碰撞器（顶面对齐）
    let solid_h = 200.0;
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(w, solid_h)),
        Transform::from_translation(Vec3::new(cx, top_y - solid_h * 0.5, Z_TILE - 0.1)),
        ContraSolid {
            size: Vec2::new(w, solid_h),
        },
        GameEntity,
    ));
}

fn spawn_grass(commands: &mut Commands, x_start: f32, x_end: f32, grass_top: f32, w: f32, cx: f32) {
    let bright_h = 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_GRASS_BRIGHT, Vec2::new(w, bright_h)),
        Transform::from_translation(Vec3::new(cx, grass_top - bright_h * 0.5, Z_TILE + 0.16)),
        GameEntity,
    ));
    let dark_h = 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(w, dark_h)),
        Transform::from_translation(Vec3::new(cx, grass_top - bright_h - dark_h * 0.5, Z_TILE + 0.15)),
        GameEntity,
    ));
    // 顶端草尖
    let n = (w / 12.0) as i32;
    for i in 0..n {
        let x = x_start + i as f32 * 12.0 + 6.0;
        let h = if i % 2 == 0 { 3.0 } else { 2.0 };
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_BRIGHT, Vec2::new(4.0, h)),
            Transform::from_translation(Vec3::new(x, grass_top + h * 0.5, Z_TILE + 0.17)),
            GameEntity,
        ));
    }
    // 下沿草丛
    let tuft_count = (w / 28.0) as i32;
    for i in 0..tuft_count {
        let tx = x_start + 14.0 + i as f32 * 28.0;
        if tx > x_end - 10.0 {
            break;
        }
        let tuft_y = grass_top - bright_h - dark_h - 2.0;
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(8.0, 6.0)),
            Transform::from_translation(Vec3::new(tx, tuft_y, Z_TILE + 0.18)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(3.0, 4.0)),
            Transform::from_translation(Vec3::new(tx, tuft_y - 4.0, Z_TILE + 0.18)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(2.0, 3.0)),
            Transform::from_translation(Vec3::new(tx - 5.0, tuft_y - 2.0, Z_TILE + 0.18)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(2.0, 3.0)),
            Transform::from_translation(Vec3::new(tx + 5.0, tuft_y - 2.0, Z_TILE + 0.18)),
            GameEntity,
        ));
    }
}

fn spawn_rock_wall(commands: &mut Commands, x_start: f32, x_end: f32, grass_top: f32, w: f32, cx: f32) {
    let rock_top = grass_top - 12.0;
    let rock_h_total = 280.0;
    let rock_bottom = rock_top - rock_h_total;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(w, rock_h_total)),
        Transform::from_translation(Vec3::new(cx, (rock_top + rock_bottom) * 0.5, Z_TILE)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(w, 8.0)),
        Transform::from_translation(Vec3::new(cx, rock_top - 4.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(w, 12.0)),
        Transform::from_translation(Vec3::new(cx, rock_bottom + 6.0, Z_TILE + 0.03)),
        GameEntity,
    ));

    let block_w = 56.0;
    let block_h = 32.0;
    let rows = (rock_h_total / block_h).ceil() as i32;
    for row in 0..rows {
        let row_offset = if row % 2 == 0 { 0.0 } else { block_w * 0.5 };
        let by = rock_top - block_h * 0.5 - row as f32 * block_h;
        if by < rock_bottom + block_h * 0.5 - 4.0 {
            break;
        }
        let cols = ((w + block_w) / block_w).ceil() as i32;
        for col in 0..cols {
            let bx = x_start + row_offset + col as f32 * block_w;
            if bx < x_start - block_w * 0.4 || bx > x_end + block_w * 0.4 {
                continue;
            }
            spawn_rock_block(commands, Vec2::new(bx, by), block_w, block_h, x_start, x_end);
        }
    }
}

fn spawn_rock_block(
    commands: &mut Commands,
    center: Vec2,
    w: f32,
    h: f32,
    bound_x_start: f32,
    bound_x_end: f32,
) {
    let half_w = w * 0.5;
    let left = (center.x - half_w).max(bound_x_start);
    let right = (center.x + half_w).min(bound_x_end);
    let real_w = right - left;
    if real_w < 8.0 {
        return;
    }
    let cx = (left + right) * 0.5;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(real_w, h)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.02)),
        GameEntity,
    ));
    let inner_w = real_w - 4.0;
    let inner_h = h - 2.0;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(inner_w, inner_h)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.03)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(inner_w - 4.0, inner_h + 2.0)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.031)),
        GameEntity,
    ));
    let mid_h = (h * 0.55).max(8.0);
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(inner_w - 2.0, mid_h)),
        Transform::from_translation(Vec3::new(cx, center.y - 1.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    let hi_w = (inner_w * 0.65).max(6.0);
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(hi_w, 3.0)),
        Transform::from_translation(Vec3::new(cx - 2.0, center.y + h * 0.5 - 5.0, Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(3.0, 2.0)),
        Transform::from_translation(Vec3::new(
            cx - inner_w * 0.25,
            center.y + h * 0.25,
            Z_TILE + 0.06,
        )),
        GameEntity,
    ));
}

pub fn spawn_water(commands: &mut Commands, x_start: f32, x_end: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;
    let top_y = GROUND_TOP - 16.0;
    let h = 220.0;
    let cy = top_y - h * 0.5;
    commands.spawn((
        Sprite::from_color(COLOR_WATER, Vec2::new(w, h)),
        Transform::from_translation(Vec3::new(cx, cy, Z_TILE)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_WATER_HI, Vec2::new(w, 6.0)),
        Transform::from_translation(Vec3::new(cx, top_y - 3.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    for row in 0..3 {
        let row_y = top_y - 14.0 - row as f32 * 18.0;
        let phase_off = (row as f32) * 12.0;
        let n = (w / 24.0) as i32;
        for i in 0..n {
            let x = x_start + 8.0 + i as f32 * 24.0 + phase_off;
            if x > x_end - 6.0 {
                break;
            }
            commands.spawn((
                Sprite::from_color(COLOR_WATER_HI, Vec2::new(10.0, 2.0)),
                Transform::from_translation(Vec3::new(x, row_y, Z_TILE + 0.05)),
                GameEntity,
            ));
        }
    }
    let n = (w / 10.0) as i32;
    for i in 0..n {
        let x = x_start + 5.0 + i as f32 * 10.0;
        commands.spawn((
            Sprite::from_color(COLOR_FOAM, Vec2::new(8.0, 2.0)),
            Transform::from_translation(Vec3::new(x, top_y + 1.0, Z_TILE + 0.06)),
            GameEntity,
        ));
    }
}

pub fn spawn_bridge(commands: &mut Commands, x_start: f32, x_end: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;
    spawn_water(commands, x_start, x_end);
    for side_x in [x_start + 4.0, x_end - 4.0] {
        commands.spawn((
            Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(6.0, 32.0)),
            Transform::from_translation(Vec3::new(side_x, GROUND_TOP + 16.0, Z_TILE + 0.10)),
            GameEntity,
        ));
    }
    let plank_top = GROUND_TOP - 1.0;
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_ROPE, Vec2::new(w, 2.0)),
        Transform::from_translation(Vec3::new(cx, GROUND_TOP + 28.0, Z_TILE + 0.11)),
        GameEntity,
    ));
    let plank_h = 12.0;
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_PLANK_DK, Vec2::new(w, plank_h)),
        Transform::from_translation(Vec3::new(cx, plank_top - plank_h * 0.5, Z_TILE + 0.13)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_PLANK, Vec2::new(w, 5.0)),
        Transform::from_translation(Vec3::new(cx, plank_top - 2.5, Z_TILE + 0.14)),
        GameEntity,
    ));
    let n = (w / 16.0) as i32;
    for i in 1..n {
        let x = x_start + i as f32 * 16.0;
        commands.spawn((
            Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(1.0, plank_h)),
            Transform::from_translation(Vec3::new(x, plank_top - plank_h * 0.5, Z_TILE + 0.15)),
            GameEntity,
        ));
    }
    for i in 0..((w / 32.0) as i32) {
        let x = x_start + 16.0 + i as f32 * 32.0;
        if x >= x_end - 6.0 {
            break;
        }
        commands.spawn((
            Sprite::from_color(COLOR_BRIDGE_ROPE, Vec2::new(1.0, 28.0)),
            Transform::from_translation(Vec3::new(x, GROUND_TOP + 14.0, Z_TILE + 0.12)),
            GameEntity,
        ));
    }
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(w, plank_h)),
        Transform::from_translation(Vec3::new(cx, plank_top - plank_h * 0.5, Z_TILE - 0.1)),
        ContraSolid {
            size: Vec2::new(w, plank_h),
        },
        GameEntity,
    ));
}

pub fn spawn_platform(commands: &mut Commands, center: Vec2, width: f32) {
    let h = 16.0;
    commands.spawn((
        Sprite::from_color(COLOR_PLATFORM, Vec2::new(width, h)),
        Transform::from_translation(Vec3::new(center.x, center.y - h * 0.5, Z_PLATFORM)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_PLATFORM_TOP, Vec2::new(width, 4.0)),
        Transform::from_translation(Vec3::new(center.x, center.y - 2.0, Z_PLATFORM + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(width, h)),
        Transform::from_translation(Vec3::new(center.x, center.y - h * 0.5, Z_PLATFORM - 0.1)),
        ContraSolid {
            size: Vec2::new(width, h),
        },
        GameEntity,
    ));
}
