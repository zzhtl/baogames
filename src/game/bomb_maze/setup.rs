use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::render::{UiFont, rect, text};
use crate::common::sprite_def::SpriteDef;
use crate::game::model::{Collider, GameEntity};

use super::components::*;
use super::constants::*;
use super::geometry::{in_safe_zone, is_border, is_pillar, tile_center};
use super::resources::BMStage;
use super::sprites::*;

// ========== 关卡建图 ==========
pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    paint_field(commands);

    let mut rng = thread_rng();
    let soft_cells = generate_soft_cells(&mut rng);
    let hidden_map = decide_hidden_items(&soft_cells, level);

    // 生成硬墙
    for c in 0..BM_COLS {
        for r in 0..BM_ROWS {
            if is_border(c, r) || is_pillar(c, r) {
                spawn_hard_wall(commands, c, r);
            }
        }
    }
    // 生成软砖
    for (c, r) in &soft_cells {
        let hides = hidden_map
            .get(&(*c, *r))
            .copied()
            .unwrap_or(HiddenItem::Nothing);
        spawn_soft_wall(commands, *c, *r, hides);
    }

    // 玩家
    spawn_bm_player(commands, 0, P1_SPAWN);
    spawn_bm_player(commands, 1, P2_SPAWN);

    // 敌人：等级越高越多种
    let enemies = pick_enemies_for_level(level);
    let mut spawn_cells = enemy_spawn_cells();
    spawn_cells.shuffle(&mut rng);
    for (i, kind) in enemies.iter().enumerate() {
        let (c, r) = spawn_cells[i % spawn_cells.len()];
        spawn_bm_enemy(commands, c, r, *kind);
    }

    spawn_hud(commands, font, level);

    commands.insert_resource(BMStage {
        level,
        time_left: 200.0,
        p1_lives: 3,
        p2_lives: 3,
        p1_respawn: 0.0,
        p2_respawn: 0.0,
        all_enemies_dead_msg_shown: false,
        status: "炸开软砖，找到出口逃出迷宫！".to_string(),
    });
}

fn generate_soft_cells(rng: &mut impl Rng) -> Vec<(i32, i32)> {
    let mut soft_cells: Vec<(i32, i32)> = Vec::new();
    for c in 1..BM_COLS - 1 {
        for r in 1..BM_ROWS - 1 {
            if is_border(c, r) || is_pillar(c, r) || in_safe_zone(c, r) {
                continue;
            }
            if rng.r#gen::<f32>() < SOFT_WALL_DENSITY {
                soft_cells.push((c, r));
            }
        }
    }
    soft_cells.shuffle(rng);
    soft_cells
}

fn decide_hidden_items(
    soft_cells: &[(i32, i32)],
    level: u8,
) -> std::collections::HashMap<(i32, i32), HiddenItem> {
    let mut map = std::collections::HashMap::new();
    let exit_idx = if !soft_cells.is_empty() { Some(0_usize) } else { None };
    let powerup_count = (3 + (level / 2) as usize).min(soft_cells.len().saturating_sub(1));
    let powerup_kinds = pick_powerup_kinds(level, powerup_count);

    if let Some(idx) = exit_idx {
        map.insert(soft_cells[idx], HiddenItem::Exit);
    }
    for (i, kind) in powerup_kinds.iter().enumerate() {
        let cell = soft_cells[exit_idx.map(|e| e + 1 + i).unwrap_or(i)];
        map.insert(cell, HiddenItem::Powerup(*kind));
    }
    map
}

fn pick_powerup_kinds(level: u8, count: usize) -> Vec<PowerupKind> {
    let mut pool = vec![
        PowerupKind::Fire,
        PowerupKind::Fire,
        PowerupKind::Bomb,
        PowerupKind::Bomb,
        PowerupKind::Speed,
    ];
    if level >= 2 {
        pool.push(PowerupKind::BombPass);
    }
    if level >= 3 {
        pool.push(PowerupKind::Detonator);
    }
    let mut rng = thread_rng();
    pool.shuffle(&mut rng);
    pool.truncate(count);
    pool
}

fn pick_enemies_for_level(level: u8) -> Vec<EnemyKind> {
    let lvl = level.max(1) as usize;
    let mut list = Vec::new();
    let balloom = (3 + lvl).min(7);
    for _ in 0..balloom {
        list.push(EnemyKind::Balloom);
    }
    if level >= 2 {
        for _ in 0..(1 + lvl / 2).min(4) {
            list.push(EnemyKind::Oneal);
        }
    }
    if level >= 4 {
        for _ in 0..((lvl / 3).min(3)) {
            list.push(EnemyKind::Doll);
        }
    }
    if level >= 6 {
        for _ in 0..((lvl / 4).min(2)) {
            list.push(EnemyKind::Kondoria);
        }
    }
    list
}

fn enemy_spawn_cells() -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    let candidates = [
        (13, 1),
        (13, 9),
        (13, 5),
        (7, 5),
        (9, 1),
        (9, 9),
        (5, 5),
        (11, 3),
        (11, 7),
    ];
    for (c, r) in candidates {
        if !is_border(c, r) && !is_pillar(c, r) {
            cells.push((c, r));
        }
    }
    cells
}

