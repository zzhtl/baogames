use bevy::prelude::*;
use rand::prelude::*;

use crate::common::render::{UiFont, panel, rect, text};

use super::model::*;

// ========== 常量 ==========
pub(super) const PLAY_W: f32 = 480.0;
pub(super) const PLAY_H: f32 = 520.0;
pub(super) const PLAY_OFFSET_X: f32 = -120.0;
pub(super) const PLAY_LEFT: f32 = PLAY_OFFSET_X - PLAY_W * 0.5;
pub(super) const PLAY_RIGHT: f32 = PLAY_OFFSET_X + PLAY_W * 0.5;
pub(super) const PLAY_TOP: f32 = PLAY_H * 0.5;
pub(super) const PLAY_BOTTOM: f32 = -PLAY_H * 0.5;

const BUBBLE_R: f32 = 18.0;
const BUBBLE_D: f32 = 36.0;
const ROW_H: f32 = 31.1769;
const COLS_EVEN: i32 = 12;
const COLS_ODD: i32 = 11;
const ROWS_MAX: usize = 14;
const TOP_PAD: f32 = 14.0;
const DEAD_Y: f32 = PLAY_BOTTOM + 70.0;

const CANNON_X: f32 = PLAY_OFFSET_X;
const CANNON_Y: f32 = PLAY_BOTTOM + 28.0;
const SHOT_SPEED: f32 = 720.0;
const MAX_AIM: f32 = 1.36;
const AIM_SPEED: f32 = 1.7;

const POP_LIFETIME: f32 = 0.22;
const FALL_GRAVITY: f32 = -780.0;

const Z_FRAME: f32 = 0.4;
const Z_BUBBLE: f32 = 1.2;
const Z_FLYING: f32 = 1.5;
const Z_CANNON: f32 = 2.0;
const Z_HUD: f32 = 8.0;

const COLORS_BY_LEVEL: [u8; 11] = [3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 6];
const INITIAL_ROWS_BY_LEVEL: [usize; 11] = [4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10];
const SHOTS_TO_DESCEND_BY_LEVEL: [i32; 11] = [12, 12, 11, 10, 10, 9, 9, 8, 8, 7, 6];

// ========== 组件 ==========
#[derive(Component, Clone, Copy)]
pub(super) struct BubbleCell {
    pub col: i32,
    pub row: i32,
}

#[derive(Component, Clone, Copy)]
pub(super) struct BubbleColor(pub u8);

#[derive(Component)]
pub(super) struct GridBubble;

#[derive(Component)]
pub(super) struct FlyingBubble {
    pub vel: Vec2,
}

#[derive(Component)]
pub(super) struct PoppingBubble {
    pub life: f32,
}

#[derive(Component)]
pub(super) struct FallingBubble {
    pub vy: f32,
}

#[derive(Component)]
pub(super) struct CannonBarrel;

#[derive(Component)]
pub(super) struct LoadedBubble;

#[derive(Component)]
pub(super) struct NextBubbleSprite;

#[derive(Component)]
pub(super) struct AimDot {
    pub idx: usize,
}

#[derive(Component)]
pub(super) struct BubbleHud;

#[derive(Component)]
pub(super) struct BubbleMessage;

// ========== 资源 ==========
#[derive(Resource)]
pub(super) struct BubbleStage {
    pub grid: Vec<Vec<Option<u8>>>,
    pub descend: usize,
    pub aim: f32,
    pub current: u8,
    pub next: u8,
    pub shot_active: bool,
    pub shots_left_for_descend: i32,
    pub max_shots_per_descend: i32,
    #[allow(dead_code)]
    pub colors_count: u8,
    pub message: String,
    pub message_clock: f32,
    pub flash_clock: f32,
}

// ========== 工具函数 ==========
fn palette(idx: u8) -> Color {
    match idx {
        0 => Color::srgb(0.94, 0.32, 0.32),
        1 => Color::srgb(0.32, 0.55, 0.96),
        2 => Color::srgb(0.34, 0.82, 0.42),
        3 => Color::srgb(0.98, 0.88, 0.28),
        4 => Color::srgb(0.82, 0.42, 0.96),
        _ => Color::srgb(0.96, 0.6, 0.24),
    }
}

fn palette_dark(idx: u8) -> Color {
    let c = palette(idx).to_srgba();
    Color::srgb(c.red * 0.55, c.green * 0.55, c.blue * 0.55)
}

fn level_idx(level: u8) -> usize {
    (level as usize).clamp(1, 10)
}

