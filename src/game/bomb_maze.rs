use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::input::input_for;
use crate::common::render::{UiFont, rect, text};

use super::model::*;

// ========== 常量 ==========
pub(super) const BM_COLS: i32 = 15;
pub(super) const BM_ROWS: i32 = 11;
pub(super) const BM_TILE: f32 = 36.0;
pub(super) const BM_OFFSET_X: f32 = -100.0;
pub(super) const BM_OFFSET_Y: f32 = -10.0;
pub(super) const BM_PLAY_W: f32 = BM_COLS as f32 * BM_TILE;
pub(super) const BM_PLAY_H: f32 = BM_ROWS as f32 * BM_TILE;

const BM_PLAYER_SIZE: f32 = 24.0;
const BM_ENEMY_SIZE: f32 = 24.0;
const BM_BOMB_SIZE: f32 = 28.0;
const BM_FLAME_SIZE: f32 = 32.0;
const BM_PLAYER_SPEED_BASE: f32 = 100.0;
const BM_PLAYER_SPEED_BONUS: f32 = 22.0;
const BM_BOMB_FUSE: f32 = 2.6;
const BM_FLAME_LIFE: f32 = 0.55;
const BM_PLACE_CD: f32 = 0.16;
const BM_RESPAWN_TIME: f32 = 1.4;
const BM_INVULN_TIME: f32 = 1.6;

const Z_FLOOR: f32 = -1.0;
const Z_GRID: f32 = -0.8;
const Z_TILE: f32 = 0.5;
const Z_ITEM: f32 = 0.4;
const Z_BOMB: f32 = 1.0;
const Z_ACTOR: f32 = 1.2;
const Z_FLAME: f32 = 1.6;

const P1_SPAWN: (i32, i32) = (1, 1);
const P2_SPAWN: (i32, i32) = (1, 9);

// 软砖生成概率（除安全区）
const SOFT_WALL_DENSITY: f32 = 0.62;

// ========== 方向 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Dir4 {
    fn vec(self) -> Vec2 {
        match self {
            Dir4::Up => Vec2::new(0.0, 1.0),
            Dir4::Down => Vec2::new(0.0, -1.0),
            Dir4::Left => Vec2::new(-1.0, 0.0),
            Dir4::Right => Vec2::new(1.0, 0.0),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Dir4::Up => Dir4::Down,
            Dir4::Down => Dir4::Up,
            Dir4::Left => Dir4::Right,
            Dir4::Right => Dir4::Left,
        }
    }

    fn all() -> [Dir4; 4] {
        [Dir4::Up, Dir4::Down, Dir4::Left, Dir4::Right]
    }
}

// ========== 道具 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) enum PowerupKind {
    Fire,
    Bomb,
    Speed,
    BombPass,
    Detonator,
}

impl PowerupKind {
    fn color(self) -> Color {
        match self {
            PowerupKind::Fire => Color::srgb(1.0, 0.55, 0.18),
            PowerupKind::Bomb => Color::srgb(0.95, 0.95, 0.95),
            PowerupKind::Speed => Color::srgb(0.4, 0.85, 1.0),
            PowerupKind::BombPass => Color::srgb(0.8, 0.5, 1.0),
            PowerupKind::Detonator => Color::srgb(1.0, 0.3, 0.3),
        }
    }

