use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;

// 背景：夜空三段渐变 + 双密度星点（跟随相机）
pub fn spawn_background(commands: &mut Commands, sky_colors: [Color; 3]) {
    let sky_w = ARENA_W * 1.4;
    let sky_h = ARENA_H * 1.4;
    let sky = commands
        .spawn((
            Sprite::from_color(sky_colors[0], Vec2::new(sky_w, sky_h)),
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_BG)),
            ContraBackground,
            GameEntity,
        ))
        .id();
    // 渐变带：从底部最深到顶端最亮，分 6 段叠在天幕上方
    let band_h = sky_h * 0.18;
    let bands: [(f32, Color); 6] = [
        (sky_h * 0.50, sky_colors[2]),
        (sky_h * 0.36, sky_colors[2]),
        (sky_h * 0.22, sky_colors[1]),
        (sky_h * 0.08, sky_colors[1]),
        (-sky_h * 0.06, sky_colors[0]),
        (-sky_h * 0.20, sky_colors[0]),
    ];
    for (y, color) in bands {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(sky_w, band_h)),
                Transform::from_translation(Vec3::new(0.0, y, 0.02)),
                GameEntity,
            ))
            .insert(ChildOf(sky));
    }

    // 大颗星点
    let bright_stars: [(f32, f32); 26] = [
        (-440.0, 220.0), (-400.0, 178.0), (-360.0, 232.0), (-300.0, 196.0),
        (-260.0, 244.0), (-220.0, 182.0), (-160.0, 224.0), (-100.0, 254.0),
        (-40.0, 192.0), (20.0, 232.0), (80.0, 198.0), (140.0, 246.0),
        (200.0, 184.0), (260.0, 224.0), (320.0, 196.0), (380.0, 244.0),
        (440.0, 188.0), (-420.0, 152.0), (-180.0, 156.0), (60.0, 142.0),
        (260.0, 158.0), (380.0, 168.0), (-300.0, 168.0), (180.0, 168.0),
        (-100.0, 172.0), (340.0, 210.0),
    ];
    for (x, y) in bright_stars {
        commands
            .spawn((
                Sprite::from_color(COLOR_STAR, Vec2::splat(2.0)),
                Transform::from_translation(Vec3::new(x, y, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(sky));
    }
    // 小颗暗星
    let dim_stars: [(f32, f32); 22] = [
        (-460.0, 186.0), (-340.0, 154.0), (-200.0, 250.0), (-60.0, 218.0),
        (40.0, 256.0), (160.0, 188.0), (280.0, 154.0), (420.0, 222.0),
        (-380.0, 200.0), (-240.0, 216.0), (-120.0, 144.0), (100.0, 174.0),
        (220.0, 252.0), (360.0, 184.0), (-280.0, 240.0), (-160.0, 186.0),
        (-20.0, 162.0), (120.0, 220.0), (240.0, 188.0), (340.0, 156.0),
        (-100.0, 244.0), (200.0, 144.0),
    ];
    for (x, y) in dim_stars {
        commands
            .spawn((
                Sprite::from_color(COLOR_STAR_DIM, Vec2::splat(1.0)),
                Transform::from_translation(Vec3::new(x, y, 0.04)),
                GameEntity,
            ))
            .insert(ChildOf(sky));
    }
}

// 远景装饰：远山阴影层 + 雪山主层 + 丛林剪影层
pub fn spawn_decor(commands: &mut Commands) {
    let base_y = GROUND_TOP + 32.0;

    // 第一层：远处大面积低矮蓝灰山影（衬纵深感）
    let far_mountains: [(f32, f32, f32); 10] = [
        (60.0, 360.0, 80.0),
        (520.0, 320.0, 70.0),
        (1000.0, 380.0, 86.0),
        (1480.0, 340.0, 74.0),
        (1960.0, 360.0, 82.0),
        (2440.0, 380.0, 88.0),
        (2920.0, 340.0, 72.0),
        (3400.0, 360.0, 82.0),
        (3820.0, 340.0, 76.0),
        (4220.0, 360.0, 82.0),
    ];
    for (x, w, h) in far_mountains {
        spawn_far_mountain(commands, Vec2::new(x, base_y + 12.0), w, h);
    }

    // 第二层：白雪红顶大山
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

    // 第三层：山脚到地面之间的深色丛林剪影（用一连串重叠树冠拼成）
    spawn_jungle_silhouette(commands, -200.0, WORLD_W + 200.0, base_y - 4.0);
}

// 远处大山阴影：单色钝三角，台阶更粗
fn spawn_far_mountain(commands: &mut Commands, base: Vec2, base_w: f32, peak_h: f32) {
    let step_h = 6.0_f32;
    let layers = (peak_h / step_h).max(1.0) as i32;
    for i in 0..layers {
        let frac = i as f32 / layers as f32;
        let w = (base_w * (1.0 - frac)).max(4.0);
        let y = base.y + i as f32 * step_h + step_h * 0.5;
        commands.spawn((
            Sprite::from_color(COLOR_MOUNTAIN_FAR, Vec2::new(w, step_h + 0.5)),
            Transform::from_translation(Vec3::new(base.x, y, Z_BG + 0.5)),
            GameEntity,
        ));
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

// 丛林剪影：在地面上方铺一排错落的圆顶 + 一条深色基带
fn spawn_jungle_silhouette(commands: &mut Commands, x_start: f32, x_end: f32, top_y: f32) {
    let strip_w = x_end - x_start;
    let strip_cx = (x_start + x_end) * 0.5;
    commands.spawn((
        Sprite::from_color(COLOR_JUNGLE_FAR, Vec2::new(strip_w, 24.0)),
        Transform::from_translation(Vec3::new(strip_cx, top_y - 12.0, Z_BG2 + 0.20)),
        GameEntity,
    ));
    let mut x = x_start;
    let mut idx: i32 = 0;
    while x < x_end {
        let (w, h, step) = match idx.rem_euclid(5) {
            0 => (40.0_f32, 18.0_f32, 28.0_f32),
            1 => (52.0, 24.0, 36.0),
            2 => (36.0, 14.0, 24.0),
            3 => (60.0, 28.0, 40.0),
            _ => (44.0, 20.0, 30.0),
        };
        let cy = top_y - 4.0 + h * 0.5;
        // 主体圆顶用三段宽度叠出弧线感
        commands.spawn((
            Sprite::from_color(COLOR_JUNGLE_FAR, Vec2::new(w, h)),
            Transform::from_translation(Vec3::new(x, cy, Z_BG2 + 0.21)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_JUNGLE_FAR, Vec2::new(w * 0.72, h + 4.0)),
            Transform::from_translation(Vec3::new(x, cy + 2.0, Z_BG2 + 0.22)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_JUNGLE_FAR, Vec2::new(w * 0.42, h + 8.0)),
            Transform::from_translation(Vec3::new(x, cy + 4.0, Z_BG2 + 0.23)),
            GameEntity,
        ));
        x += step;
        idx += 1;
    }
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
    // 三层草：亮 / 中 / 暗，形成层次而不是平涂
    let bright_h = 4.0;
    commands.spawn((
        Sprite::from_color(COLOR_GRASS_BRIGHT, Vec2::new(w, bright_h)),
        Transform::from_translation(Vec3::new(cx, grass_top - bright_h * 0.5, Z_TILE + 0.16)),
        GameEntity,
    ));
    let mid_h = 4.0;
    commands.spawn((
        Sprite::from_color(COLOR_GRASS_MID, Vec2::new(w, mid_h)),
        Transform::from_translation(Vec3::new(cx, grass_top - bright_h - mid_h * 0.5, Z_TILE + 0.155)),
        GameEntity,
    ));
    let dark_h = 5.0;
    commands.spawn((
        Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(w, dark_h)),
        Transform::from_translation(Vec3::new(cx, grass_top - bright_h - mid_h - dark_h * 0.5, Z_TILE + 0.15)),
        GameEntity,
    ));
    // 顶端密集草尖（间距加密 + 高度抖动）
    let n = (w / 6.0) as i32;
    for i in 0..n {
        let x = x_start + i as f32 * 6.0 + 3.0;
        let h = match i % 4 {
            0 => 4.0,
            1 => 2.0,
            2 => 3.0,
            _ => 2.0,
        };
        let color = if i % 3 == 0 { COLOR_GRASS_MID } else { COLOR_GRASS_BRIGHT };
        commands.spawn((
            Sprite::from_color(color, Vec2::new(2.0, h)),
            Transform::from_translation(Vec3::new(x, grass_top + h * 0.5, Z_TILE + 0.17)),
            GameEntity,
        ));
    }
    // 下沿垂悬草丛（更小更密，模仿草随悬崖下垂的感觉）
    let tuft_count = (w / 18.0) as i32;
    for i in 0..tuft_count {
        let tx = x_start + 10.0 + i as f32 * 18.0;
        if tx > x_end - 8.0 {
            break;
        }
        let tuft_y = grass_top - bright_h - mid_h - dark_h - 2.0;
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(8.0, 4.0)),
            Transform::from_translation(Vec3::new(tx, tuft_y, Z_TILE + 0.18)),
            GameEntity,
        ));
        let stem_h = if i % 2 == 0 { 5.0 } else { 3.0 };
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(2.0, stem_h)),
            Transform::from_translation(Vec3::new(tx, tuft_y - stem_h * 0.5 - 1.0, Z_TILE + 0.18)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(2.0, stem_h * 0.6)),
            Transform::from_translation(Vec3::new(tx - 4.0, tuft_y - stem_h * 0.3, Z_TILE + 0.18)),
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(2.0, stem_h * 0.6)),
            Transform::from_translation(Vec3::new(tx + 4.0, tuft_y - stem_h * 0.3, Z_TILE + 0.18)),
            GameEntity,
        ));
    }
}

