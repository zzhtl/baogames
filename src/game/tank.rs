use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::input::input_for;
use crate::common::render::{UiFont, rect, text};

use super::model::*;

// ========== 常量 ==========
pub(super) const SUBTILE: f32 = 16.0;
pub(super) const MAP_TILES_PER_SIDE: i32 = 13;
pub(super) const SUBTILES_PER_SIDE: i32 = MAP_TILES_PER_SIDE * 2;
pub(super) const PLAY_SIZE: f32 = SUBTILES_PER_SIDE as f32 * SUBTILE;
pub(super) const PLAY_OFFSET_X: f32 = -120.0;
pub(super) const PLAY_OFFSET_Y: f32 = -10.0;

pub(super) const TILE: f32 = 2.0 * SUBTILE;
pub(super) const TANK_SIZE: f32 = 2.0 * SUBTILE;
pub(super) const BULLET_SIZE: f32 = 10.0;

pub(super) const PLAYER_SPEED: f32 = 110.0;
pub(super) const ENEMY_SPEED_BASE: f32 = 70.0;
pub(super) const PLAYER_BULLET_SPEED: f32 = 320.0;
pub(super) const ENEMY_BULLET_SPEED: f32 = 220.0;
pub(super) const PLAYER_FIRE_CD: f32 = 0.25;
pub(super) const ENEMY_FIRE_CD: f32 = 1.4;
pub(super) const SPAWN_SHIELD_TIME: f32 = 2.0;

pub(super) const STAGE_TOTAL_ENEMIES: u8 = 20;
pub(super) const MAX_ALIVE_ENEMIES: u8 = 4;
pub(super) const SPAWN_INTERVAL: f32 = 3.0;
pub(super) const RESPAWN_TIME: f32 = 1.2;

pub(super) const Z_TILE: f32 = 0.5;
pub(super) const Z_TANK: f32 = 1.0;
pub(super) const Z_BULLET: f32 = 1.6;
pub(super) const Z_BASE: f32 = 1.0;
pub(super) const Z_BUSH: f32 = 3.0;

const ENEMY_SPAWN_COLS: [i32; 3] = [0, 6, 12];
const PLAYER1_SPAWN: (i32, i32) = (4, 12);
const PLAYER2_SPAWN: (i32, i32) = (8, 12);

// 一个简单的关卡布局，13x13 全块格。
// '.' 空 / 'b' 砖 / 's' 钢 / 'w' 水 / 'g' 草 / 'i' 冰 / 'E' 老巢
const STAGE_MAP: [&str; MAP_TILES_PER_SIDE as usize] = [
    ".............",
    ".bb.......bb.",
    ".bb.......bb.",
    "....s...s....",
    ".............",
    ".b.bb...bb.b.",
    ".b.b.....b.b.",
    ".b.b.www.b.b.",
    ".b.b.....b.b.",
    ".b.bb...bb.b.",
    ".....ggg.....",
    ".....bbb.....",
    ".....bEb.....",
];

// ========== 组件 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum TankDir {
    Up,
    Down,
    Left,
    Right,
}