fn cols_in_row(row: i32) -> i32 {
    if row.rem_euclid(2) == 1 {
        COLS_ODD
    } else {
        COLS_EVEN
    }
}

fn cell_valid(col: i32, row: i32) -> bool {
    row >= 0 && (row as usize) < ROWS_MAX && col >= 0 && col < cols_in_row(row)
}

fn cell_to_pos(col: i32, row: i32, descend: usize) -> Vec2 {
    let offset = if row.rem_euclid(2) == 1 {
        BUBBLE_R
    } else {
        0.0
    };
    let x = PLAY_LEFT + BUBBLE_R + offset + col as f32 * BUBBLE_D;
    let y = PLAY_TOP - TOP_PAD - BUBBLE_R - (row as f32 + descend as f32) * ROW_H;
    Vec2::new(x, y)
}

fn neighbors(col: i32, row: i32) -> [(i32, i32); 6] {
    if row.rem_euclid(2) == 0 {
        [
            (col - 1, row),
            (col + 1, row),
            (col - 1, row - 1),
            (col, row - 1),
            (col - 1, row + 1),
            (col, row + 1),
        ]
    } else {
        [
            (col - 1, row),
            (col + 1, row),
            (col, row - 1),
            (col + 1, row - 1),
            (col, row + 1),
            (col + 1, row + 1),
        ]
    }
}

fn snap_nearest_empty(
    pos: Vec2,
    descend: usize,
    grid: &[Vec<Option<u8>>],
) -> Option<(i32, i32)> {
    let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - pos.y) / ROW_H - descend as f32;
    let row0 = row_f.round() as i32;
    let mut best: Option<((i32, i32), f32)> = None;
    for r in (row0 - 1)..=(row0 + 1) {
        if r < 0 || (r as usize) >= ROWS_MAX {
            continue;
        }
        let offset = if r.rem_euclid(2) == 1 {
            BUBBLE_R
        } else {
            0.0
        };
        let col_f = (pos.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
        let col0 = col_f.round() as i32;
        for c in (col0 - 1)..=(col0 + 1) {
            if !cell_valid(c, r) {
                continue;
            }
            if grid[r as usize][c as usize].is_some() {
                continue;
            }
            // 必须与至少一个邻居或顶部相邻才能放置
            let touches_top = r == 0;
            let touches_neighbor = neighbors(c, r).iter().any(|(nc, nr)| {
                cell_valid(*nc, *nr) && grid[*nr as usize][*nc as usize].is_some()
            });
            if !touches_top && !touches_neighbor {
                continue;
            }
            let center = cell_to_pos(c, r, descend);
            let d2 = (center - pos).length_squared();
            match best {
                None => best = Some(((c, r), d2)),
                Some((_, bd)) if d2 < bd => best = Some(((c, r), d2)),
                _ => {}
            }
        }
    }
    best.map(|(cell, _)| cell)
}

fn flood_same_color(
    start: (i32, i32),
    color: u8,
    grid: &[Vec<Option<u8>>],
) -> Vec<(i32, i32)> {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut stack = vec![start];
    while let Some((c, r)) = stack.pop() {
        if !cell_valid(c, r) {
            continue;
        }
        if visited.contains(&(c, r)) {
            continue;
        }
        if grid[r as usize][c as usize] != Some(color) {
            continue;
        }
        visited.push((c, r));
        for (nc, nr) in neighbors(c, r) {
            stack.push((nc, nr));
        }
    }
    visited
}

fn floating_cells(grid: &[Vec<Option<u8>>]) -> Vec<(i32, i32)> {
    let mut reached = vec![vec![false; COLS_EVEN as usize]; ROWS_MAX];
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for c in 0..cols_in_row(0) {
        if grid[0][c as usize].is_some() {
            stack.push((c, 0));
        }
    }
    while let Some((c, r)) = stack.pop() {
        if !cell_valid(c, r) {
            continue;
        }
        if reached[r as usize][c as usize] {
            continue;
        }
        if grid[r as usize][c as usize].is_none() {
            continue;
        }
        reached[r as usize][c as usize] = true;
        for (nc, nr) in neighbors(c, r) {
            stack.push((nc, nr));
        }
    }
    let mut result = Vec::new();
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if grid[r][c as usize].is_some() && !reached[r][c as usize] {
                result.push((c, r as i32));
            }
        }
    }
    result
}

fn available_colors(grid: &[Vec<Option<u8>>]) -> Vec<u8> {
    let mut set = [false; 6];
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if let Some(col) = grid[r][c as usize] {
                if (col as usize) < 6 {
                    set[col as usize] = true;
                }
            }
        }
    }
    (0u8..6u8).filter(|i| set[*i as usize]).collect()
}