fn spawn_rock_wall(commands: &mut Commands, x_start: f32, x_end: f32, grass_top: f32, w: f32, cx: f32) {
    let rock_top = grass_top - 12.0;
    let rock_h_total = 280.0;
    let rock_bottom = rock_top - rock_h_total;
    // 底色：暗棕，向下渐变
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(w, rock_h_total)),
        Transform::from_translation(Vec3::new(cx, (rock_top + rock_bottom) * 0.5, Z_TILE)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(w, rock_h_total * 0.55)),
        Transform::from_translation(Vec3::new(
            cx,
            rock_top - rock_h_total * 0.275,
            Z_TILE + 0.005,
        )),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(w, 6.0)),
        Transform::from_translation(Vec3::new(cx, rock_top - 3.0, Z_TILE + 0.04)),
        GameEntity,
    ));

    // 大小错落的天然石块：每行块宽小幅抖动，相邻行 1/3 偏移
    let base_w = 56.0;
    let base_h = 32.0;
    let rows = (rock_h_total / base_h).ceil() as i32;
    for row in 0..rows {
        let by = rock_top - base_h * 0.5 - row as f32 * base_h;
        if by < rock_bottom + base_h * 0.5 - 4.0 {
            break;
        }
        let row_offset = match row.rem_euclid(3) {
            0 => 0.0,
            1 => base_w * 0.4,
            _ => base_w * 0.7,
        };
        let cols = ((w + base_w * 2.0) / base_w).ceil() as i32;
        for col in 0..cols {
            // 同一行内每块宽度略微不同，避免规整感
            let jitter = match (row * 7 + col * 3).rem_euclid(5) {
                0 => -8.0,
                1 => 4.0,
                2 => 10.0,
                3 => -4.0,
                _ => 0.0,
            };
            let block_w = (base_w + jitter).clamp(40.0, 72.0);
            let bx = x_start + row_offset + col as f32 * base_w;
            if bx < x_start - block_w || bx > x_end + block_w {
                continue;
            }
            spawn_rock_block(commands, Vec2::new(bx, by), block_w, base_h, x_start, x_end, row, col);
        }
    }

    // 底部加一条更深的阴影边
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_SEAM, Vec2::new(w, 4.0)),
        Transform::from_translation(Vec3::new(cx, rock_bottom + 2.0, Z_TILE + 0.05)),
        GameEntity,
    ));
}