impl TankDir {
    pub fn vec(self) -> Vec2 {
        match self {
            TankDir::Up => Vec2::new(0.0, 1.0),
            TankDir::Down => Vec2::new(0.0, -1.0),
            TankDir::Left => Vec2::new(-1.0, 0.0),
            TankDir::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn rotation(self) -> f32 {
        use std::f32::consts::*;
        match self {
            TankDir::Up => 0.0,
            TankDir::Left => FRAC_PI_2,
            TankDir::Down => PI,
            TankDir::Right => -FRAC_PI_2,
        }
    }

    pub fn from_input(v: Vec2) -> Option<Self> {
        if v.length_squared() < 0.05 {
            return None;
        }
        if v.x.abs() >= v.y.abs() {
            Some(if v.x > 0.0 {
                TankDir::Right
            } else {
                TankDir::Left
            })
        } else {
            Some(if v.y > 0.0 {
                TankDir::Up
            } else {
                TankDir::Down
            })
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) enum TankSide {
    Player,
    Enemy,
}

#[derive(Component)]
pub(super) struct TankFC {
    pub side: TankSide,
    pub speed: f32,
    pub fire_cd: f32,
    pub fire_cd_left: f32,
    pub bullet_speed: f32,
    pub max_bullets: u8,
    pub bullets_alive: u8,
    pub hp: u8,
    pub shield_left: f32,
}

#[derive(Component)]
pub(super) struct PlayerTankFC {
    pub id: usize,
}

#[derive(Component)]
pub(super) struct EnemyTankFC {
    pub turn_timer: f32,
}

#[derive(Component)]
pub(super) struct BrickFC;

#[derive(Component)]
pub(super) struct SteelFC;

#[derive(Component)]
pub(super) struct WaterFC;

#[derive(Component)]
pub(super) struct BushFC;

#[derive(Component)]
pub(super) struct IceFC;

#[derive(Component)]
pub(super) struct BaseFC;

#[derive(Component)]
pub(super) struct BulletFC {
    pub side: TankSide,
    #[allow(dead_code)]
    pub dir: TankDir,
    #[allow(dead_code)]
    pub power: u8,
    pub owner: Option<Entity>,
}

#[derive(Component)]
pub(super) struct SpawnEffect {
    pub timer: Timer,
    pub spawn_pos: Vec2,
    pub side: TankSide,
    pub player_id: Option<usize>,
}

#[derive(Component)]
pub(super) struct TankHud;

// ========== 资源 ==========
#[derive(Resource)]
pub(super) struct TankStage {
    pub remaining_to_spawn: u8,
    pub spawn_timer: f32,
    pub spawn_idx: usize,
    #[allow(dead_code)]
    pub stage_num: u8,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub base_alive: bool,
    pub kills: u8,
}

// ========== 工具 ==========
fn tile_center(col: i32, row: i32) -> Vec2 {
    // row 0 在顶部，row 12 在底部。tile 是 2 个子格 = 32px。
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5 + col as f32 * 32.0 + 16.0,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - row as f32 * 32.0 - 16.0,
    )
}

fn subtile_center(sx: i32, sy: i32) -> Vec2 {
    // sy 0 在顶部，sy 25 在底部。
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5 + sx as f32 * SUBTILE + SUBTILE * 0.5,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - sy as f32 * SUBTILE - SUBTILE * 0.5,
    )
}

fn play_min() -> Vec2 {
    Vec2::new(
        PLAY_OFFSET_X - PLAY_SIZE * 0.5,
        PLAY_OFFSET_Y - PLAY_SIZE * 0.5,
    )
}

fn play_max() -> Vec2 {
    Vec2::new(
        PLAY_OFFSET_X + PLAY_SIZE * 0.5,
        PLAY_OFFSET_Y + PLAY_SIZE * 0.5,
    )
}

fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    // 黑色游戏底
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_OFFSET_Y),
        Vec2::splat(PLAY_SIZE),
        Color::srgb(0.0, 0.0, 0.0),
        GameEntity,
    );
    // 灰色外框
    let frame_thickness = 6.0;
    let outer = PLAY_SIZE + frame_thickness * 2.0;
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_OFFSET_Y + PLAY_SIZE * 0.5 + frame_thickness * 0.5),
        Vec2::new(outer, frame_thickness),
        Color::srgb(0.45, 0.45, 0.5),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_OFFSET_Y - PLAY_SIZE * 0.5 - frame_thickness * 0.5),
        Vec2::new(outer, frame_thickness),
        Color::srgb(0.45, 0.45, 0.5),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X - PLAY_SIZE * 0.5 - frame_thickness * 0.5, PLAY_OFFSET_Y),
        Vec2::new(frame_thickness, PLAY_SIZE),
        Color::srgb(0.45, 0.45, 0.5),
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X + PLAY_SIZE * 0.5 + frame_thickness * 0.5, PLAY_OFFSET_Y),
        Vec2::new(frame_thickness, PLAY_SIZE),
        Color::srgb(0.45, 0.45, 0.5),
        GameEntity,
    );

    // 解析地图
    for (row, line) in STAGE_MAP.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;
            match ch {
                'b' => spawn_brick_tile(commands, col, row),
                's' => spawn_steel(commands, col, row),
                'w' => spawn_water(commands, col, row),
                'g' => spawn_bush(commands, col, row),
                'i' => spawn_ice(commands, col, row),
                'E' => spawn_base(commands, col, row),
                _ => {}
            }
        }
    }

    // HUD：右侧栏
    let hud_x = PLAY_OFFSET_X + PLAY_SIZE * 0.5 + 90.0;
    text(
        commands,
        font,
        "STAGE",
        Vec2::new(hud_x, PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - 24.0),
        18.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        &format!("{}", level),
        Vec2::new(hud_x, PLAY_OFFSET_Y + PLAY_SIZE * 0.5 - 50.0),
        28.0,
        Color::srgb(1.0, 0.85, 0.3),
        GameEntity,
    );
    text(
        commands,
        font,
        "敌方剩余",
        Vec2::new(hud_x, PLAY_OFFSET_Y + 50.0),
        16.0,
        Color::srgb(0.85, 0.85, 0.85),
        GameEntity,
    );
    text(
        commands,
        font,
        "20",
        Vec2::new(hud_x, PLAY_OFFSET_Y + 20.0),
        26.0,
        Color::srgb(0.95, 0.6, 0.4),
        GameEntity,
    )
    .insert(TankHud);
    text(
        commands,
        font,
        "P1",
        Vec2::new(hud_x - 22.0, PLAY_OFFSET_Y - 60.0),
        18.0,
        Color::srgb(0.85, 0.78, 0.36),
        GameEntity,
    );
    text(
        commands,
        font,
        "P2",
        Vec2::new(hud_x + 22.0, PLAY_OFFSET_Y - 60.0),
        18.0,
        Color::srgb(0.46, 0.7, 0.95),
        GameEntity,
    );

    // 玩家出生
    let p1_pos = tile_center(PLAYER1_SPAWN.0, PLAYER1_SPAWN.1);
    let p2_pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
    spawn_player_tank(commands, 0, p1_pos);
    spawn_player_tank(commands, 1, p2_pos);

    commands.insert_resource(TankStage {
        remaining_to_spawn: STAGE_TOTAL_ENEMIES,
        spawn_timer: 1.5,
        spawn_idx: 0,
        stage_num: level,
        p1_lives: 2,
        p2_lives: 2,
        p1_respawn: 0.0,
        p2_respawn: 0.0,
        base_alive: true,
        kills: 0,
    });
}