fn grid_is_empty(grid: &[Vec<Option<u8>>]) -> bool {
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if grid[r][c as usize].is_some() {
                return false;
            }
        }
    }
    true
}

fn aim_dir(aim: f32) -> Vec2 {
    Vec2::new(aim.sin(), aim.cos())
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    // 主背景
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, 0.0),
        Vec2::new(PLAY_W, PLAY_H),
        Color::srgb(0.07, 0.06, 0.13),
        GameEntity,
    );
    // 上下两条带
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.1, 0.08, 0.18),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM + 36.0),
        Vec2::new(PLAY_W, 72.0),
        Color::srgb(0.1, 0.08, 0.18),
        GameEntity,
    );
    // 顶部"天花板"
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_TOP - 4.0),
        Vec2::new(PLAY_W, 6.0),
        Color::srgb(0.62, 0.34, 0.62),
        GameEntity,
    );
    // 死亡线
    for i in 0..16 {
        let x = PLAY_LEFT + 6.0 + i as f32 * (PLAY_W - 12.0) / 16.0 + 8.0;
        rect(
            commands,
            Vec2::new(x, DEAD_Y),
            Vec2::new(14.0, 2.0),
            Color::srgb(0.85, 0.32, 0.42),
            GameEntity,
        );
    }

    // 边框
    let frame_thickness = 6.0;
    let outer_w = PLAY_W + frame_thickness * 2.0;
    let outer_h = PLAY_H + frame_thickness * 2.0;
    let frame_color = Color::srgb(0.46, 0.32, 0.6);
    for (pos, size) in [
        (
            Vec2::new(PLAY_OFFSET_X, PLAY_TOP + frame_thickness * 0.5),
            Vec2::new(outer_w, frame_thickness),
        ),
        (
            Vec2::new(PLAY_OFFSET_X, PLAY_BOTTOM - frame_thickness * 0.5),
            Vec2::new(outer_w, frame_thickness),
        ),
        (
            Vec2::new(PLAY_LEFT - frame_thickness * 0.5, 0.0),
            Vec2::new(frame_thickness, outer_h),
        ),
        (
            Vec2::new(PLAY_RIGHT + frame_thickness * 0.5, 0.0),
            Vec2::new(frame_thickness, outer_h),
        ),
    ] {
        commands.spawn((
            Sprite::from_color(frame_color, size),
            Transform::from_translation(pos.extend(Z_FRAME)),
            GameEntity,
        ));
    }

    // 关卡参数
    let li = level_idx(level);
    let colors_count = COLORS_BY_LEVEL[li];
    let initial_rows = INITIAL_ROWS_BY_LEVEL[li];
    let max_shots = SHOTS_TO_DESCEND_BY_LEVEL[li];

    // 生成网格
    let mut grid: Vec<Vec<Option<u8>>> = vec![vec![None; COLS_EVEN as usize]; ROWS_MAX];
    let mut rng = thread_rng();
    for r in 0..initial_rows {
        let cols = cols_in_row(r as i32);
        for c in 0..cols {
            grid[r][c as usize] = Some(rng.gen_range(0..colors_count));
        }
    }

    // 生成网格泡泡精灵
    for r in 0..ROWS_MAX {
        for c in 0..cols_in_row(r as i32) {
            if let Some(color_id) = grid[r][c as usize] {
                spawn_grid_bubble(commands, c, r as i32, color_id, 0);
            }
        }
    }

    // 大炮底座
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 18.0),
        Vec2::new(64.0, 18.0),
        Color::srgb(0.28, 0.18, 0.32),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X, CANNON_Y - 18.0),
        Vec2::new(56.0, 10.0),
        Color::srgb(0.7, 0.46, 0.78),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X - 28.0, CANNON_Y - 8.0),
        Vec2::new(8.0, 12.0),
        Color::srgb(0.42, 0.28, 0.5),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(CANNON_X + 28.0, CANNON_Y - 8.0),
        Vec2::new(8.0, 12.0),
        Color::srgb(0.42, 0.28, 0.5),
        GameEntity,
    );

    // 炮管 (旋转的)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.62, 0.4, 0.7), Vec2::new(14.0, 44.0)),
        Transform {
            translation: Vec3::new(CANNON_X, CANNON_Y + 22.0, Z_CANNON),
            rotation: Quat::IDENTITY,
            ..default()
        },
        CannonBarrel,
        GameEntity,
    ));

    // 决定首发与下一发颜色
    let current = rng.gen_range(0..colors_count);
    let next = rng.gen_range(0..colors_count);

    // 装填的当前泡泡（在炮口）
    commands.spawn((
        Sprite::from_color(palette(current), Vec2::splat(BUBBLE_D - 6.0)),
        Transform::from_translation(Vec3::new(CANNON_X, CANNON_Y + 6.0, Z_CANNON - 0.1)),
        LoadedBubble,
        BubbleColor(current),
        GameEntity,
    ));
    // 装填泡泡的高光
    commands.spawn((
        Sprite::from_color(
            Color::srgba(1.0, 1.0, 1.0, 0.45),
            Vec2::new(8.0, 8.0),
        ),
        Transform::from_translation(Vec3::new(
            CANNON_X - 7.0,
            CANNON_Y + 12.0,
            Z_CANNON - 0.05,
        )),
        LoadedBubble,
        GameEntity,
    ));

    // 下一发预览
    let preview_x = CANNON_X - 92.0;
    let preview_y = CANNON_Y - 4.0;
    panel(
        commands,
        Vec2::new(preview_x, preview_y),
        Vec2::new(56.0, 56.0),
        Color::srgb(0.12, 0.1, 0.18),
        Color::srgb(0.5, 0.34, 0.6),
        GameEntity,
    );
    text(
        commands,
        font,
        "下一",
        Vec2::new(preview_x, preview_y + 38.0),
        12.0,
        Color::srgb(0.86, 0.78, 0.96),
        GameEntity,
    );
    commands.spawn((
        Sprite::from_color(palette(next), Vec2::splat(BUBBLE_D - 8.0)),
        Transform::from_translation(Vec3::new(preview_x, preview_y, Z_CANNON - 0.1)),
        NextBubbleSprite,
        BubbleColor(next),
        GameEntity,
    ));

    // 瞄准虚线
    for i in 0..7 {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.92, 0.86, 1.0, 0.65),
                Vec2::splat(4.0),
            ),
            Transform::from_translation(Vec3::new(
                CANNON_X,
                CANNON_Y + 50.0 + i as f32 * 18.0,
                Z_CANNON - 0.2,
            )),
            AimDot { idx: i },
            GameEntity,
        ));
    }

    // HUD
    let hud_x = PLAY_RIGHT + 110.0;
    panel(
        commands,
        Vec2::new(hud_x, 180.0),
        Vec2::new(180.0, 230.0),
        Color::srgb(0.1, 0.08, 0.16),
        Color::srgb(0.7, 0.4, 0.78),
        GameEntity,
    );
    text(
        commands,
        font,
        "泡泡龙",
        Vec2::new(hud_x, 270.0),
        22.0,
        Color::srgb(1.0, 0.88, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        "P1\n← / → 或 A/D 瞄准\nSpace / J 发射\nEsc 暂停",
        Vec2::new(hud_x, 200.0),
        14.0,
        Color::srgb(0.92, 0.78, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        "",
        Vec2::new(hud_x, 110.0),
        16.0,
        Color::srgb(1.0, 0.94, 0.7),
        GameEntity,
    )
    .insert(BubbleHud);

    // 中央提示
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(24.0).with_font(font.0.clone()),
        TextColor(Color::srgb(1.0, 0.92, 0.5)),
        Transform::from_translation(Vec3::new(PLAY_OFFSET_X, PLAY_TOP - 56.0, Z_HUD)),
        BubbleMessage,
        GameEntity,
    ));

    commands.insert_resource(BubbleStage {
        grid,
        descend: 0,
        aim: 0.0,
        current,
        next,
        shot_active: false,
        shots_left_for_descend: max_shots,
        max_shots_per_descend: max_shots,
        colors_count,
        message: format!("第 {} 关 - 三连同色消除", level),
        message_clock: 2.4,
        flash_clock: 0.0,
    });
}