fn spawn_rock_block(
    commands: &mut Commands,
    center: Vec2,
    w: f32,
    h: f32,
    bound_x_start: f32,
    bound_x_end: f32,
    row: i32,
    col: i32,
) {
    let half_w = w * 0.5;
    let left = (center.x - half_w).max(bound_x_start);
    let right = (center.x + half_w).min(bound_x_end);
    let real_w = right - left;
    if real_w < 8.0 {
        return;
    }
    let cx = (left + right) * 0.5;

    // 描边：用细线模拟石缝，而不是整块黑底
    let inner_w = real_w - 2.0;
    let inner_h = h - 2.0;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_SEAM, Vec2::new(real_w, 1.0)),
        Transform::from_translation(Vec3::new(cx, center.y - h * 0.5 + 0.5, Z_TILE + 0.02)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_SEAM, Vec2::new(1.0, h)),
        Transform::from_translation(Vec3::new(cx - real_w * 0.5 + 0.5, center.y, Z_TILE + 0.02)),
        GameEntity,
    ));

    // 主体：中色
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(inner_w, inner_h)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.03)),
        GameEntity,
    ));
    // 暗色阴影（右下角）
    let shade_w = inner_w * 0.45;
    let shade_h = inner_h * 0.6;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(shade_w, shade_h)),
        Transform::from_translation(Vec3::new(
            cx + (inner_w - shade_w) * 0.5,
            center.y - (inner_h - shade_h) * 0.5,
            Z_TILE + 0.035,
        )),
        GameEntity,
    ));
    // 顶部高光
    let hi_w = inner_w * 0.7;
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(hi_w, 2.0)),
        Transform::from_translation(Vec3::new(
            cx - inner_w * 0.1,
            center.y + h * 0.5 - 2.5,
            Z_TILE + 0.05,
        )),
        GameEntity,
    ));
    // 内部小高光（每隔几块亮一个，避免每块都一样）
    if (row * 5 + col * 11).rem_euclid(3) == 0 {
        commands.spawn((
            Sprite::from_color(COLOR_ROCK_HI, Vec2::new(3.0, 2.0)),
            Transform::from_translation(Vec3::new(
                cx - inner_w * 0.25,
                center.y + h * 0.15,
                Z_TILE + 0.055,
            )),
            GameEntity,
        ));
    }
    // 小裂纹细节
    if (row * 3 + col * 7).rem_euclid(4) == 1 {
        commands.spawn((
            Sprite::from_color(COLOR_ROCK_SEAM, Vec2::new(1.0, h * 0.35)),
            Transform::from_translation(Vec3::new(
                cx + inner_w * 0.15,
                center.y - h * 0.05,
                Z_TILE + 0.045,
            )),
            GameEntity,
        ));
    }
}