// ========== 绘制 / 实体生成 ==========
fn paint_field(commands: &mut Commands) {
    // 草地底色
    rect(
        commands,
        Vec2::new(BM_OFFSET_X, BM_OFFSET_Y),
        Vec2::new(BM_PLAY_W, BM_PLAY_H),
        Color::srgb(0.18, 0.32, 0.18),
        GameEntity,
    )
    .insert(Transform::from_translation(Vec3::new(
        BM_OFFSET_X,
        BM_OFFSET_Y,
        Z_FLOOR,
    )));

    // 浅色横纹做点装饰
    for r in 0..BM_ROWS {
        let y = BM_OFFSET_Y + BM_PLAY_H * 0.5 - r as f32 * BM_TILE - BM_TILE * 0.5;
        let color = if r % 2 == 0 {
            Color::srgba(1.0, 1.0, 1.0, 0.04)
        } else {
            Color::srgba(0.0, 0.0, 0.0, 0.06)
        };
        commands.spawn((
            Sprite::from_color(color, Vec2::new(BM_PLAY_W - 4.0, BM_TILE - 2.0)),
            Transform::from_translation(Vec3::new(BM_OFFSET_X, y, Z_GRID)),
            GameEntity,
        ));
    }

    // 外框
    let frame = 5.0;
    let outer_w = BM_PLAY_W + frame * 2.0;
    let outer_h = BM_PLAY_H + frame * 2.0;
    let frame_color = Color::srgb(0.25, 0.18, 0.1);
    rect(
        commands,
        Vec2::new(BM_OFFSET_X, BM_OFFSET_Y + BM_PLAY_H * 0.5 + frame * 0.5),
        Vec2::new(outer_w, frame),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(BM_OFFSET_X, BM_OFFSET_Y - BM_PLAY_H * 0.5 - frame * 0.5),
        Vec2::new(outer_w, frame),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(BM_OFFSET_X - BM_PLAY_W * 0.5 - frame * 0.5, BM_OFFSET_Y),
        Vec2::new(frame, outer_h),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(BM_OFFSET_X + BM_PLAY_W * 0.5 + frame * 0.5, BM_OFFSET_Y),
        Vec2::new(frame, outer_h),
        frame_color,
        GameEntity,
    );
}

/// 按 SpriteDef 给父实体挂子块（与离线预览同源）。
fn bm_def_children(commands: &mut Commands, parent: Entity, def: &SpriteDef) {
    for p in def.parts {
        commands
            .spawn((
                Sprite::from_color(p.color, Vec2::new(p.w, p.h)),
                Transform::from_translation(Vec3::new(p.dx, p.dy, p.dz)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

fn enemy_def(kind: EnemyKind) -> &'static SpriteDef {
    match kind {
        EnemyKind::Balloom => &ENEMY_BALLOOM,
        EnemyKind::Oneal => &ENEMY_ONEAL,
        EnemyKind::Doll => &ENEMY_DOLL,
        EnemyKind::Kondoria => &ENEMY_KONDORIA,
    }
}

fn spawn_hard_wall(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_TILE)),
            Transform::from_translation(pos.extend(Z_TILE)),
            BMHardWall,
            BMTilePos { col, row },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, &HARD_WALL);
}

fn spawn_soft_wall(commands: &mut Commands, col: i32, row: i32, hides: HiddenItem) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_TILE - 1.0)),
            Transform::from_translation(pos.extend(Z_TILE)),
            BMSoftWall { hides },
            BMTilePos { col, row },
            Collider {
                size: Vec2::splat(BM_TILE - 2.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, &SOFT_WALL);
}

pub fn spawn_bm_player(commands: &mut Commands, id: usize, spawn: (i32, i32)) {
    let pos = tile_center(spawn.0, spawn.1);
    let def = if id == 0 { &BM_PLAYER1 } else { &BM_PLAYER2 };
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_PLAYER_SIZE)),
            Transform::from_translation(pos.extend(Z_ACTOR)),
            BMPlayer::new(id),
            Collider {
                size: Vec2::splat(BM_PLAYER_SIZE - 4.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, def);
}

fn spawn_bm_enemy(commands: &mut Commands, col: i32, row: i32, kind: EnemyKind) {
    let pos = tile_center(col, row);
    let mut rng = thread_rng();
    let dir = *Dir4::all().choose(&mut rng).unwrap();
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_ENEMY_SIZE)),
            Transform::from_translation(pos.extend(Z_ACTOR - 0.05)),
            BMEnemy {
                kind,
                dir,
                change_timer: rng.gen_range(0.8..2.4),
            },
            Collider {
                size: Vec2::splat(BM_ENEMY_SIZE - 4.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, enemy_def(kind));
}