fn spawn_grid_bubble(commands: &mut Commands, col: i32, row: i32, color_id: u8, descend: usize) {
    let pos = cell_to_pos(col, row, descend);
    let body_color = palette(color_id);
    // 主体
    commands.spawn((
        Sprite::from_color(body_color, Vec2::splat(BUBBLE_D - 4.0)),
        Transform::from_translation(pos.extend(Z_BUBBLE)),
        GridBubble,
        BubbleCell { col, row },
        BubbleColor(color_id),
        GameEntity,
    ));
    // 暗色描边内圈
    commands.spawn((
        Sprite::from_color(palette_dark(color_id), Vec2::splat(BUBBLE_D - 14.0)),
        Transform::from_translation(Vec3::new(pos.x, pos.y - 5.0, Z_BUBBLE + 0.05)),
        GridBubble,
        BubbleCell { col, row },
        GameEntity,
    ));
    // 高光
    commands.spawn((
        Sprite::from_color(
            Color::srgba(1.0, 1.0, 1.0, 0.55),
            Vec2::new(7.0, 7.0),
        ),
        Transform::from_translation(Vec3::new(pos.x - 7.0, pos.y + 6.0, Z_BUBBLE + 0.1)),
        GridBubble,
        BubbleCell { col, row },
        GameEntity,
    ));
}