pub fn spawn_water(commands: &mut Commands, x_start: f32, x_end: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;
    let top_y = GROUND_TOP - 16.0;
    let h = 220.0;
    let cy = top_y - h * 0.5;
    // 双层水底：上半亮、下半暗，做出深度
    commands.spawn((
        Sprite::from_color(COLOR_WATER_DEEP, Vec2::new(w, h)),
        Transform::from_translation(Vec3::new(cx, cy, Z_TILE)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_WATER, Vec2::new(w, h * 0.55)),
        Transform::from_translation(Vec3::new(cx, top_y - h * 0.275, Z_TILE + 0.005)),
        GameEntity,
    ));
    // 顶部亮带
    commands.spawn((
        Sprite::from_color(COLOR_WATER_HI, Vec2::new(w, 4.0)),
        Transform::from_translation(Vec3::new(cx, top_y - 2.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    // 4 行错位高光波纹
    for row in 0..4 {
        let row_y = top_y - 10.0 - row as f32 * 14.0;
        let phase_off = (row as f32) * 8.0;
        let spacing = 20.0 + (row as f32 % 2.0) * 6.0;
        let n = (w / spacing) as i32;
        for i in 0..n {
            let x = x_start + 6.0 + i as f32 * spacing + phase_off;
            if x > x_end - 6.0 {
                break;
            }
            let len = if (i + row) % 3 == 0 { 14.0 } else { 8.0 };
            commands.spawn((
                Sprite::from_color(COLOR_WATER_HI, Vec2::new(len, 2.0)),
                Transform::from_translation(Vec3::new(x, row_y, Z_TILE + 0.05)),
                GameEntity,
            ));
        }
    }
    // 顶端密集白色泡沫
    let n = (w / 8.0) as i32;
    for i in 0..n {
        let x = x_start + 4.0 + i as f32 * 8.0;
        let h_off = if i % 2 == 0 { 0.0 } else { 1.0 };
        commands.spawn((
            Sprite::from_color(COLOR_FOAM, Vec2::new(6.0, 2.0)),
            Transform::from_translation(Vec3::new(x, top_y + 1.0 + h_off, Z_TILE + 0.06)),
            GameEntity,
        ));
    }
    // 第二层小泡沫点（错位）
    for i in 0..n {
        if i % 3 != 1 {
            continue;
        }
        let x = x_start + 4.0 + i as f32 * 8.0;
        commands.spawn((
            Sprite::from_color(COLOR_FOAM, Vec2::new(2.0, 2.0)),
            Transform::from_translation(Vec3::new(x, top_y - 3.0, Z_TILE + 0.065)),
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
    // 桥面必须与陆地顶面齐平：低 1 单位会让站在桥上的人与陆地碰撞体重叠，
    // 每帧被沿 X 轴推回岸边，形成一堵走不过去的隐形墙。
    let plank_top = GROUND_TOP;
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