    fn label(self) -> &'static str {
        match self {
            PowerupKind::Fire => "火",
            PowerupKind::Bomb => "弹",
            PowerupKind::Speed => "速",
            PowerupKind::BombPass => "穿",
            PowerupKind::Detonator => "控",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HiddenItem {
    Nothing,
    Powerup(PowerupKind),
    Exit,
}

// ========== 敌人 ==========
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum EnemyKind {
    Balloom,    // 慢，纯随机
    Oneal,      // 中，偶尔追踪
    Doll,       // 快，快速换向
    Kondoria,   // 慢但能穿软砖
}

impl EnemyKind {
    fn body_color(self) -> Color {
        match self {
            EnemyKind::Balloom => Color::srgb(0.85, 0.45, 0.85),
            EnemyKind::Oneal => Color::srgb(0.95, 0.45, 0.32),
            EnemyKind::Doll => Color::srgb(0.45, 0.85, 0.45),
            EnemyKind::Kondoria => Color::srgb(0.55, 0.45, 0.78),
        }
    }

    fn eye_color(self) -> Color {
        Color::srgb(1.0, 1.0, 1.0)
    }

    fn speed(self) -> f32 {
        match self {
            EnemyKind::Balloom => 50.0,
            EnemyKind::Oneal => 70.0,
            EnemyKind::Doll => 95.0,
            EnemyKind::Kondoria => 45.0,
        }
    }

    fn score(self) -> u32 {
        match self {
            EnemyKind::Balloom => 100,
            EnemyKind::Oneal => 200,
            EnemyKind::Doll => 400,
            EnemyKind::Kondoria => 1000,
        }
    }

    fn hunt(self) -> f32 {
        match self {
            EnemyKind::Balloom => 0.0,
            EnemyKind::Oneal => 0.35,
            EnemyKind::Doll => 0.45,
            EnemyKind::Kondoria => 0.2,
        }
    }

    fn wall_pass(self) -> bool {
        matches!(self, EnemyKind::Kondoria)
    }
}

// ========== 组件 ==========
#[derive(Component)]
pub(super) struct BMPlayer {
    pub id: usize,
    pub speed_level: u8,
    pub max_bombs: u8,
    pub bombs_alive: u8,
    pub bomb_range: i32,
    pub bomb_pass: bool,
    pub remote: bool,
    pub place_cd: f32,
    pub detonate_cd: f32,
    pub walking_off: Vec<Entity>,
    pub invuln: f32,
}

impl BMPlayer {
    fn new(id: usize) -> Self {
        Self {
            id,
            speed_level: 0,
            max_bombs: 1,
            bombs_alive: 0,
            bomb_range: 1,
            bomb_pass: false,
            remote: false,
            place_cd: 0.0,
            detonate_cd: 0.0,
            walking_off: Vec::new(),
            invuln: BM_INVULN_TIME,
        }
    }

    fn speed(&self) -> f32 {
        BM_PLAYER_SPEED_BASE + self.speed_level as f32 * BM_PLAYER_SPEED_BONUS
    }
}

#[derive(Component)]
pub(super) struct BMBomb {
    pub fuse: Timer,
    pub range: i32,
    pub owner: Option<Entity>,
    pub remote: bool,
    pub triggered: bool,
}

#[derive(Component)]
pub(super) struct BMFlame {
    pub timer: Timer,
}

#[derive(Component)]
pub(super) struct BMHardWall;

#[derive(Component)]
pub(super) struct BMSoftWall {
    hides: HiddenItem,
}

#[derive(Component)]
pub(super) struct BMPowerup {
    pub kind: PowerupKind,
}

#[derive(Component)]
pub(super) struct BMExit;

#[derive(Component)]
pub(super) struct BMEnemy {
    pub kind: EnemyKind,
    pub dir: Dir4,
    pub change_timer: f32,
}

#[derive(Component)]
pub(super) struct BMTilePos {
    pub col: i32,
    pub row: i32,
}

#[derive(Component, Clone, Copy)]
pub(super) enum BMHud {
    Stage,
    Time,
    Score,
    P1Lives,
    P2Lives,
    Enemies,
    Status,
}

// ========== 资源 ==========
#[derive(Resource)]
pub(super) struct BMStage {
    pub level: u8,
    pub time_left: f32,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub all_enemies_dead_msg_shown: bool,
    pub status: String,
}

// ========== 网格坐标 ==========
fn tile_center(col: i32, row: i32) -> Vec2 {
    Vec2::new(
        BM_OFFSET_X - BM_PLAY_W * 0.5 + col as f32 * BM_TILE + BM_TILE * 0.5,
        BM_OFFSET_Y + BM_PLAY_H * 0.5 - row as f32 * BM_TILE - BM_TILE * 0.5,
    )
}

fn world_to_tile(pos: Vec2) -> (i32, i32) {
    let local_x = pos.x - (BM_OFFSET_X - BM_PLAY_W * 0.5);
    let local_y = (BM_OFFSET_Y + BM_PLAY_H * 0.5) - pos.y;
    let col = (local_x / BM_TILE).floor() as i32;
    let row = (local_y / BM_TILE).floor() as i32;
    (col.clamp(0, BM_COLS - 1), row.clamp(0, BM_ROWS - 1))
}

fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

fn in_safe_zone(col: i32, row: i32) -> bool {
    // 玩家出生附近 L 形 3 格 留空，避免一开局被困
    let zones = [
        (P1_SPAWN.0, P1_SPAWN.1),
        (P1_SPAWN.0 + 1, P1_SPAWN.1),
        (P1_SPAWN.0, P1_SPAWN.1 + 1),
        (P2_SPAWN.0, P2_SPAWN.1),
        (P2_SPAWN.0 + 1, P2_SPAWN.1),
        (P2_SPAWN.0, P2_SPAWN.1 - 1),
    ];
    zones.iter().any(|(c, r)| *c == col && *r == row)
}

fn is_pillar(col: i32, row: i32) -> bool {
    col % 2 == 0 && row % 2 == 0
}

fn is_border(col: i32, row: i32) -> bool {
    col == 0 || col == BM_COLS - 1 || row == 0 || row == BM_ROWS - 1
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    paint_field(commands);

    let mut rng = thread_rng();

    // 生成软砖坐标
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
    soft_cells.shuffle(&mut rng);

    // 抽取一格做出口
    let exit_idx = if !soft_cells.is_empty() { Some(0_usize) } else { None };
    // 抽取 N 格藏道具
    let powerup_count = (3 + (level / 2) as usize).min(soft_cells.len().saturating_sub(1));
    let powerup_kinds = pick_powerup_kinds(level, powerup_count);

    let mut hidden_map: std::collections::HashMap<(i32, i32), HiddenItem> =
        std::collections::HashMap::new();
    if let Some(idx) = exit_idx {
        hidden_map.insert(soft_cells[idx], HiddenItem::Exit);
    }
    for (i, kind) in powerup_kinds.iter().enumerate() {
        let cell = soft_cells[exit_idx.map(|e| e + 1 + i).unwrap_or(i)];
        hidden_map.insert(cell, HiddenItem::Powerup(*kind));
    }

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

    // HUD（右侧）
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

fn spawn_hard_wall(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    // 主体灰
    commands.spawn((
        Sprite::from_color(Color::srgb(0.55, 0.58, 0.62), Vec2::splat(BM_TILE)),
        Transform::from_translation(pos.extend(Z_TILE)),
        BMHardWall,
        BMTilePos { col, row },
        GameEntity,
    ));
    // 高光（左上）
    commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.82, 0.88), Vec2::new(BM_TILE - 4.0, 5.0)),
        Transform::from_translation((pos + Vec2::new(0.0, BM_TILE * 0.5 - 4.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.82, 0.88), Vec2::new(5.0, BM_TILE - 4.0)),
        Transform::from_translation((pos + Vec2::new(-BM_TILE * 0.5 + 4.0, 0.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    // 阴影（右下）
    commands.spawn((
        Sprite::from_color(Color::srgb(0.30, 0.32, 0.36), Vec2::new(BM_TILE - 4.0, 4.0)),
        Transform::from_translation((pos + Vec2::new(0.0, -BM_TILE * 0.5 + 3.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.30, 0.32, 0.36), Vec2::new(4.0, BM_TILE - 4.0)),
        Transform::from_translation((pos + Vec2::new(BM_TILE * 0.5 - 3.0, 0.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

fn spawn_soft_wall(commands: &mut Commands, col: i32, row: i32, hides: HiddenItem) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.74, 0.42, 0.18), Vec2::splat(BM_TILE - 1.0)),
        Transform::from_translation(pos.extend(Z_TILE)),
        BMSoftWall { hides },
        BMTilePos { col, row },
        Collider {
            size: Vec2::splat(BM_TILE - 2.0),
        },
        GameEntity,
    ));
    // 砖缝
    let line = Color::srgb(0.4, 0.22, 0.08);
    commands.spawn((
        Sprite::from_color(line, Vec2::new(BM_TILE - 4.0, 1.5)),
        Transform::from_translation((pos + Vec2::new(0.0, BM_TILE * 0.25)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(line, Vec2::new(BM_TILE - 4.0, 1.5)),
        Transform::from_translation((pos + Vec2::new(0.0, -BM_TILE * 0.25)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(line, Vec2::new(1.5, BM_TILE * 0.5)),
        Transform::from_translation((pos + Vec2::new(BM_TILE * 0.25, 0.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(line, Vec2::new(1.5, BM_TILE * 0.5)),
        Transform::from_translation((pos + Vec2::new(-BM_TILE * 0.25, 0.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

fn spawn_bm_player(commands: &mut Commands, id: usize, spawn: (i32, i32)) {
    let pos = tile_center(spawn.0, spawn.1);
    let body_color = if id == 0 {
        Color::srgb(0.95, 0.95, 0.98)
    } else {
        Color::srgb(0.95, 0.95, 0.98)
    };
    let head_color = if id == 0 {
        Color::srgb(0.32, 0.55, 0.95)
    } else {
        Color::srgb(0.95, 0.42, 0.42)
    };

    commands
        .spawn((
            Sprite::from_color(body_color, Vec2::splat(BM_PLAYER_SIZE)),
            Transform::from_translation(pos.extend(Z_ACTOR)),
            BMPlayer::new(id),
            Collider {
                size: Vec2::splat(BM_PLAYER_SIZE - 4.0),
            },
            GameEntity,
        ))
        .with_children(|p| {
            // 头盔 / 帽子
            p.spawn((
                Sprite::from_color(head_color, Vec2::new(BM_PLAYER_SIZE - 4.0, 8.0)),
                Transform::from_translation(Vec3::new(0.0, 7.0, 0.05)),
            ));
            // 面罩（黑色横条）
            p.spawn((
                Sprite::from_color(Color::srgb(0.2, 0.2, 0.2), Vec2::new(BM_PLAYER_SIZE - 8.0, 3.0)),
                Transform::from_translation(Vec3::new(0.0, 3.0, 0.06)),
            ));
            // 腰带
            p.spawn((
                Sprite::from_color(head_color, Vec2::new(BM_PLAYER_SIZE - 4.0, 3.0)),
                Transform::from_translation(Vec3::new(0.0, -4.0, 0.05)),
            ));
            // 双脚
            p.spawn((
                Sprite::from_color(Color::srgb(0.25, 0.25, 0.3), Vec2::new(7.0, 5.0)),
                Transform::from_translation(Vec3::new(-5.0, -BM_PLAYER_SIZE * 0.5 + 2.0, 0.05)),
            ));
            p.spawn((
                Sprite::from_color(Color::srgb(0.25, 0.25, 0.3), Vec2::new(7.0, 5.0)),
                Transform::from_translation(Vec3::new(5.0, -BM_PLAYER_SIZE * 0.5 + 2.0, 0.05)),
            ));
        });
}

fn spawn_bm_enemy(commands: &mut Commands, col: i32, row: i32, kind: EnemyKind) {
    let pos = tile_center(col, row);
    let body = kind.body_color();
    let mut rng = thread_rng();
    let dir = *Dir4::all().choose(&mut rng).unwrap();

    commands
        .spawn((
            Sprite::from_color(body, Vec2::splat(BM_ENEMY_SIZE)),
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
        .with_children(|p| {
            // 双眼
            p.spawn((
                Sprite::from_color(kind.eye_color(), Vec2::splat(5.0)),
                Transform::from_translation(Vec3::new(-5.0, 4.0, 0.05)),
            ));
            p.spawn((
                Sprite::from_color(kind.eye_color(), Vec2::splat(5.0)),
                Transform::from_translation(Vec3::new(5.0, 4.0, 0.05)),
            ));
            // 瞳孔
            p.spawn((
                Sprite::from_color(Color::srgb(0.05, 0.05, 0.05), Vec2::splat(2.0)),
                Transform::from_translation(Vec3::new(-5.0, 4.0, 0.06)),
            ));
            p.spawn((
                Sprite::from_color(Color::srgb(0.05, 0.05, 0.05), Vec2::splat(2.0)),
                Transform::from_translation(Vec3::new(5.0, 4.0, 0.06)),
            ));
        });
}

fn spawn_bm_bomb(
    commands: &mut Commands,
    col: i32,
    row: i32,
    range: i32,
    owner: Option<Entity>,
    remote: bool,
) -> Entity {
    let pos = tile_center(col, row);
    let id = commands
        .spawn((
            Sprite::from_color(Color::srgb(0.08, 0.08, 0.1), Vec2::splat(BM_BOMB_SIZE)),
            Transform::from_translation(pos.extend(Z_BOMB)),
            BMBomb {
                fuse: Timer::new(
                    Duration::from_millis(if remote { 60_000 } else { (BM_BOMB_FUSE * 1000.0) as u64 }),
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
        .with_children(|p| {
            // 高光
            p.spawn((
                Sprite::from_color(Color::srgb(0.32, 0.32, 0.36), Vec2::splat(8.0)),
                Transform::from_translation(Vec3::new(-5.0, 6.0, 0.05)),
            ));
            // 引线
            p.spawn((
                Sprite::from_color(Color::srgb(0.5, 0.36, 0.18), Vec2::new(2.0, 6.0)),
                Transform::from_translation(Vec3::new(0.0, BM_BOMB_SIZE * 0.5 + 1.0, 0.05)),
            ));
            // 火花
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.86, 0.32), Vec2::splat(4.0)),
                Transform::from_translation(Vec3::new(0.0, BM_BOMB_SIZE * 0.5 + 5.0, 0.06)),
            ));
        })
        .id();
    id
}

fn spawn_bm_flame(commands: &mut Commands, pos: Vec2, color: Color) {
    commands
        .spawn((
            Sprite::from_color(color, Vec2::splat(BM_FLAME_SIZE)),
            Transform::from_translation(pos.extend(Z_FLAME)),
            BMFlame {
                timer: Timer::new(Duration::from_secs_f32(BM_FLAME_LIFE), TimerMode::Once),
            },
            Collider {
                size: Vec2::splat(BM_FLAME_SIZE - 6.0),
            },
            GameEntity,
        ))
        .with_children(|p| {
            // 内核
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.95, 0.62), Vec2::splat(BM_FLAME_SIZE - 12.0)),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)),
            ));
            // 中心点
            p.spawn((
                Sprite::from_color(Color::srgb(1.0, 1.0, 0.9), Vec2::splat(BM_FLAME_SIZE - 22.0)),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.06)),
            ));
        });
}

fn spawn_bm_powerup(commands: &mut Commands, col: i32, row: i32, kind: PowerupKind, font: &UiFont) {
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

fn spawn_bm_exit(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.1, 0.22), Vec2::splat(BM_TILE - 6.0)),
        Transform::from_translation(pos.extend(Z_ITEM)),
        BMExit,
        BMTilePos { col, row },
        Collider {
            size: Vec2::splat(BM_TILE - 8.0),
        },
        GameEntity,
    ));
    // 门廊
    commands.spawn((
        Sprite::from_color(Color::srgb(0.62, 0.45, 0.86), Vec2::new(BM_TILE - 14.0, BM_TILE - 14.0)),
        Transform::from_translation(pos.extend(Z_ITEM + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.1, 0.22), Vec2::new(BM_TILE - 18.0, BM_TILE - 18.0)),
        Transform::from_translation(pos.extend(Z_ITEM + 0.1)),
        GameEntity,
    ));
    // 上箭头
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.95, 0.6), Vec2::new(3.0, 10.0)),
        Transform::from_translation((pos + Vec2::new(0.0, 0.0)).extend(Z_ITEM + 0.15)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.95, 0.6), Vec2::new(8.0, 3.0)),
        Transform::from_translation((pos + Vec2::new(0.0, 4.0)).extend(Z_ITEM + 0.15)),
        GameEntity,
    ));
}

// ========== HUD ==========
fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    let hud_x = BM_OFFSET_X + BM_PLAY_W * 0.5 + 95.0;
    let top_y = BM_OFFSET_Y + BM_PLAY_H * 0.5;

    // 标题
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

    // 下方状态条
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

// ========== 系统：玩家输入 + 移动 + 放弹 ==========
pub(super) fn bm_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<BMStage>,
    mut players: Query<(Entity, &mut Transform, &mut BMPlayer)>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<&BMTilePos, With<BMSoftWall>>,
    mut bombs: Query<(Entity, &mut BMBomb, &BMTilePos)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    stage.time_left = (stage.time_left - delta).max(0.0);
    if stage.p1_respawn > 0.0 {
        stage.p1_respawn = (stage.p1_respawn - delta).max(0.0);
    }
    if stage.p2_respawn > 0.0 {
        stage.p2_respawn = (stage.p2_respawn - delta).max(0.0);
    }

    // 网格映射
    let mut blocked = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            blocked[tp.col as usize][tp.row as usize] = true;
        }
    }
    for tp in &soft_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            blocked[tp.col as usize][tp.row as usize] = true;
        }
    }
    // (Entity, col, row, owner, remote)
    let bomb_meta: Vec<(Entity, i32, i32, Option<Entity>, bool)> = bombs
        .iter()
        .map(|(e, b, tp)| (e, tp.col, tp.row, b.owner, b.remote))
        .collect();
    let bomb_tiles: Vec<(Entity, i32, i32)> = bomb_meta
        .iter()
        .map(|(e, c, r, _, _)| (*e, *c, *r))
        .collect();

    // 收集本帧已存在的炸弹格子（用于禁止重复放置；同帧多人也会随之更新）
    let mut occupied_bomb_cells: std::collections::HashSet<(i32, i32)> =
        bomb_tiles.iter().map(|(_, c, r)| (*c, *r)).collect();

    let mut place_requests: Vec<(Entity, i32, i32, i32, bool)> = Vec::new();
    let mut detonate_requests: Vec<Entity> = Vec::new();

    for (entity, mut tf, mut player) in &mut players {
        player.place_cd = (player.place_cd - delta).max(0.0);
        player.detonate_cd = (player.detonate_cd - delta).max(0.0);
        if player.invuln > 0.0 {
            player.invuln = (player.invuln - delta).max(0.0);
        }

        let input = input_for(&keys, player.id);
        let mut dir = input.move_dir;
        if dir.length_squared() > 1.01 {
            dir = dir.normalize();
        }

        let speed = player.speed();
        let pos = tf.translation.truncate();

        let target_pos = move_with_collision(
            pos,
            dir,
            speed,
            delta,
            &blocked,
            &bomb_tiles,
            entity,
            &player.walking_off,
            player.bomb_pass,
        );

        tf.translation.x = target_pos.x;
        tf.translation.y = target_pos.y;

        // 离开自己放下的炸弹后，从豁免列表移除：
        // 用 bbox 重叠判断，避免玩家走到 tile 边界时（中心刚跨格、bbox 还压在炸弹上）
        // 因 walking_off 立即清空被自家炸弹卡住。
        if !player.walking_off.is_empty() {
            player.walking_off.retain(|bomb_e| {
                bomb_tiles.iter().any(|(e, c, r)| {
                    if *e != *bomb_e {
                        return false;
                    }
                    let cell_pos = tile_center(*c, *r);
                    aabb_overlap(
                        target_pos,
                        Vec2::splat(BM_PLAYER_SIZE - 4.0),
                        cell_pos,
                        Vec2::splat(BM_BOMB_SIZE - 6.0),
                    )
                })
            });
        }

        // 放炸弹
        if input.fire && player.place_cd <= 0.0 && player.bombs_alive < player.max_bombs {
            let (pc, pr) = world_to_tile(target_pos);
            if !occupied_bomb_cells.contains(&(pc, pr)) && !blocked[pc as usize][pr as usize] {
                place_requests.push((entity, pc, pr, player.bomb_range, player.remote));
                occupied_bomb_cells.insert((pc, pr));
                player.place_cd = BM_PLACE_CD;
            }
        }

        // 遥控引爆：只能引爆自己拥有的、最早放下的一颗遥控炸弹
        if input.jump && player.remote && player.detonate_cd <= 0.0 {
            if let Some((bomb_e, _, _, _, _)) = bomb_meta
                .iter()
                .find(|(_, _, _, owner, remote)| *remote && *owner == Some(entity))
            {
                detonate_requests.push(*bomb_e);
                player.detonate_cd = 0.18;
            }
        }
    }

    // 实际放置炸弹
    for (owner, c, r, range, remote) in place_requests {
        let bomb = spawn_bm_bomb(&mut commands, c, r, range, Some(owner), remote);
        if let Ok((_e, _, mut p)) = players.get_mut(owner) {
            p.bombs_alive += 1;
            p.walking_off.push(bomb);
        }
    }
    // 远程引爆：直接把对应炸弹标记为已触发
    for e in detonate_requests {
        if let Ok((_, mut bomb, _)) = bombs.get_mut(e) {
            bomb.triggered = true;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn move_with_collision(
    pos: Vec2,
    dir: Vec2,
    speed: f32,
    delta: f32,
    blocked: &[Vec<bool>],
    bombs: &[(Entity, i32, i32)],
    self_entity: Entity,
    walking_off: &[Entity],
    bomb_pass: bool,
) -> Vec2 {
    let half = (BM_PLAYER_SIZE - 4.0) * 0.5;
    let pmin = Vec2::new(
        BM_OFFSET_X - BM_PLAY_W * 0.5 + half + 1.0,
        BM_OFFSET_Y - BM_PLAY_H * 0.5 + half + 1.0,
    );
    let pmax = Vec2::new(
        BM_OFFSET_X + BM_PLAY_W * 0.5 - half - 1.0,
        BM_OFFSET_Y + BM_PLAY_H * 0.5 - half - 1.0,
    );

    let collide = |p: Vec2| -> bool {
        let half_v = Vec2::splat(half);
        // 网格阻挡：用四角分别求所属 tile，再对网格范围取 min/max
        let corners = [
            p + Vec2::new(-half_v.x, -half_v.y),
            p + Vec2::new(half_v.x, -half_v.y),
            p + Vec2::new(-half_v.x, half_v.y),
            p + Vec2::new(half_v.x, half_v.y),
        ];
        let mut cmin = i32::MAX;
        let mut cmax = i32::MIN;
        let mut rmin = i32::MAX;
        let mut rmax = i32::MIN;
        for cp in corners {
            let (cc, rr) = world_to_tile(cp);
            cmin = cmin.min(cc);
            cmax = cmax.max(cc);
            rmin = rmin.min(rr);
            rmax = rmax.max(rr);
        }
        for c in cmin..=cmax {
            for r in rmin..=rmax {
                if !(0..BM_COLS).contains(&c) || !(0..BM_ROWS).contains(&r) {
                    return true;
                }
                if blocked[c as usize][r as usize] {
                    let cell_pos = tile_center(c, r);
                    if aabb_overlap(p, Vec2::splat(half * 2.0), cell_pos, Vec2::splat(BM_TILE - 1.0)) {
                        return true;
                    }
                }
            }
        }
        // 炸弹阻挡（除非穿弹/或正在豁免）
        if !bomb_pass {
            for (bomb_e, c, r) in bombs {
                if walking_off.contains(bomb_e) {
                    continue;
                }
                let _ = self_entity;
                let cell_pos = tile_center(*c, *r);
                if aabb_overlap(
                    p,
                    Vec2::splat(half * 2.0),
                    cell_pos,
                    Vec2::splat(BM_BOMB_SIZE - 6.0),
                ) {
                    return true;
                }
            }
        }
        false
    };

    let mut new_pos = pos;
    let dx = dir.x * speed * delta;
    let dy = dir.y * speed * delta;
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    let move_x = abs_dx > 1e-3;
    let move_y = abs_dy > 1e-3;
    let cardinal_x = move_x && !move_y;
    let cardinal_y = move_y && !move_x;
    let snap_step = speed * delta;

    // 计算朝当前 tile 中心吸附（仅单轴移动时使用），返回偏移量
    let snap_to_row_center = |p: Vec2| -> f32 {
        let (_, row) = world_to_tile(p);
        let row_y = tile_center(0, row).y;
        let diff = row_y - p.y;
        if diff.abs() < 0.5 {
            0.0
        } else if diff.abs() < snap_step {
            diff
        } else {
            diff.signum() * snap_step
        }
    };
    let snap_to_col_center = |p: Vec2| -> f32 {
        let (col, _) = world_to_tile(p);
        let col_x = tile_center(col, 0).x;
        let diff = col_x - p.x;
        if diff.abs() < 0.5 {
            0.0
        } else if diff.abs() < snap_step {
            diff
        } else {
            diff.signum() * snap_step
        }
    };

    // X 方向
    if move_x {
        let snap_y = if cardinal_x { snap_to_row_center(new_pos) } else { 0.0 };
        let target_x = (new_pos.x + dx).clamp(pmin.x, pmax.x);
        let try_combo = Vec2::new(target_x, (new_pos.y + snap_y).clamp(pmin.y, pmax.y));
        if !collide(try_combo) {
            new_pos = try_combo;
        } else {
            let try_x = Vec2::new(target_x, new_pos.y);
            if !collide(try_x) {
                new_pos = try_x;
            } else if cardinal_x {
                // 撞墙时尝试更大幅度的角落贴边，最大半个 tile
                let (_, row) = world_to_tile(new_pos);
                let row_y = tile_center(0, row).y;
                let diff = row_y - new_pos.y;
                if diff.abs() > 0.5 && diff.abs() < BM_TILE * 0.5 {
                    let step = diff.signum() * snap_step;
                    let nudged_y = if step.abs() > diff.abs() {
                        row_y
                    } else {
                        new_pos.y + step
                    };
                    let candidate = Vec2::new(target_x, nudged_y).clamp(pmin, pmax);
                    if !collide(candidate) {
                        new_pos = candidate;
                    }
                }
            }
        }
    }

    // Y 方向
    if move_y {
        let snap_x = if cardinal_y { snap_to_col_center(new_pos) } else { 0.0 };
        let target_y = (new_pos.y + dy).clamp(pmin.y, pmax.y);
        let try_combo = Vec2::new((new_pos.x + snap_x).clamp(pmin.x, pmax.x), target_y);
        if !collide(try_combo) {
            new_pos = try_combo;
        } else {
            let try_y = Vec2::new(new_pos.x, target_y);
            if !collide(try_y) {
                new_pos = try_y;
            } else if cardinal_y {
                let (col, _) = world_to_tile(new_pos);
                let col_x = tile_center(col, 0).x;
                let diff = col_x - new_pos.x;
                if diff.abs() > 0.5 && diff.abs() < BM_TILE * 0.5 {
                    let step = diff.signum() * snap_step;
                    let nudged_x = if step.abs() > diff.abs() {
                        col_x
                    } else {
                        new_pos.x + step
                    };
                    let candidate = Vec2::new(nudged_x, target_y).clamp(pmin, pmax);
                    if !collide(candidate) {
                        new_pos = candidate;
                    }
                }
            }
        }
    }

    new_pos.clamp(pmin, pmax)
}

// ========== 系统：炸弹倒计时 + 爆炸 ==========
pub(super) fn bm_bomb_tick(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bombs: Query<(Entity, &mut BMBomb, &BMTilePos), With<BMBomb>>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<(Entity, &BMTilePos, &BMSoftWall)>,
    mut players: Query<&mut BMPlayer>,
    font: Res<UiFont>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }

    // 网格信息
    let mut hard = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            hard[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut soft_at: std::collections::HashMap<(i32, i32), (Entity, HiddenItem)> =
        std::collections::HashMap::new();
    for (e, tp, sw) in &soft_walls {
        soft_at.insert((tp.col, tp.row), (e, sw.hides));
    }
    let mut bomb_at: std::collections::HashMap<(i32, i32), Entity> =
        std::collections::HashMap::new();
    for (e, _, tp) in bombs.iter() {
        bomb_at.insert((tp.col, tp.row), e);
    }

    // 第一步：tick 计时 / 标记将要爆炸的炸弹
    let mut to_explode: Vec<Entity> = Vec::new();
    for (entity, mut bomb, _) in &mut bombs {
        bomb.fuse.tick(time.delta());
        if bomb.triggered || (!bomb.remote && bomb.fuse.just_finished()) {
            to_explode.push(entity);
        }
    }
    if to_explode.is_empty() {
        return;
    }

    // BFS 处理连环引爆
    let mut queue: std::collections::VecDeque<Entity> = to_explode.iter().copied().collect();
    let mut exploded: std::collections::HashSet<Entity> = std::collections::HashSet::new();
    let mut flame_cells: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut destroyed_softs: std::collections::HashSet<Entity> = std::collections::HashSet::new();
    let mut bomb_owners: Vec<Option<Entity>> = Vec::new();

    while let Some(entity) = queue.pop_front() {
        if !exploded.insert(entity) {
            continue;
        }
        let Ok((_, bomb, tp)) = bombs.get(entity) else {
            continue;
        };
        bomb_owners.push(bomb.owner);
        let (cx, cy) = (tp.col, tp.row);
        flame_cells.insert((cx, cy));
        for d in Dir4::all() {
            let dv = d.vec();
            for step in 1..=bomb.range {
                let nc = cx + dv.x as i32 * step;
                let nr = cy + dv.y as i32 * step;
                if !(0..BM_COLS).contains(&nc) || !(0..BM_ROWS).contains(&nr) {
                    break;
                }
                if hard[nc as usize][nr as usize] {
                    break;
                }
                flame_cells.insert((nc, nr));
                if let Some((soft_e, _)) = soft_at.get(&(nc, nr)) {
                    destroyed_softs.insert(*soft_e);
                    break; // 软砖吸火焰一格
                }
                if let Some(bomb_e) = bomb_at.get(&(nc, nr)) {
                    if !exploded.contains(bomb_e) {
                        queue.push_back(*bomb_e);
                    }
                    // 火焰会穿过被引爆的炸弹？经典版会停在炸弹格，这里也停
                    break;
                }
            }
        }
    }

    // 还原炸弹计数 + despawn 已爆炸炸弹
    for owner in bomb_owners.into_iter().flatten() {
        if let Ok(mut p) = players.get_mut(owner) {
            p.bombs_alive = p.bombs_alive.saturating_sub(1);
            // 同步从 walking_off 中移除（万一玩家还没走出去）
            // 但这里没有 bomb entity 信息，先跳过；下面会在炸弹消失后由 bm_player_input 自动清理
        }
    }
    for e in &exploded {
        commands.entity(*e).despawn();
    }

    // 软砖被毁：露出底下藏的东西
    for soft_e in &destroyed_softs {
        // 查找它对应的 (col, row) 与 hides
        if let Some(((c, r), (_, hides))) = soft_at.iter().find(|(_, (e, _))| e == soft_e) {
            commands.entity(*soft_e).despawn();
            match hides {
                HiddenItem::Powerup(kind) => {
                    spawn_bm_powerup(&mut commands, *c, *r, *kind, &font);
                }
                HiddenItem::Exit => {
                    spawn_bm_exit(&mut commands, *c, *r);
                }
                HiddenItem::Nothing => {}
            }
        }
    }

    // 火焰
    for (c, r) in flame_cells {
        let pos = tile_center(c, r);
        spawn_bm_flame(&mut commands, pos, Color::srgb(1.0, 0.55, 0.18));
    }
}

// ========== 系统：火焰生命 ==========
pub(super) fn bm_flame_tick(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut flames: Query<(Entity, &mut BMFlame, &mut Sprite)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    for (entity, mut flame, mut sprite) in &mut flames {
        flame.timer.tick(time.delta());
        // 渐变颜色
        let t = flame.timer.fraction();
        let alpha = (1.0 - t).clamp(0.0, 1.0);
        let mut c = Color::srgba(1.0, 0.55, 0.18, alpha);
        if t > 0.5 {
            c = Color::srgba(1.0, 0.85, 0.32, alpha);
        }
        sprite.color = c;
        if flame.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ========== 系统：敌人 AI ==========
pub(super) fn bm_enemy_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut enemies: Query<(&mut Transform, &mut BMEnemy)>,
    hard_walls: Query<&BMTilePos, With<BMHardWall>>,
    soft_walls: Query<&BMTilePos, With<BMSoftWall>>,
    bombs: Query<&BMTilePos, With<BMBomb>>,
    players: Query<&Transform, (With<BMPlayer>, Without<BMEnemy>)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();

    let mut hard = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &hard_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            hard[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut soft = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &soft_walls {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            soft[tp.col as usize][tp.row as usize] = true;
        }
    }
    let mut bomb_grid = vec![vec![false; BM_ROWS as usize]; BM_COLS as usize];
    for tp in &bombs {
        if (0..BM_COLS).contains(&tp.col) && (0..BM_ROWS).contains(&tp.row) {
            bomb_grid[tp.col as usize][tp.row as usize] = true;
        }
    }

    let player_positions: Vec<Vec2> = players.iter().map(|t| t.translation.truncate()).collect();

    let mut rng = thread_rng();

    for (mut tf, mut enemy) in &mut enemies {
        let speed = enemy.kind.speed();
        let wall_pass = enemy.kind.wall_pass();
        let pos = tf.translation.truncate();
        let dir_v = enemy.dir.vec();
        let step = dir_v * speed * delta;
        let new_pos = pos + step;

        // 看下一步是否撞墙或将出格
        let half = (BM_ENEMY_SIZE - 4.0) * 0.5;
        let pmin = Vec2::new(
            BM_OFFSET_X - BM_PLAY_W * 0.5 + half + 1.0,
            BM_OFFSET_Y - BM_PLAY_H * 0.5 + half + 1.0,
        );
        let pmax = Vec2::new(
            BM_OFFSET_X + BM_PLAY_W * 0.5 - half - 1.0,
            BM_OFFSET_Y + BM_PLAY_H * 0.5 - half - 1.0,
        );
        let out = new_pos.x < pmin.x || new_pos.x > pmax.x || new_pos.y < pmin.y || new_pos.y > pmax.y;
        let blocked_at = |c: i32, r: i32| -> bool {
            if !(0..BM_COLS).contains(&c) || !(0..BM_ROWS).contains(&r) {
                return true;
            }
            if hard[c as usize][r as usize] {
                return true;
            }
            if !wall_pass && soft[c as usize][r as usize] {
                return true;
            }
            if bomb_grid[c as usize][r as usize] {
                return true;
            }
            false
        };

        // 用看前方的 tile 判断
        let look = pos + dir_v * (BM_ENEMY_SIZE * 0.5 + 6.0);
        let (lc, lr) = world_to_tile(look);
        let blocked_ahead = blocked_at(lc, lr);

        enemy.change_timer -= delta;

        if blocked_ahead || out || enemy.change_timer <= 0.0 {
            // 选新方向
            let mut candidates: Vec<Dir4> = Dir4::all().to_vec();
            candidates.shuffle(&mut rng);

            // 偶尔追玩家
            let hunt = enemy.kind.hunt();
            if !player_positions.is_empty() && rng.r#gen::<f32>() < hunt {
                let target = nearest_point(&player_positions, pos);
                let toward = (target - pos).normalize_or_zero();
                let mut towards: Vec<Dir4> = Vec::new();
                if toward.x.abs() > 0.1 {
                    towards.push(if toward.x > 0.0 {
                        Dir4::Right
                    } else {
                        Dir4::Left
                    });
                }
                if toward.y.abs() > 0.1 {
                    towards.push(if toward.y > 0.0 {
                        Dir4::Up
                    } else {
                        Dir4::Down
                    });
                }
                towards.shuffle(&mut rng);
                for c in towards.into_iter().rev() {
                    candidates.retain(|x| *x != c);
                    candidates.insert(0, c);
                }
            }

            let mut new_dir = enemy.dir;
            for c in &candidates {
                if blocked_ahead && *c == enemy.dir.opposite() && candidates.len() > 1 {
                    // 若被阻则尽量不走回头路；除非别无选择
                    continue;
                }
                let probe = pos + c.vec() * (BM_ENEMY_SIZE * 0.5 + 6.0);
                let (pc, pr) = world_to_tile(probe);
                if !blocked_at(pc, pr) {
                    new_dir = *c;
                    break;
                }
            }
            // 如果完全被困，允许调头
            if new_dir == enemy.dir && blocked_ahead {
                let probe = pos + enemy.dir.opposite().vec() * (BM_ENEMY_SIZE * 0.5 + 6.0);
                let (pc, pr) = world_to_tile(probe);
                if !blocked_at(pc, pr) {
                    new_dir = enemy.dir.opposite();
                }
            }
            enemy.dir = new_dir;
            enemy.change_timer = rng.gen_range(0.9..2.4);

            // 网格对齐：把垂直轴吸附到 tile 中心，避免卡墙
            snap_perpendicular(&mut tf.translation, enemy.dir);
        } else {
            tf.translation.x = new_pos.x;
            tf.translation.y = new_pos.y;
        }
    }
}

fn nearest_point(points: &[Vec2], from: Vec2) -> Vec2 {
    let mut best = points[0];
    let mut best_d = best.distance_squared(from);
    for p in &points[1..] {
        let d = p.distance_squared(from);
        if d < best_d {
            best_d = d;
            best = *p;
        }
    }
    best
}

fn snap_perpendicular(t: &mut Vec3, dir: Dir4) {
    match dir {
        Dir4::Up | Dir4::Down => {
            let pos = Vec2::new(t.x, t.y);
            let (col, _) = world_to_tile(pos);
            t.x = tile_center(col, 0).x;
        }
        Dir4::Left | Dir4::Right => {
            let pos = Vec2::new(t.x, t.y);
            let (_, row) = world_to_tile(pos);
            t.y = tile_center(0, row).y;
        }
    }
}

// ========== 系统：火焰造成伤害 ==========
pub(super) fn bm_flame_damage(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<BMStage>,
    flames: Query<(&Transform, &Collider), With<BMFlame>>,
    enemies: Query<(Entity, &Transform, &Collider, &BMEnemy)>,
    mut players: Query<(Entity, &Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let flame_data: Vec<(Vec2, Vec2)> = flames
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    if flame_data.is_empty() {
        return;
    }

    // 敌人
    for (enemy_e, et, ec, enemy) in &enemies {
        let ep = et.translation.truncate();
        for (fp, fs) in &flame_data {
            if aabb_overlap(ep, ec.size, *fp, *fs) {
                session.score += enemy.kind.score();
                commands.entity(enemy_e).despawn();
                break;
            }
        }
    }

    // 玩家
    for (player_e, pt, pc, mut player) in &mut players {
        if player.invuln > 0.0 {
            continue;
        }
        let pp = pt.translation.truncate();
        for (fp, fs) in &flame_data {
            if aabb_overlap(pp, pc.size, *fp, *fs) {
                hurt_player(
                    &mut commands,
                    &mut stage,
                    player_e,
                    &mut player,
                );
                break;
            }
        }
    }
}

fn hurt_player(
    commands: &mut Commands,
    stage: &mut BMStage,
    entity: Entity,
    player: &mut BMPlayer,
) {
    commands.entity(entity).despawn();
    if player.id == 0 {
        stage.p1_lives -= 1;
        if stage.p1_lives >= 0 {
            stage.p1_respawn = BM_RESPAWN_TIME;
        }
    } else {
        stage.p2_lives -= 1;
        if stage.p2_lives >= 0 {
            stage.p2_respawn = BM_RESPAWN_TIME;
        }
    }
}

// ========== 系统：敌人接触玩家 ==========
pub(super) fn bm_enemy_touch(
    mut commands: Commands,
    session: Res<GameSession>,
    mut stage: ResMut<BMStage>,
    enemies: Query<(&Transform, &Collider), With<BMEnemy>>,
    mut players: Query<(Entity, &Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let enemy_data: Vec<(Vec2, Vec2)> = enemies
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    for (player_e, pt, pc, mut player) in &mut players {
        if player.invuln > 0.0 {
            continue;
        }
        let pp = pt.translation.truncate();
        for (ep, es) in &enemy_data {
            if aabb_overlap(pp, pc.size, *ep, *es) {
                hurt_player(&mut commands, &mut stage, player_e, &mut player);
                break;
            }
        }
    }
}

// ========== 系统：道具拾取 ==========
pub(super) fn bm_powerup_pickup(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    powerups: Query<(Entity, &Transform, &Collider, &BMPowerup)>,
    mut players: Query<(&Transform, &Collider, &mut BMPlayer)>,
) {
    if session.kind != GameKind::BombMaze || session.paused || session.finished {
        return;
    }
    let mut consumed: Vec<Entity> = Vec::new();
    for (item_e, it, ic, powerup) in &powerups {
        let ip = it.translation.truncate();
        for (pt, pc, mut player) in &mut players {
            if aabb_overlap(ip, ic.size, pt.translation.truncate(), pc.size) {
                apply_powerup(&mut player, powerup.kind);
                session.score += 50;
                consumed.push(item_e);
                break;
            }
        }
    }
    for e in consumed {
        commands.entity(e).despawn();
    }
}

fn apply_powerup(player: &mut BMPlayer, kind: PowerupKind) {
    match kind {
        PowerupKind::Fire => {
            player.bomb_range = (player.bomb_range + 1).min(8);
        }
        PowerupKind::Bomb => {
            player.max_bombs = (player.max_bombs + 1).min(8);
        }
        PowerupKind::Speed => {
            player.speed_level = (player.speed_level + 1).min(4);
        }
        PowerupKind::BombPass => {
            player.bomb_pass = true;
        }
        PowerupKind::Detonator => {
            player.remote = true;
        }
    }
}

// ========== 系统：出口检测 / 复活 / 通关 ==========
pub(super) fn bm_exit_and_respawn(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<BMStage>,
    enemies: Query<&BMEnemy>,
    exits: Query<(&Transform, &Collider), With<BMExit>>,
    players: Query<(&Transform, &Collider), With<BMPlayer>>,
    all_players: Query<&BMPlayer>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    if session.finished {
        return;
    }
    if session.paused {
        return;
    }

    let enemy_count = enemies.iter().count();

    // 状态文案
    if enemy_count == 0 && !stage.all_enemies_dead_msg_shown {
        stage.all_enemies_dead_msg_shown = true;
        stage.status = "敌人清干净了，去出口！".to_string();
    }

    // 出口判定
    if enemy_count == 0 {
        let exit_data: Vec<(Vec2, Vec2)> = exits
            .iter()
            .map(|(t, c)| (t.translation.truncate(), c.size))
            .collect();
        for (pt, pc) in &players {
            let pp = pt.translation.truncate();
            for (ep, es) in &exit_data {
                if aabb_overlap(pp, pc.size, *ep, *es) {
                    finish_bomb_maze(&mut session, &mut save, &stage, true);
                    return;
                }
            }
        }
    }

    // 时间到 → 失败
    if stage.time_left <= 0.0 {
        finish_bomb_maze(&mut session, &mut save, &stage, false);
        return;
    }

    // 双方阵亡 → 失败
    let any_alive = !all_players.is_empty();
    let p1_dead_done = stage.p1_lives < 0;
    let p2_dead_done = stage.p2_lives < 0;
    if !any_alive && p1_dead_done && p2_dead_done {
        finish_bomb_maze(&mut session, &mut save, &stage, false);
        return;
    }

    // 复活逻辑：玩家不存在 + 还有命 → 重新生成
    let mut p1_alive = false;
    let mut p2_alive = false;
    for p in &all_players {
        if p.id == 0 {
            p1_alive = true;
        } else if p.id == 1 {
            p2_alive = true;
        }
    }
    if !p1_alive && stage.p1_lives >= 0 && stage.p1_respawn <= 0.0 {
        spawn_bm_player(&mut commands, 0, P1_SPAWN);
    }
    if !p2_alive && stage.p2_lives >= 0 && stage.p2_respawn <= 0.0 {
        spawn_bm_player(&mut commands, 1, P2_SPAWN);
    }
}

fn finish_bomb_maze(session: &mut GameSession, save: &mut SaveData, stage: &BMStage, won: bool) {
    session.finished = true;
    session.won = won;
    if won {
        session.score += 500 + (stage.time_left * 5.0) as u32;
        let idx = session.kind.index();
        save.high_scores[idx] = save.high_scores[idx].max(session.score);
        save.unlocked_levels[idx] =
            save.unlocked_levels[idx].max((stage.level + 1).min(10));
        save.store();
        session.status = "成功逃出迷宫！Enter 重玩，Esc 返回".to_string();
    } else {
        session.status = "迷宫之旅失败……Enter 重试，Esc 返回".to_string();
    }
}

// ========== 系统：玩家闪烁（无敌时间）==========
pub(super) fn bm_player_blink(
    time: Res<Time>,
    session: Res<GameSession>,
    mut players: Query<(&BMPlayer, &mut Sprite)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let t = time.elapsed_secs();
    for (player, mut sprite) in &mut players {
        if player.invuln > 0.0 {
            let pulse = ((t * 18.0).sin() * 0.5 + 0.5) as f32;
            sprite.color = Color::srgba(0.95, 0.95, 0.98, 0.4 + 0.6 * pulse);
        } else {
            sprite.color = Color::srgb(0.95, 0.95, 0.98);
        }
    }
}

// ========== 系统：HUD 更新 ==========
pub(super) fn bm_hud_update(
    session: Res<GameSession>,
    stage: Option<Res<BMStage>>,
    enemies: Query<&BMEnemy>,
    mut hud: Query<(&BMHud, &mut Text2d)>,
) {
    if session.kind != GameKind::BombMaze {
        return;
    }
    let Some(stage) = stage else { return };
    let enemy_count = enemies.iter().count();
    for (kind, mut text) in &mut hud {
        match kind {
            BMHud::Stage => **text = format!("{}", stage.level),
            BMHud::Time => **text = format!("{:.0}", stage.time_left.max(0.0)),
            BMHud::Score => **text = format!("{}", session.score),
            BMHud::Enemies => **text = format!("{}", enemy_count),
            BMHud::P1Lives => **text = format!("x{}", stage.p1_lives.max(0)),
            BMHud::P2Lives => **text = format!("x{}", stage.p2_lives.max(0)),
            BMHud::Status => {
                let s = if session.finished {
                    if session.won {
                        "成功逃出迷宫！Enter 重玩，Esc 返回"
                    } else {
                        "迷宫之旅失败……Enter 重试，Esc 返回"
                    }
                } else if session.paused {
                    "已暂停：Esc 继续，Backspace 返回菜单"
                } else {
                    stage.status.as_str()
                };
                **text = s.to_string();
            }
        }
    }
}