// ========== 系统 ==========

pub(super) fn bubble_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<BubbleStage>,
    mut barrel_q: Query<&mut Transform, (With<CannonBarrel>, Without<LoadedBubble>)>,
    mut loaded_q: Query<
        (Entity, &mut Transform, Option<&BubbleColor>),
        (With<LoadedBubble>, Without<CannonBarrel>),
    >,
) {
    if session.kind != GameKind::BubbleBobble || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    if stage.message_clock > 0.0 {
        stage.message_clock = (stage.message_clock - dt).max(0.0);
    }
    if stage.flash_clock > 0.0 {
        stage.flash_clock = (stage.flash_clock - dt).max(0.0);
    }
    let mut aim = stage.aim;
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        aim -= AIM_SPEED * dt;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        aim += AIM_SPEED * dt;
    }
    aim = aim.clamp(-MAX_AIM, MAX_AIM);
    stage.aim = aim;

    let dir = aim_dir(aim);
    if let Ok(mut t) = barrel_q.single_mut() {
        let center = Vec2::new(CANNON_X, CANNON_Y) + dir * 22.0;
        t.translation.x = center.x;
        t.translation.y = center.y;
        t.rotation = Quat::from_rotation_z(-aim);
    }
    let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + dir * 6.0;
    let highlight_local = dir.perp() * -7.0 + dir * 6.0;
    let mut had_main = false;
    for (_e, mut t, color) in &mut loaded_q {
        if color.is_some() {
            t.translation.x = loaded_pos.x;
            t.translation.y = loaded_pos.y;
            had_main = true;
        } else {
            // 高光
            t.translation.x = loaded_pos.x + highlight_local.x;
            t.translation.y = loaded_pos.y + highlight_local.y;
        }
    }
    let _ = had_main;

    // 发射
    let shoot = !stage.shot_active
        && (keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::KeyJ)
            || keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::KeyW));
    if shoot {
        let muzzle = Vec2::new(CANNON_X, CANNON_Y) + dir * 36.0;
        let color_id = stage.current;
        // 飞行泡泡
        commands.spawn((
            Sprite::from_color(palette(color_id), Vec2::splat(BUBBLE_D - 4.0)),
            Transform::from_translation(muzzle.extend(Z_FLYING)),
            FlyingBubble {
                vel: dir * SHOT_SPEED,
            },
            BubbleColor(color_id),
            GameEntity,
        ));
        // 销毁炮口处的装填精灵（主体+高光）
        for (e, _t, _c) in &loaded_q {
            commands.entity(e).despawn();
        }
        stage.shot_active = true;
    }
}