// ========== 实体生成 ==========
fn spawn_brick_tile(commands: &mut Commands, col: i32, row: i32) {
    // 一个全块拆成 2x2 个子格
    let sx0 = col * 2;
    let sy0 = row * 2;
    for dy in 0..2 {
        for dx in 0..2 {
            let pos = subtile_center(sx0 + dx, sy0 + dy);
            commands.spawn((
                Sprite::from_color(Color::srgb(0.78, 0.4, 0.16), Vec2::splat(SUBTILE - 0.5)),
                Transform::from_translation(pos.extend(Z_TILE)),
                BrickFC,
                Collider {
                    size: Vec2::splat(SUBTILE),
                },
                GameEntity,
            ));
        }
    }
}

fn spawn_steel(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.72, 0.78, 0.85), Vec2::splat(32.0)),
        Transform::from_translation(pos.extend(Z_TILE)),
        SteelFC,
        Collider {
            size: Vec2::splat(32.0),
        },
        GameEntity,
    ));
    // 高光
    commands.spawn((
        Sprite::from_color(Color::srgb(0.92, 0.96, 1.0), Vec2::new(28.0, 4.0)),
        Transform::from_translation((pos + Vec2::new(0.0, 12.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.45, 0.5, 0.58), Vec2::new(28.0, 4.0)),
        Transform::from_translation((pos + Vec2::new(0.0, -12.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

fn spawn_water(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.4, 0.8), Vec2::splat(32.0)),
        Transform::from_translation(pos.extend(Z_TILE)),
        WaterFC,
        Collider {
            size: Vec2::splat(32.0),
        },
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.55, 0.78, 1.0), Vec2::new(20.0, 3.0)),
        Transform::from_translation((pos + Vec2::new(-3.0, 6.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.55, 0.78, 1.0), Vec2::new(14.0, 3.0)),
        Transform::from_translation((pos + Vec2::new(4.0, -6.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
}

fn spawn_bush(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.62, 0.25), Vec2::splat(32.0)),
        Transform::from_translation(pos.extend(Z_BUSH)),
        BushFC,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.34, 0.78, 0.36), Vec2::new(8.0, 8.0)),
        Transform::from_translation((pos + Vec2::new(-8.0, 6.0)).extend(Z_BUSH + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.34, 0.78, 0.36), Vec2::new(8.0, 8.0)),
        Transform::from_translation((pos + Vec2::new(8.0, -6.0)).extend(Z_BUSH + 0.05)),
        GameEntity,
    ));
}

fn spawn_ice(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.92, 1.0), Vec2::splat(32.0)),
        Transform::from_translation(pos.extend(Z_TILE - 0.1)),
        IceFC,
        GameEntity,
    ));
}

fn spawn_base(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    // 底座
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.18, 0.22), Vec2::splat(28.0)),
        Transform::from_translation(pos.extend(Z_BASE)),
        BaseFC,
        Collider {
            size: Vec2::splat(28.0),
        },
        GameEntity,
    ));
    // 鹰图标用三个色块拼
    commands.spawn((
        Sprite::from_color(Color::srgb(0.95, 0.85, 0.3), Vec2::new(18.0, 12.0)),
        Transform::from_translation((pos + Vec2::new(0.0, 2.0)).extend(Z_BASE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.95, 0.85, 0.3), Vec2::new(8.0, 6.0)),
        Transform::from_translation((pos + Vec2::new(0.0, -6.0)).extend(Z_BASE + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.2, 0.2), Vec2::new(3.0, 3.0)),
        Transform::from_translation((pos + Vec2::new(4.0, 4.0)).extend(Z_BASE + 0.1)),
        GameEntity,
    ));
}