pub fn spawn_bm_bomb(
    commands: &mut Commands,
    col: i32,
    row: i32,
    range: i32,
    owner: Option<Entity>,
    remote: bool,
) -> Entity {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_BOMB_SIZE)),
            Transform::from_translation(pos.extend(Z_BOMB)),
            BMBomb {
                fuse: Timer::new(
                    Duration::from_millis(if remote {
                        60_000
                    } else {
                        (BM_BOMB_FUSE * 1000.0) as u64
                    }),
                    TimerMode::Once,
                ),
                range,
                owner,
                remote,
                triggered: false,
            },
            BMTilePos { col, row },
            Collider {
                size: Vec2::splat(BM_BOMB_SIZE - 4.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, &BOMB_DEF);
    parent
}

pub fn spawn_bm_flame(commands: &mut Commands, pos: Vec2, _color: Color) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_FLAME_SIZE)),
            Transform::from_translation(pos.extend(Z_FLAME)),
            BMFlame {
                timer: Timer::new(Duration::from_secs_f32(BM_FLAME_LIFE), TimerMode::Once),
            },
            Collider {
                size: Vec2::splat(BM_FLAME_SIZE - 6.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, &FLAME_DEF);
}

pub fn spawn_bm_powerup(
    commands: &mut Commands,
    col: i32,
    row: i32,
    kind: PowerupKind,
    font: &UiFont,
) {
    let pos = tile_center(col, row);
    let bg = kind.color();
    commands.spawn((
        Sprite::from_color(bg, Vec2::splat(BM_TILE - 8.0)),
        Transform::from_translation(pos.extend(Z_ITEM)),
        BMPowerup { kind },
        BMTilePos { col, row },
        Collider {
            size: Vec2::splat(BM_TILE - 8.0),
        },
        GameEntity,
    ));
    // 内框
    commands.spawn((
        Sprite::from_color(Color::srgb(0.1, 0.1, 0.12), Vec2::splat(BM_TILE - 14.0)),
        Transform::from_translation(pos.extend(Z_ITEM + 0.05)),
        GameEntity,
    ));
    // 标签字
    text(commands, font, kind.label(), pos, 16.0, bg, GameEntity)
        .insert(Transform::from_translation(pos.extend(Z_ITEM + 0.1)));
}

pub fn spawn_bm_exit(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(BM_TILE - 6.0)),
            Transform::from_translation(pos.extend(Z_ITEM)),
            BMExit,
            BMTilePos { col, row },
            Collider {
                size: Vec2::splat(BM_TILE - 8.0),
            },
            GameEntity,
        ))
        .id();
    bm_def_children(commands, parent, &EXIT_DEF);
}

// ========== HUD ==========
fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    let hud_x = BM_OFFSET_X + BM_PLAY_W * 0.5 + 95.0;
    let top_y = BM_OFFSET_Y + BM_PLAY_H * 0.5;

    text(
        commands,
        font,
        "炸弹迷宫",
        Vec2::new(hud_x, top_y - 18.0),
        20.0,
        Color::srgb(0.95, 0.78, 0.32),
        GameEntity,
    );
    text(
        commands,
        font,
        "STAGE",
        Vec2::new(hud_x, top_y - 50.0),
        14.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        &format!("{}", level),
        Vec2::new(hud_x, top_y - 72.0),
        24.0,
        Color::srgb(1.0, 0.85, 0.3),
        GameEntity,
    )
    .insert(BMHud::Stage);

    text(
        commands,
        font,
        "时间",
        Vec2::new(hud_x, top_y - 110.0),
        14.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        "200",
        Vec2::new(hud_x, top_y - 132.0),
        20.0,
        Color::srgb(0.45, 0.85, 1.0),
        GameEntity,
    )
    .insert(BMHud::Time);

    text(
        commands,
        font,
        "得分",
        Vec2::new(hud_x, top_y - 168.0),
        14.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        "0",
        Vec2::new(hud_x, top_y - 190.0),
        20.0,
        Color::srgb(0.95, 0.6, 0.4),
        GameEntity,
    )
    .insert(BMHud::Score);

    text(
        commands,
        font,
        "敌人",
        Vec2::new(hud_x, top_y - 226.0),
        14.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        "-",
        Vec2::new(hud_x, top_y - 248.0),
        20.0,
        Color::srgb(0.95, 0.4, 0.4),
        GameEntity,
    )
    .insert(BMHud::Enemies);

    // P1 / P2 命数
    text(
        commands,
        font,
        "P1",
        Vec2::new(hud_x - 22.0, top_y - 290.0),
        14.0,
        Color::srgb(0.4, 0.66, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        "x3",
        Vec2::new(hud_x - 22.0, top_y - 312.0),
        18.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    )
    .insert(BMHud::P1Lives);
    text(
        commands,
        font,
        "P2",
        Vec2::new(hud_x + 22.0, top_y - 290.0),
        14.0,
        Color::srgb(0.96, 0.45, 0.45),
        GameEntity,
    );
    text(
        commands,
        font,
        "x3",
        Vec2::new(hud_x + 22.0, top_y - 312.0),
        18.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    )
    .insert(BMHud::P2Lives);

    text(
        commands,
        font,
        "炸开软砖，找到出口逃出迷宫！",
        Vec2::new(0.0, BM_OFFSET_Y - BM_PLAY_H * 0.5 - 24.0),
        16.0,
        Color::srgb(0.85, 0.92, 1.0),
        GameEntity,
    )
    .insert(BMHud::Status);
}