pub(super) fn bubble_aim_dots_update(
    session: Res<GameSession>,
    stage: Res<BubbleStage>,
    mut dots: Query<(&AimDot, &mut Transform, &mut Sprite)>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    let visible = !stage.shot_active && !session.paused && !session.finished;
    let dir = aim_dir(stage.aim);
    let start = Vec2::new(CANNON_X, CANNON_Y) + dir * 36.0;
    let mut p = start;
    let mut v = dir;
    let step = 22.0;
    for i in 0..7 {
        // 简单步进 + 墙反射；遇到泡泡或顶部停在那个位置
        let mut traveled = 0.0;
        let segment = step;
        let mut hit = false;
        while traveled < segment {
            let remain = segment - traveled;
            let next_x = p.x + v.x * remain;
            // 撞左右墙
            if v.x < 0.0 && next_x - BUBBLE_R < PLAY_LEFT {
                let t = (PLAY_LEFT + BUBBLE_R - p.x) / v.x;
                p += v * t;
                v.x = -v.x;
                traveled += t;
                continue;
            }
            if v.x > 0.0 && next_x + BUBBLE_R > PLAY_RIGHT {
                let t = (PLAY_RIGHT - BUBBLE_R - p.x) / v.x;
                p += v * t;
                v.x = -v.x;
                traveled += t;
                continue;
            }
            p += v * remain;
            traveled = segment;
        }
        // 顶部检测：超过最高 row 0 中心 y 即停止
        let top_y = cell_to_pos(0, 0, stage.descend).y;
        if p.y >= top_y {
            hit = true;
        }
        // 网格碰撞检测：若靠近任何已占据 cell 则停止
        if !hit {
            let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - p.y) / ROW_H - stage.descend as f32;
            let row0 = row_f.round() as i32;
            'outer: for r in (row0 - 1).max(0)..=(row0 + 1).min(ROWS_MAX as i32 - 1) {
                let offset = if r.rem_euclid(2) == 1 {
                    BUBBLE_R
                } else {
                    0.0
                };
                let col_f = (p.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
                let col0 = col_f.round() as i32;
                for c in (col0 - 1)..=(col0 + 1) {
                    if !cell_valid(c, r) {
                        continue;
                    }
                    if stage.grid[r as usize][c as usize].is_some() {
                        let center = cell_to_pos(c, r, stage.descend);
                        if (center - p).length_squared() < (BUBBLE_D - 1.0).powi(2) {
                            hit = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
        for (dot, mut t, mut sp) in &mut dots {
            if dot.idx == i {
                t.translation.x = p.x;
                t.translation.y = p.y;
                sp.color = if visible {
                    let alpha = if hit { 0.25 } else { 0.7 - i as f32 * 0.06 };
                    Color::srgba(0.92, 0.86, 1.0, alpha)
                } else {
                    Color::srgba(0.0, 0.0, 0.0, 0.0)
                };
            }
        }
        if hit {
            // 后续点设为不可见
            for (dot, mut t, mut sp) in &mut dots {
                if dot.idx > i {
                    t.translation.x = p.x;
                    t.translation.y = p.y;
                    sp.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                }
            }
            return;
        }
    }
}

pub(super) fn bubble_shot_update(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<BubbleStage>,
    mut shot_q: Query<(Entity, &mut Transform, &mut FlyingBubble, &BubbleColor)>,
    grid_q: Query<(Entity, &BubbleCell), With<GridBubble>>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();

    let Ok((shot_e, mut tr, mut fly, color)) = shot_q.single_mut() else {
        return;
    };

    // 步进 + 反射，使用细粒度子步以避免穿透
    let mut remaining = dt;
    let max_step = BUBBLE_R * 0.6;
    let mut settled: Option<Vec2> = None;
    while remaining > 0.0 {
        let speed = fly.vel.length().max(1.0);
        let step_dt = (max_step / speed).min(remaining);
        let next = Vec2::new(
            tr.translation.x + fly.vel.x * step_dt,
            tr.translation.y + fly.vel.y * step_dt,
        );
        // 左右墙反射
        let mut new_pos = next;
        if new_pos.x - BUBBLE_R < PLAY_LEFT {
            new_pos.x = PLAY_LEFT + BUBBLE_R;
            fly.vel.x = fly.vel.x.abs();
        } else if new_pos.x + BUBBLE_R > PLAY_RIGHT {
            new_pos.x = PLAY_RIGHT - BUBBLE_R;
            fly.vel.x = -fly.vel.x.abs();
        }
        // 顶部判定
        let top_y = cell_to_pos(0, 0, stage.descend).y;
        if new_pos.y >= top_y {
            settled = Some(Vec2::new(new_pos.x, top_y));
            tr.translation.x = new_pos.x;
            tr.translation.y = top_y;
            break;
        }
        // 网格碰撞
        let mut hit = false;
        let row_f = (PLAY_TOP - TOP_PAD - BUBBLE_R - new_pos.y) / ROW_H - stage.descend as f32;
        let row0 = row_f.round() as i32;
        'outer: for r in (row0 - 1).max(0)..=(row0 + 1).min(ROWS_MAX as i32 - 1) {
            let offset = if r.rem_euclid(2) == 1 {
                BUBBLE_R
            } else {
                0.0
            };
            let col_f = (new_pos.x - PLAY_LEFT - BUBBLE_R - offset) / BUBBLE_D;
            let col0 = col_f.round() as i32;
            for c in (col0 - 1)..=(col0 + 1) {
                if !cell_valid(c, r) {
                    continue;
                }
                if stage.grid[r as usize][c as usize].is_some() {
                    let center = cell_to_pos(c, r, stage.descend);
                    if (center - new_pos).length_squared() < (BUBBLE_D - 1.0).powi(2) {
                        hit = true;
                        break 'outer;
                    }
                }
            }
        }
        tr.translation.x = new_pos.x;
        tr.translation.y = new_pos.y;
        if hit {
            settled = Some(new_pos);
            break;
        }
        remaining -= step_dt;
    }

    if let Some(pos) = settled {
        commands.entity(shot_e).despawn();
        let placed = snap_nearest_empty(pos, stage.descend, &stage.grid);
        let Some((col, row)) = placed else {
            // 没找到可用位置（可能溢出），强制结束
            stage.shot_active = false;
            finish_bubble(&mut session, &mut save, false);
            return;
        };
        let color_id = color.0;
        stage.grid[row as usize][col as usize] = Some(color_id);
        spawn_grid_bubble(&mut commands, col, row, color_id, stage.descend);

        // 同色聚集
        let group = flood_same_color((col, row), color_id, &stage.grid);
        let mut popped = 0u32;
        let mut fell = 0u32;
        if group.len() >= 3 {
            for (gc, gr) in &group {
                stage.grid[*gr as usize][*gc as usize] = None;
            }
            popped = group.len() as u32;
            // 把网格里这些 cell 的精灵改为爆裂态
            for (e, cell) in &grid_q {
                if group.iter().any(|(gc, gr)| *gc == cell.col && *gr == cell.row) {
                    commands
                        .entity(e)
                        .remove::<GridBubble>()
                        .insert(PoppingBubble { life: POP_LIFETIME });
                }
            }
            // 检查浮空泡泡
            let floating = floating_cells(&stage.grid);
            for (fc, fr) in &floating {
                stage.grid[*fr as usize][*fc as usize] = None;
            }
            fell = floating.len() as u32;
            for (e, cell) in &grid_q {
                if floating
                    .iter()
                    .any(|(fc, fr)| *fc == cell.col && *fr == cell.row)
                {
                    commands
                        .entity(e)
                        .remove::<GridBubble>()
                        .insert(FallingBubble { vy: 30.0 });
                }
            }
        }

        // 计分
        if popped > 0 {
            session.score += popped * 50;
            // 连消额外加分
            if popped >= 5 {
                session.score += (popped - 4) * 30;
            }
        }
        if fell > 0 {
            session.score += fell * 100;
        }

        // 决定是否下移天花板
        if popped == 0 {
            stage.shots_left_for_descend -= 1;
            if stage.shots_left_for_descend <= 0 {
                descend_grid(&mut commands, &mut stage, &grid_q);
            }
        } else {
            stage.shots_left_for_descend = stage.max_shots_per_descend;
        }

        // 检查死亡线
        if !grid_is_empty(&stage.grid) {
            for r in (0..ROWS_MAX).rev() {
                for c in 0..cols_in_row(r as i32) {
                    if stage.grid[r][c as usize].is_some() {
                        let p = cell_to_pos(c, r as i32, stage.descend);
                        if p.y - BUBBLE_R <= DEAD_Y {
                            stage.shot_active = false;
                            finish_bubble(&mut session, &mut save, false);
                            return;
                        }
                    }
                }
            }
        }

        // 准备下一发
        let mut rng = thread_rng();
        // 当前色用 next，next 重新滚（仅用尚存色）
        let new_current = stage.next;
        let avail = available_colors(&stage.grid);
        let new_next = if avail.is_empty() {
            stage.next
        } else {
            // 若当前色不在 avail，重新挑
            let cur_ok = avail.iter().any(|c| *c == new_current);
            let cur = if cur_ok {
                new_current
            } else {
                avail[rng.gen_range(0..avail.len())]
            };
            // 更新 stage.current 为可用
            stage.current = cur;
            avail[rng.gen_range(0..avail.len())]
        };
        if avail.is_empty() {
            stage.current = new_current;
        }
        stage.next = new_next;
        stage.shot_active = false;

        // 重新生成 LoadedBubble（主+高光）
        let dir = aim_dir(stage.aim);
        let loaded_pos = Vec2::new(CANNON_X, CANNON_Y) + dir * 6.0;
        commands.spawn((
            Sprite::from_color(palette(stage.current), Vec2::splat(BUBBLE_D - 6.0)),
            Transform::from_translation(loaded_pos.extend(Z_CANNON - 0.1)),
            LoadedBubble,
            BubbleColor(stage.current),
            GameEntity,
        ));
        let highlight_local = dir.perp() * -7.0 + dir * 6.0;
        commands.spawn((
            Sprite::from_color(
                Color::srgba(1.0, 1.0, 1.0, 0.45),
                Vec2::new(8.0, 8.0),
            ),
            Transform::from_translation(
                (loaded_pos + highlight_local).extend(Z_CANNON - 0.05),
            ),
            LoadedBubble,
            GameEntity,
        ));

        // 胜利判定
        if grid_is_empty(&stage.grid) {
            finish_bubble(&mut session, &mut save, true);
        }
    }
}

fn descend_grid(
    commands: &mut Commands,
    stage: &mut BubbleStage,
    grid_q: &Query<(Entity, &BubbleCell), With<GridBubble>>,
) {
    stage.descend += 1;
    stage.shots_left_for_descend = stage.max_shots_per_descend;
    stage.message = "天花板下移！".to_string();
    stage.message_clock = 1.4;
    stage.flash_clock = 0.4;
    // 所有网格泡泡精灵下移 ROW_H
    for (e, cell) in grid_q {
        let new_pos = cell_to_pos(cell.col, cell.row, stage.descend);
        commands.entity(e).insert(Transform::from_translation(
            new_pos.extend(Z_BUBBLE),
        ));
    }
}

pub(super) fn bubble_pop_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(Entity, &mut PoppingBubble, &mut Sprite, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (e, mut pop, mut sp, mut tr) in &mut q {
        pop.life -= dt;
        if pop.life <= 0.0 {
            commands.entity(e).despawn();
        } else {
            let t = pop.life / POP_LIFETIME;
            let s = (1.0 - t) * 1.4 + t;
            tr.scale = Vec3::splat(s.max(0.1));
            let mut c = sp.color.to_srgba();
            c.alpha = (t * 0.9 + 0.05).clamp(0.0, 1.0);
            sp.color = Color::srgba(c.red, c.green, c.blue, c.alpha);
        }
    }
}

pub(super) fn bubble_fall_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(Entity, &mut FallingBubble, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (e, mut fall, mut tr) in &mut q {
        fall.vy += FALL_GRAVITY * dt;
        tr.translation.y += fall.vy * dt;
        if tr.translation.y < PLAY_BOTTOM - 30.0 {
            commands.entity(e).despawn();
        }
    }
}

pub(super) fn bubble_next_preview_update(
    session: Res<GameSession>,
    stage: Res<BubbleStage>,
    mut q: Query<(&mut Sprite, &mut BubbleColor), With<NextBubbleSprite>>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    for (mut sp, mut bc) in &mut q {
        if bc.0 != stage.next {
            bc.0 = stage.next;
            sp.color = palette(stage.next);
        }
    }
}

pub(super) fn bubble_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<BubbleStage>,
    grid_q: Query<&BubbleCell, With<GridBubble>>,
    mut hud: Query<&mut Text2d, (With<BubbleHud>, Without<BubbleMessage>)>,
    mut msg: Query<&mut Text2d, (With<BubbleMessage>, Without<BubbleHud>)>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let high = save.high_scores[GameKind::BubbleBobble.index()].max(session.score);
        // 数泡泡：用 grid 数据更准确
        let mut left = 0u32;
        for r in 0..ROWS_MAX {
            for c in 0..cols_in_row(r as i32) {
                if stage.grid[r][c as usize].is_some() {
                    left += 1;
                }
            }
        }
        let _ = grid_q;
        **t = format!(
            "分数 {}\n纪录 {}\n第 {} 关\n剩余 {} 颗\n{} 步后下移",
            session.score,
            high,
            session.level,
            left,
            stage.shots_left_for_descend.max(0),
        );
    }
    if let Ok(mut t) = msg.single_mut() {
        **t = if session.finished {
            if session.won {
                "通关！Enter 重玩，Esc 返回".to_string()
            } else {
                "撞到死亡线…Enter 重试，Esc 返回".to_string()
            }
        } else if session.paused {
            "已暂停：Esc 继续，Backspace 返回".to_string()
        } else if stage.message_clock > 0.0 {
            stage.message.clone()
        } else {
            String::new()
        };
    }
}

fn finish_bubble(session: &mut GameSession, save: &mut SaveData, won: bool) {
    if session.finished {
        return;
    }
    session.finished = true;
    session.won = won;
    let idx = GameKind::BubbleBobble.index();
    if won {
        session.score += 1000;
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] =
            save.unlocked_levels[idx].max((session.level + 1).min(10));
        save.store();
        session.status = "通关！Enter 重玩，Esc 返回".to_string();
    } else {
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.store();
        session.status = "撞到死亡线…Enter 重试，Esc 返回".to_string();
    }
}