fn spawn_player_tank(commands: &mut Commands, id: usize, pos: Vec2) {
    let body_color = if id == 0 {
        Color::srgb(0.85, 0.78, 0.36)
    } else {
        Color::srgb(0.46, 0.7, 0.95)
    };
    let mut entity = commands.spawn((
        Sprite::from_color(body_color, Vec2::splat(TANK_SIZE)),
        Transform::from_translation(pos.extend(Z_TANK)),
        GameEntity,
        TankFC {
            side: TankSide::Player,
            speed: PLAYER_SPEED,
            fire_cd: PLAYER_FIRE_CD,
            fire_cd_left: 0.0,
            bullet_speed: PLAYER_BULLET_SPEED,
            max_bullets: 1,
            bullets_alive: 0,
            hp: 1,
            shield_left: SPAWN_SHIELD_TIME,
        },
        TankDir::Up,
        PlayerTankFC { id },
        Collider {
            size: Vec2::splat(TANK_SIZE - 2.0),
        },
    ));
    entity.with_children(|p| {
        // 履带横纹
        p.spawn((
            Sprite::from_color(Color::srgb(0.25, 0.22, 0.12), Vec2::new(28.0, 5.0)),
            Transform::from_translation(Vec3::new(0.0, 8.0, 0.05)),
        ));
        p.spawn((
            Sprite::from_color(Color::srgb(0.25, 0.22, 0.12), Vec2::new(28.0, 5.0)),
            Transform::from_translation(Vec3::new(0.0, -8.0, 0.05)),
        ));
        // 炮塔（圆心）
        p.spawn((
            Sprite::from_color(Color::srgb(0.5, 0.45, 0.18), Vec2::splat(12.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.06)),
        ));
        // 炮管（默认朝上）
        p.spawn((
            Sprite::from_color(Color::srgb(0.15, 0.13, 0.08), Vec2::new(4.0, 14.0)),
            Transform::from_translation(Vec3::new(0.0, 12.0, 0.07)),
        ));
    });
}

fn spawn_enemy_tank(commands: &mut Commands, pos: Vec2) {
    let mut entity = commands.spawn((
        Sprite::from_color(Color::srgb(0.78, 0.78, 0.78), Vec2::splat(TANK_SIZE)),
        Transform {
            translation: pos.extend(Z_TANK),
            rotation: Quat::from_rotation_z(TankDir::Down.rotation()),
            ..default()
        },
        GameEntity,
        TankFC {
            side: TankSide::Enemy,
            speed: ENEMY_SPEED_BASE,
            fire_cd: ENEMY_FIRE_CD,
            fire_cd_left: rand::thread_rng().gen_range(0.4..1.2),
            bullet_speed: ENEMY_BULLET_SPEED,
            max_bullets: 1,
            bullets_alive: 0,
            hp: 1,
            shield_left: 0.0,
        },
        TankDir::Down,
        EnemyTankFC { turn_timer: 0.0 },
        Collider {
            size: Vec2::splat(TANK_SIZE - 2.0),
        },
    ));
    entity.with_children(|p| {
        p.spawn((
            Sprite::from_color(Color::srgb(0.32, 0.32, 0.36), Vec2::new(28.0, 5.0)),
            Transform::from_translation(Vec3::new(0.0, 8.0, 0.05)),
        ));
        p.spawn((
            Sprite::from_color(Color::srgb(0.32, 0.32, 0.36), Vec2::new(28.0, 5.0)),
            Transform::from_translation(Vec3::new(0.0, -8.0, 0.05)),
        ));
        p.spawn((
            Sprite::from_color(Color::srgb(0.5, 0.5, 0.55), Vec2::splat(12.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.06)),
        ));
        p.spawn((
            Sprite::from_color(Color::srgb(0.15, 0.15, 0.18), Vec2::new(4.0, 14.0)),
            Transform::from_translation(Vec3::new(0.0, 12.0, 0.07)),
        ));
    });
}

fn spawn_spawn_effect(commands: &mut Commands, pos: Vec2, side: TankSide, player_id: Option<usize>) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.95, 1.0), Vec2::splat(TANK_SIZE)),
        Transform::from_translation(pos.extend(Z_TANK + 0.5)),
        SpawnEffect {
            timer: Timer::new(Duration::from_millis(900), TimerMode::Once),
            spawn_pos: pos,
            side,
            player_id,
        },
        GameEntity,
    ));
}

fn spawn_explosion(commands: &mut Commands, pos: Vec2, big: bool) {
    let size = if big { 36.0 } else { 18.0 };
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.78, 0.22), Vec2::splat(size)),
        Transform::from_translation(pos.extend(Z_BULLET + 0.1)),
        Lifetime(Timer::new(Duration::from_millis(220), TimerMode::Once)),
        GameEntity,
    ));
}

fn spawn_bullet(
    commands: &mut Commands,
    pos: Vec2,
    dir: TankDir,
    speed: f32,
    side: TankSide,
    power: u8,
    owner: Entity,
) {
    let color = match side {
        TankSide::Player => Color::srgb(1.0, 0.95, 0.55),
        TankSide::Enemy => Color::srgb(1.0, 0.5, 0.35),
    };
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(BULLET_SIZE)),
        Transform::from_translation(pos.extend(Z_BULLET)),
        BulletFC {
            side,
            dir,
            power,
            owner: Some(owner),
        },
        Velocity(dir.vec() * speed),
        GameEntity,
    ));
}

fn spawn_muzzle_flash(commands: &mut Commands, pos: Vec2, dir: TankDir) {
    // 一个短促的发光块，方便玩家看见自己刚开了一炮
    let size = match dir {
        TankDir::Up | TankDir::Down => Vec2::new(14.0, 10.0),
        TankDir::Left | TankDir::Right => Vec2::new(10.0, 14.0),
    };
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.95, 0.6), size),
        Transform::from_translation(pos.extend(Z_BULLET + 0.05)),
        Lifetime(Timer::new(Duration::from_millis(80), TimerMode::Once)),
        GameEntity,
    ));
}

// ========== 系统：玩家输入 ==========
pub(super) fn tank_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    mut tanks: Query<
        (
            Entity,
            &PlayerTankFC,
            &mut Transform,
            &mut TankDir,
            &mut TankFC,
        ),
        Without<EnemyTankFC>,
    >,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    if stage.p1_respawn > 0.0 {
        stage.p1_respawn = (stage.p1_respawn - delta).max(0.0);
    }
    if stage.p2_respawn > 0.0 {
        stage.p2_respawn = (stage.p2_respawn - delta).max(0.0);
    }

    for (entity, player, mut tf, mut dir, mut tank) in &mut tanks {
        let input = input_for(&keys, player.id);
        tank.fire_cd_left = (tank.fire_cd_left - delta).max(0.0);
        if tank.shield_left > 0.0 {
            tank.shield_left = (tank.shield_left - delta).max(0.0);
        }

        if let Some(want) = TankDir::from_input(input.move_dir) {
            if want != *dir {
                *dir = want;
                tf.rotation = Quat::from_rotation_z(want.rotation());
                snap_perpendicular(&mut tf.translation, want);
            }
            // 朝向已设定，下一步在 tank_movement 中实际推进
        }

        if input.fire && tank.fire_cd_left <= 0.0 && tank.bullets_alive < tank.max_bullets {
            let muzzle = tf.translation.truncate() + dir.vec() * (TANK_SIZE * 0.5 + 2.0);
            // 只有炮口仍在场地内才允许射击，避免朝边界外开火立即销毁导致体感"打不出子弹"
            let pmin = play_min();
            let pmax = play_max();
            let in_field = muzzle.x > pmin.x && muzzle.x < pmax.x
                && muzzle.y > pmin.y && muzzle.y < pmax.y;
            if in_field {
                spawn_bullet(
                    &mut commands,
                    muzzle,
                    *dir,
                    tank.bullet_speed,
                    TankSide::Player,
                    1,
                    entity,
                );
                spawn_muzzle_flash(&mut commands, muzzle, *dir);
                tank.bullets_alive += 1;
                tank.fire_cd_left = tank.fire_cd;
            }
        }
    }
}

fn snap_perpendicular(t: &mut Vec3, dir: TankDir) {
    // 转向时，把和移动方向垂直的轴吸附到最近的 tile 中线（与所有阻挡物的 32px 网格一致），
    // 这样穿越宽度恰为一个 tile 的巷道时不会因为 8px 偏移卡住。
    let origin = play_min();
    match dir {
        TankDir::Up | TankDir::Down => {
            let rel = t.x - origin.x - TILE * 0.5;
            let snapped = (rel / TILE).round() * TILE + TILE * 0.5 + origin.x;
            t.x = snapped;
        }
        TankDir::Left | TankDir::Right => {
            let rel = t.y - origin.y - TILE * 0.5;
            let snapped = (rel / TILE).round() * TILE + TILE * 0.5 + origin.y;
            t.y = snapped;
        }
    }
}

// ========== 系统：坦克移动（玩家+敌人同享）==========
pub(super) fn tank_movement(
    time: Res<Time>,
    session: Res<GameSession>,
    keys: Res<ButtonInput<KeyCode>>,
    mut tanks: Query<(
        Entity,
        &mut Transform,
        &TankDir,
        &TankFC,
        &Collider,
        Option<&PlayerTankFC>,
    )>,
    blockers: Query<
        (&Transform, &Collider),
        (
            Or<(With<BrickFC>, With<SteelFC>, With<WaterFC>, With<BaseFC>)>,
            Without<TankFC>,
        ),
    >,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let blocker_data: Vec<(Vec2, Vec2)> = blockers
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    let tank_data: Vec<(Entity, Vec2, Vec2)> = tanks
        .iter()
        .map(|(e, t, _, _, c, _)| (e, t.translation.truncate(), c.size))
        .collect();

    for (self_entity, mut tf, dir, tank, _collider, player) in &mut tanks {
        // 玩家：仅按住方向键时才推进
        let advance = if let Some(p) = player {
            input_for(&keys, p.id).move_dir.length_squared() > 0.05
        } else {
            true
        };
        if !advance {
            continue;
        }
        let step = dir.vec() * tank.speed * delta;
        let new_pos = tf.translation.truncate() + step;
        let half = (TANK_SIZE - 2.0) * 0.5;

        // 与场地边界碰撞
        let pmin = play_min();
        let pmax = play_max();
        let clamped = Vec2::new(
            new_pos.x.clamp(pmin.x + half + 1.0, pmax.x - half - 1.0),
            new_pos.y.clamp(pmin.y + half + 1.0, pmax.y - half - 1.0),
        );

        // 阻挡物
        let tank_size = Vec2::splat(TANK_SIZE - 2.0);
        let mut blocked = false;
        for (bp, bs) in &blocker_data {
            if aabb_overlap(clamped, tank_size, *bp, *bs) {
                blocked = true;
                break;
            }
        }
        if !blocked {
            for (other_e, op, os) in &tank_data {
                if *other_e == self_entity {
                    continue;
                }
                if aabb_overlap(clamped, tank_size, *op, *os) {
                    blocked = true;
                    break;
                }
            }
        }
        if !blocked {
            tf.translation.x = clamped.x;
            tf.translation.y = clamped.y;
        }
    }
}

// ========== 系统：敌人 AI ==========
pub(super) fn tank_enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut enemies: Query<
        (
            Entity,
            &mut TankDir,
            &mut Transform,
            &mut TankFC,
            &mut EnemyTankFC,
        ),
        With<EnemyTankFC>,
    >,
    blockers: Query<
        (&Transform, &Collider),
        (
            Or<(With<BrickFC>, With<SteelFC>, With<WaterFC>, With<BaseFC>)>,
            Without<TankFC>,
        ),
    >,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let blocker_data: Vec<(Vec2, Vec2)> = blockers
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    let mut rng = thread_rng();

    for (entity, mut dir, mut tf, mut tank, mut ai) in &mut enemies {
        tank.fire_cd_left = (tank.fire_cd_left - delta).max(0.0);
        ai.turn_timer -= delta;

        // 卡墙检测：试探移动方向是否被挡
        let look_ahead = tf.translation.truncate() + dir.vec() * (tank.speed * delta + 1.0);
        let half_size = Vec2::splat(TANK_SIZE - 2.0);
        let pmin = play_min();
        let pmax = play_max();
        let half = (TANK_SIZE - 2.0) * 0.5;
        let out_of_bounds = look_ahead.x - half < pmin.x
            || look_ahead.x + half > pmax.x
            || look_ahead.y - half < pmin.y
            || look_ahead.y + half > pmax.y;
        let mut wall_blocked = out_of_bounds;
        if !wall_blocked {
            for (bp, bs) in &blocker_data {
                if aabb_overlap(look_ahead, half_size, *bp, *bs) {
                    wall_blocked = true;
                    break;
                }
            }
        }

        if wall_blocked || ai.turn_timer <= 0.0 {
            // 选一个新方向（不和当前一致也不和当前反向）
            let candidates = [TankDir::Up, TankDir::Down, TankDir::Left, TankDir::Right];
            let mut shuffled = candidates;
            shuffled.shuffle(&mut rng);
            let mut new_dir = *dir;
            for c in shuffled {
                if c == *dir {
                    continue;
                }
                let test = tf.translation.truncate() + c.vec() * (tank.speed * delta + 1.0);
                let mut ok = !(test.x - half < pmin.x
                    || test.x + half > pmax.x
                    || test.y - half < pmin.y
                    || test.y + half > pmax.y);
                if ok {
                    for (bp, bs) in &blocker_data {
                        if aabb_overlap(test, half_size, *bp, *bs) {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    new_dir = c;
                    break;
                }
            }
            if new_dir != *dir {
                *dir = new_dir;
                tf.rotation = Quat::from_rotation_z(new_dir.rotation());
                snap_perpendicular(&mut tf.translation, new_dir);
            }
            ai.turn_timer = rng.gen_range(1.5..3.5);
        }

        // 开火
        if tank.fire_cd_left <= 0.0
            && tank.bullets_alive < tank.max_bullets
            && rng.r#gen::<f32>() < 0.04
        {
            let muzzle = tf.translation.truncate() + dir.vec() * (TANK_SIZE * 0.5 + 4.0);
            spawn_bullet(
                &mut commands,
                muzzle,
                *dir,
                tank.bullet_speed,
                TankSide::Enemy,
                1,
                entity,
            );
            tank.bullets_alive += 1;
            tank.fire_cd_left = tank.fire_cd;
        }
    }
}

// ========== 系统：敌人刷怪 ==========
pub(super) fn tank_enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    enemies: Query<&Transform, With<EnemyTankFC>>,
    pending_effects: Query<&SpawnEffect>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let alive: u8 = enemies.iter().count() as u8;
    let pending_enemies: u8 = pending_effects
        .iter()
        .filter(|e| e.side == TankSide::Enemy)
        .count() as u8;
    if alive + pending_enemies >= MAX_ALIVE_ENEMIES || stage.remaining_to_spawn == 0 {
        return;
    }
    stage.spawn_timer -= time.delta_secs();
    if stage.spawn_timer > 0.0 {
        return;
    }
    let col = ENEMY_SPAWN_COLS[stage.spawn_idx % ENEMY_SPAWN_COLS.len()];
    stage.spawn_idx += 1;
    let pos = tile_center(col, 0);
    spawn_spawn_effect(&mut commands, pos, TankSide::Enemy, None);
    stage.remaining_to_spawn -= 1;
    stage.spawn_timer = SPAWN_INTERVAL;
}

// ========== 系统：出生闪烁 → 真正生成 ==========
pub(super) fn tank_spawn_effect(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut effects: Query<(Entity, &mut SpawnEffect, &mut Sprite)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    for (entity, mut effect, mut sprite) in &mut effects {
        effect.timer.tick(time.delta());
        let t = effect.timer.fraction();
        // 闪烁：高低亮交替
        let pulse = ((t * 18.0).sin() * 0.5 + 0.5) as f32;
        sprite.color = Color::srgb(0.8 + 0.2 * pulse, 0.9, 1.0);
        if effect.timer.just_finished() {
            let pos = effect.spawn_pos;
            let side = effect.side;
            let player_id = effect.player_id;
            commands.entity(entity).despawn();
            match side {
                TankSide::Enemy => spawn_enemy_tank(&mut commands, pos),
                TankSide::Player => {
                    if let Some(id) = player_id {
                        spawn_player_tank(&mut commands, id, pos);
                    }
                }
            }
        }
    }
}

// ========== 系统：子弹推进 + 命中 ==========
pub(super) fn tank_bullet_update(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut stage: ResMut<TankStage>,
    mut bullets: Query<(Entity, &mut Transform, &Velocity, &BulletFC)>,
    mut tanks: Query<(Entity, &Transform, &Collider, &mut TankFC), Without<BulletFC>>,
    players: Query<&PlayerTankFC>,
    bricks: Query<(Entity, &Transform, &Collider), (With<BrickFC>, Without<BulletFC>)>,
    steels: Query<(&Transform, &Collider), (With<SteelFC>, Without<BulletFC>)>,
    bases: Query<(Entity, &Transform, &Collider), (With<BaseFC>, Without<BulletFC>)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let pmin = play_min();
    let pmax = play_max();

    let mut consumed: Vec<Entity> = Vec::new();
    let mut destroyed_tanks: Vec<Entity> = Vec::new();
    let mut bullet_owner_release: Vec<Entity> = Vec::new();

    for (entity, mut tf, vel, bullet) in &mut bullets {
        if consumed.contains(&entity) {
            continue;
        }
        tf.translation += vel.extend(0.0) * delta;
        let pos = tf.translation.truncate();
        let half = BULLET_SIZE * 0.5;
        if pos.x - half < pmin.x
            || pos.x + half > pmax.x
            || pos.y - half < pmin.y
            || pos.y + half > pmax.y
        {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        let bsize = Vec2::splat(BULLET_SIZE);
        // 命中砖块（破最近的若干子砖）
        let mut hit_brick = false;
        for (be, bt, bc) in &bricks {
            if aabb_overlap(pos, bsize, bt.translation.truncate(), bc.size) {
                commands.entity(be).despawn();
                hit_brick = true;
                break;
            }
        }
        if hit_brick {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        // 命中钢墙
        let mut hit_steel = false;
        for (st, sc) in &steels {
            if aabb_overlap(pos, bsize, st.translation.truncate(), sc.size) {
                hit_steel = true;
                break;
            }
        }
        if hit_steel {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, false);
            continue;
        }

        // 命中老巢
        let mut hit_base = false;
        for (base_e, bt, bc) in &bases {
            if aabb_overlap(pos, bsize, bt.translation.truncate(), bc.size) {
                commands.entity(base_e).despawn();
                hit_base = true;
                break;
            }
        }
        if hit_base {
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            stage.base_alive = false;
            spawn_explosion(&mut commands, pos, true);
            session.finished = true;
            session.won = false;
            session.status = "基地被打掉了！".to_string();
            continue;
        }

        // 命中坦克
        let mut hit_tank: Option<Entity> = None;
        for (te, tt, tc, _) in tanks.iter() {
            // 同阵营不互伤；自己不伤自己
            if Some(te) == bullet.owner {
                continue;
            }
            if aabb_overlap(pos, bsize, tt.translation.truncate(), tc.size) {
                hit_tank = Some(te);
                break;
            }
        }
        if let Some(victim) = hit_tank {
            // 取目标侧别
            if let Ok((_, _, _, mut victim_tank)) = tanks.get_mut(victim) {
                if victim_tank.side == bullet.side {
                    // 同阵营：仅消耗子弹
                    consumed.push(entity);
                    if let Some(owner) = bullet.owner {
                        bullet_owner_release.push(owner);
                    }
                    spawn_explosion(&mut commands, pos, false);
                    continue;
                }
                if victim_tank.shield_left > 0.0 {
                    consumed.push(entity);
                    if let Some(owner) = bullet.owner {
                        bullet_owner_release.push(owner);
                    }
                    spawn_explosion(&mut commands, pos, false);
                    continue;
                }
                victim_tank.hp = victim_tank.hp.saturating_sub(1);
                if victim_tank.hp == 0 {
                    destroyed_tanks.push(victim);
                }
            }
            consumed.push(entity);
            if let Some(owner) = bullet.owner {
                bullet_owner_release.push(owner);
            }
            spawn_explosion(&mut commands, pos, true);
        }
    }

    // 释放主人的子弹计数
    for owner in bullet_owner_release {
        if let Ok((_, _, _, mut t)) = tanks.get_mut(owner) {
            t.bullets_alive = t.bullets_alive.saturating_sub(1);
        }
    }

    // 处理被击毁的坦克
    for victim in destroyed_tanks {
        if let Ok((_, vt, _, vtank)) = tanks.get(victim) {
            let pos = vt.translation.truncate();
            let was_player = vtank.side == TankSide::Player;
            spawn_explosion(&mut commands, pos, true);
            commands.entity(victim).despawn();
            if was_player {
                if let Ok(p) = players.get(victim) {
                    if p.id == 0 {
                        if stage.p1_lives > 0 {
                            stage.p1_lives -= 1;
                            stage.p1_respawn = RESPAWN_TIME;
                        } else {
                            stage.p1_respawn = -1.0;
                        }
                    } else {
                        if stage.p2_lives > 0 {
                            stage.p2_lives -= 1;
                            stage.p2_respawn = RESPAWN_TIME;
                        } else {
                            stage.p2_respawn = -1.0;
                        }
                    }
                }
            } else {
                stage.kills += 1;
                session.score += 100;
            }
        }
    }

    for entity in consumed {
        commands.entity(entity).despawn();
    }

    // 通关 / 失败判定
    if !session.finished {
        if stage.kills >= STAGE_TOTAL_ENEMIES {
            session.finished = true;
            session.won = true;
            let idx = session.kind.index();
            save.high_scores[idx] = save.high_scores[idx].max(session.score);
            save.unlocked_levels[idx] =
                save.unlocked_levels[idx].max((session.level + 1).min(10));
            save.store();
            session.status = "通关！Enter 重玩，Esc 返回".to_string();
        } else if !stage.base_alive {
            session.finished = true;
            session.won = false;
        } else if stage.p1_respawn < 0.0 && stage.p2_respawn < 0.0 {
            session.finished = true;
            session.won = false;
            session.status = "全员阵亡！".to_string();
        }
    }
}

// ========== 系统：玩家复活 ==========
pub(super) fn tank_player_respawn(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    players: Query<&PlayerTankFC>,
    spawning: Query<&SpawnEffect>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let _ = time;
    let p1_alive = players.iter().any(|p| p.id == 0)
        || spawning.iter().any(|e| e.player_id == Some(0));
    let p2_alive = players.iter().any(|p| p.id == 1)
        || spawning.iter().any(|e| e.player_id == Some(1));

    if !p1_alive && stage.p1_lives > 0 && stage.p1_respawn <= 0.0 && stage.p1_respawn != -1.0 {
        let pos = tile_center(PLAYER1_SPAWN.0, PLAYER1_SPAWN.1);
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(0));
        stage.p1_respawn = RESPAWN_TIME;
    }
    if !p2_alive && stage.p2_lives > 0 && stage.p2_respawn <= 0.0 && stage.p2_respawn != -1.0 {
        let pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(1));
        stage.p2_respawn = RESPAWN_TIME;
    }
}

// ========== 系统：HUD 更新 ==========
pub(super) fn tank_hud_update(
    session: Res<GameSession>,
    stage: Option<Res<TankStage>>,
    mut hud: Query<&mut Text2d, With<TankHud>>,
) {
    if session.kind != GameKind::Tank {
        return;
    }
    let Some(stage) = stage else { return };
    if let Ok(mut text) = hud.single_mut() {
        let remaining = STAGE_TOTAL_ENEMIES - stage.kills;
        **text = format!("{}", remaining);
    }
}
