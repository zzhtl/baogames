use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::common::render::{UiFont, text};

use super::model::*;

// ========== 常量 ==========
pub(super) const TILE: f32 = 24.0;
// 地面抬到屏幕垂直中线偏下，让岩壁断面占满屏幕下半部
pub(super) const FLOOR_Y: f32 = -ARENA_H * 0.5 + 250.0;
pub(super) const GROUND_TOP: f32 = FLOOR_Y + TILE * 0.5;

pub(super) const PLAYER_W: f32 = 18.0;
pub(super) const PLAYER_H: f32 = 32.0;
pub(super) const PRONE_H: f32 = 14.0;
pub(super) const PLAYER_SPEED: f32 = 200.0;
pub(super) const GRAVITY: f32 = 1500.0;
pub(super) const JUMP_VEL: f32 = 480.0;
pub(super) const FALL_MAX: f32 = 800.0;

pub(super) const BULLET_SIZE: f32 = 6.0;
pub(super) const BULLET_BIG_SIZE: f32 = 9.0;
pub(super) const FLAME_SIZE: f32 = 16.0;
pub(super) const PLAYER_BULLET_SPEED: f32 = 620.0;
pub(super) const FLAME_SPEED: f32 = 360.0;
pub(super) const ENEMY_BULLET_SPEED: f32 = 280.0;
pub(super) const FIRE_CD_M: f32 = 0.20;
pub(super) const FIRE_CD_R: f32 = 0.10;
pub(super) const FIRE_CD_S: f32 = 0.20;
pub(super) const FIRE_CD_F: f32 = 0.34;
pub(super) const BULLET_LIFE: f32 = 1.6;

pub(super) const ENEMY_W: f32 = 18.0;
pub(super) const ENEMY_H: f32 = 28.0;
pub(super) const ENEMY_SPEED: f32 = 78.0;
pub(super) const ENEMY_FIRE_CD: f32 = 1.6;
pub(super) const SNIPER_FIRE_CD: f32 = 1.4;
pub(super) const JUMPER_JUMP_VEL: f32 = 360.0;

pub(super) const FALCON_W: f32 = 32.0;
pub(super) const FALCON_H: f32 = 18.0;
pub(super) const FALCON_SPEED: f32 = 150.0;
pub(super) const PICKUP_SIZE: f32 = 22.0;
pub(super) const PICKUP_FALL_SPEED: f32 = 110.0;

pub(super) const BOSS_W: f32 = 220.0;
pub(super) const BOSS_H: f32 = 240.0;
pub(super) const BOSS_HP: i32 = 30;
pub(super) const BOSS_CORE_SIZE: f32 = 38.0;
pub(super) const TURRET_W: f32 = 42.0;
pub(super) const TURRET_H: f32 = 26.0;
pub(super) const TURRET_FIRE_CD: f32 = 1.5;
pub(super) const TURRET_HP: i32 = 6;

pub(super) const CAMERA_FOLLOW_OFFSET: f32 = 60.0;
pub(super) const FALL_DEATH_Y: f32 = -ARENA_H * 0.5 - 40.0;
pub(super) const WORLD_W: f32 = 4400.0;
pub(super) const RESPAWN_TIME: f32 = 1.4;
pub(super) const INVINCIBLE_TIME: f32 = 1.6;
pub(super) const ENEMY_DESPAWN_BACK: f32 = 200.0;

pub(super) const Z_BG: f32 = -8.0;
pub(super) const Z_BG2: f32 = -6.0;
pub(super) const Z_TILE: f32 = 0.4;
pub(super) const Z_PLATFORM: f32 = 0.45;
pub(super) const Z_BOSS: f32 = 0.6;
pub(super) const Z_PICKUP: f32 = 1.05;
pub(super) const Z_ENEMY: f32 = 1.0;
pub(super) const Z_PLAYER: f32 = 1.2;
pub(super) const Z_BULLET: f32 = 1.4;
pub(super) const Z_EXPL: f32 = 1.6;

// FC 魂斗罗 1-1 调色板（NES 实机色）
const COLOR_SKY: Color = Color::srgb(0.0, 0.0, 0.0); // FC 1-1 纯黑天
const COLOR_STAR: Color = Color::srgb(1.0, 1.0, 1.0);
const COLOR_MOUNTAIN: Color = Color::srgb(1.0, 1.0, 1.0); // 雪山主体（纯白）
const COLOR_MOUNTAIN_SHADE: Color = Color::srgb(0.62, 0.62, 0.62); // 雪山阴影（中灰）
const COLOR_MOUNTAIN_TOP: Color = Color::srgb(0.92, 0.18, 0.10); // 山顶被夕阳染红
const COLOR_MOUNTAIN_DARK: Color = Color::srgb(0.0, 0.0, 0.0); // 山脚黑
const COLOR_PALM_LIGHT: Color = Color::srgb(0.27, 0.74, 0.10);
const COLOR_PALM_DARK: Color = Color::srgb(0.04, 0.36, 0.04);
const COLOR_PALM_TRUNK: Color = Color::srgb(0.55, 0.30, 0.10);
const COLOR_GRASS_BRIGHT: Color = Color::srgb(0.27, 0.74, 0.10);
const COLOR_GRASS_DARK: Color = Color::srgb(0.04, 0.36, 0.04);
const COLOR_ROCK_HI: Color = Color::srgb(0.99, 0.74, 0.36); // FC 橙
const COLOR_ROCK_MID: Color = Color::srgb(0.78, 0.42, 0.10);
const COLOR_ROCK_LO: Color = Color::srgb(0.42, 0.20, 0.04);
const COLOR_ROCK_OUT: Color = Color::srgb(0.0, 0.0, 0.0);
const COLOR_PLATFORM: Color = Color::srgb(0.78, 0.42, 0.10);
const COLOR_PLATFORM_TOP: Color = Color::srgb(0.27, 0.74, 0.10);
const COLOR_WATER: Color = Color::srgb(0.20, 0.36, 0.99); // FC 鲜蓝
const COLOR_WATER_HI: Color = Color::srgb(0.62, 0.78, 1.00);
const COLOR_FOAM: Color = Color::srgb(1.00, 1.00, 1.00);
const COLOR_BRIDGE_PLANK: Color = Color::srgb(0.78, 0.42, 0.10);
const COLOR_BRIDGE_PLANK_DK: Color = Color::srgb(0.42, 0.20, 0.04);
const COLOR_BRIDGE_ROPE: Color = Color::srgb(0.99, 0.86, 0.46);

// FC Bill：浅橙头盔 + 白脸 + 绿背心 + 深蓝裤 + 棕靴
const COLOR_PLAYER_SKIN: Color = Color::srgb(0.99, 0.82, 0.62);
const COLOR_PLAYER_HAIR: Color = Color::srgb(0.40, 0.22, 0.08);
const COLOR_PLAYER_HELMET: Color = Color::srgb(0.99, 0.74, 0.36);
const COLOR_PLAYER_HELMET_DK: Color = Color::srgb(0.62, 0.36, 0.10);
const COLOR_PLAYER_BODY: Color = Color::srgb(0.27, 0.74, 0.10); // 绿背心
const COLOR_PLAYER_BODY_DK: Color = Color::srgb(0.04, 0.36, 0.04);
const COLOR_PLAYER_PANTS: Color = Color::srgb(0.10, 0.18, 0.62);
const COLOR_PLAYER_BOOT: Color = Color::srgb(0.42, 0.20, 0.04);
const COLOR_PLAYER_GUN: Color = Color::srgb(0.18, 0.18, 0.22);

const COLOR_ENEMY_BODY: Color = Color::srgb(0.94, 0.30, 0.20); // 红色军团红
const COLOR_ENEMY_HAT: Color = Color::srgb(0.99, 0.74, 0.36);
const COLOR_ENEMY_RED: Color = Color::srgb(0.74, 0.10, 0.10);
const COLOR_ENEMY_BLUE: Color = Color::srgb(0.20, 0.36, 0.99);
const COLOR_ENEMY_SKIN: Color = Color::srgb(0.99, 0.82, 0.62);
const COLOR_ENEMY_PANTS: Color = Color::srgb(0.78, 0.62, 0.20);
const COLOR_ENEMY_GUN: Color = Color::srgb(0.10, 0.10, 0.10);

const COLOR_BULLET_P: Color = Color::srgb(1.0, 0.95, 0.40);
const COLOR_BULLET_E: Color = Color::srgb(1.0, 0.40, 0.32);
const COLOR_FLAME_CORE: Color = Color::srgb(1.0, 0.84, 0.30);

const COLOR_FALCON: Color = Color::srgb(0.88, 0.88, 0.96);
const COLOR_FALCON_DARK: Color = Color::srgb(0.32, 0.32, 0.38);
const COLOR_FALCON_BEAK: Color = Color::srgb(1.0, 0.78, 0.18);
const COLOR_PICKUP_BG: Color = Color::srgb(0.06, 0.06, 0.08);
const COLOR_PICKUP_M: Color = Color::srgb(0.42, 0.92, 0.32);
const COLOR_PICKUP_S: Color = Color::srgb(1.00, 0.78, 0.18);
const COLOR_PICKUP_F: Color = Color::srgb(1.00, 0.42, 0.16);
const COLOR_PICKUP_R: Color = Color::srgb(0.46, 0.78, 1.00);

const COLOR_BOSS_WALL: Color = Color::srgb(0.42, 0.46, 0.52);
const COLOR_BOSS_WALL_DARK: Color = Color::srgb(0.22, 0.24, 0.30);
const COLOR_BOSS_TRIM: Color = Color::srgb(0.78, 0.32, 0.22);
const COLOR_TURRET: Color = Color::srgb(0.30, 0.30, 0.36);
const COLOR_TURRET_BARREL: Color = Color::srgb(0.20, 0.20, 0.24);
const COLOR_BOSS_CORE: Color = Color::srgb(0.96, 0.40, 0.22);
const COLOR_BOSS_CORE_HI: Color = Color::srgb(1.0, 0.78, 0.42);
const COLOR_EXPL_HOT: Color = Color::srgb(1.0, 0.92, 0.36);
const COLOR_EXPL_MID: Color = Color::srgb(1.0, 0.55, 0.18);
const COLOR_EXPL_OUT: Color = Color::srgb(0.78, 0.18, 0.10);

// ========== 枚举 ==========
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Weapon {
    M,
    S,
    F,
    R,
}

impl Weapon {
    fn fire_cd(self) -> f32 {
        match self {
            Weapon::M => FIRE_CD_M,
            Weapon::S => FIRE_CD_S,
            Weapon::F => FIRE_CD_F,
            Weapon::R => FIRE_CD_R,
        }
    }
    fn pickup_color(self) -> Color {
        match self {
            Weapon::M => COLOR_PICKUP_M,
            Weapon::S => COLOR_PICKUP_S,
            Weapon::F => COLOR_PICKUP_F,
            Weapon::R => COLOR_PICKUP_R,
        }
    }
    fn letter(self) -> &'static str {
        match self {
            Weapon::M => "M",
            Weapon::S => "S",
            Weapon::F => "F",
            Weapon::R => "R",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum EnemyKind {
    Soldier, // 跑步射击
    Sniper,  // 站立射击
    Jumper,  // 高处跳下后跑动
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum AimDir {
    Up,
    UpLeft,
    UpRight,
    Left,
    Right,
    DownLeft,
    DownRight,
    Down,
}

impl AimDir {
    fn vec(self) -> Vec2 {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        match self {
            AimDir::Up => Vec2::new(0.0, 1.0),
            AimDir::Down => Vec2::new(0.0, -1.0),
            AimDir::Left => Vec2::new(-1.0, 0.0),
            AimDir::Right => Vec2::new(1.0, 0.0),
            AimDir::UpLeft => Vec2::new(-s, s),
            AimDir::UpRight => Vec2::new(s, s),
            AimDir::DownLeft => Vec2::new(-s, -s),
            AimDir::DownRight => Vec2::new(s, -s),
        }
    }
}

// ========== 组件 ==========
#[derive(Component)]
pub(super) struct ContraPlayer {
    pub vel: Vec2,
    pub on_ground: bool,
    pub prone: bool,
    pub facing: f32,
    pub aim: AimDir,
    pub weapon: Weapon,
    pub fire_cd: f32,
    pub dead_timer: f32,
    pub invincible: f32,
    pub finish: bool,
}

#[derive(Component)]
pub(super) struct ContraBackground;

#[derive(Component, Clone, Copy)]
pub(super) struct ContraSolid {
    pub size: Vec2,
}

#[derive(Component)]
pub(super) struct ContraEnemy {
    pub kind: EnemyKind,
    pub vel: Vec2,
    pub on_ground: bool,
    pub facing: f32,
    pub fire_cd: f32,
    pub ai_t: f32,
    pub hp: i32,
}

#[derive(Component)]
pub(super) struct ContraBullet {
    pub vel: Vec2,
    pub from_player: bool,
    pub weapon: Weapon,
    pub life: f32,
}

#[derive(Component)]
pub(super) struct ContraFalcon {
    pub vel: Vec2,
    pub weapon: Weapon,
}

#[derive(Component)]
pub(super) struct ContraPickup {
    pub weapon: Weapon,
    pub vel_y: f32,
    pub on_ground: bool,
    pub pulse: f32,
}

#[derive(Component)]
pub(super) struct ContraExplosion {
    pub t: f32,
    pub max_t: f32,
    pub size: f32,
}

#[derive(Component)]
pub(super) struct ContraTurret {
    pub fire_cd: f32,
    pub hp: i32,
}

#[derive(Component)]
pub(super) struct ContraBoss {
    pub hp: i32,
    pub die_t: f32,
    pub flash_t: f32,
    pub spawn_t: f32,
}

#[derive(Component, Clone, Copy)]
pub(super) struct ContraHud {
    pub kind: ContraHudKind,
    pub offset: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ContraHudKind {
    Score,
    TopScore,
    Lives,
    Weapon,
    WeaponLetter,
    World,
    Status,
    BossHp,
}

// 命数图标（顶部 HUD 上显示的小 Bill 头）
#[derive(Component)]
pub(super) struct ContraHudLifeIcon {
    pub idx: i32,
}

// ========== 资源 ==========
pub(super) struct EnemySpawnMark {
    pub trigger_x: f32,
    pub pos: Vec2,
    pub kind: EnemyKind,
    pub facing: f32,
}

pub(super) struct FalconMark {
    pub trigger_x: f32,
    pub start: Vec2,
    pub vx: f32,
    pub weapon: Weapon,
}

#[derive(Resource)]
pub(super) struct ContraStage {
    pub level: u8,
    pub world_w: f32,
    pub player_spawn: Vec2,
    pub spawn_marks: Vec<EnemySpawnMark>,
    pub spawn_idx: usize,
    pub falcon_marks: Vec<FalconMark>,
    pub falcon_idx: usize,
    pub boss_x: f32,
    pub boss_spawned: bool,
    pub boss_dead: bool,
    pub top_score: u32,
}

// ========== 工具 ==========
fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

fn player_size(prone: bool) -> Vec2 {
    Vec2::new(PLAYER_W, if prone { PRONE_H } else { PLAYER_H })
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8, top_score: u32) {
    spawn_background(commands);
    spawn_decor(commands);

    // FC 1-1 地形：左侧起点水域 + 陆地段之间用可踏木桥连接
    spawn_water(commands, 0.0, 110.0);
    spawn_ground_run(commands, 110.0, 1100.0, GROUND_TOP);
    spawn_bridge(commands, 1100.0, 1280.0);
    spawn_ground_run(commands, 1280.0, 2280.0, GROUND_TOP);
    spawn_bridge(commands, 2280.0, 2480.0);
    spawn_ground_run(commands, 2480.0, 3700.0, GROUND_TOP);
    spawn_ground_run(commands, 3700.0, WORLD_W, GROUND_TOP);

    // 高处岩石平台
    spawn_platform(commands, Vec2::new(560.0, GROUND_TOP + TILE * 4.0), 144.0);
    spawn_platform(commands, Vec2::new(820.0, GROUND_TOP + TILE * 6.0), 96.0);
    spawn_platform(commands, Vec2::new(1660.0, GROUND_TOP + TILE * 3.5), 192.0);
    spawn_platform(commands, Vec2::new(1900.0, GROUND_TOP + TILE * 6.0), 120.0);
    spawn_platform(commands, Vec2::new(2680.0, GROUND_TOP + TILE * 5.0), 168.0);
    spawn_platform(commands, Vec2::new(3220.0, GROUND_TOP + TILE * 4.0), 200.0);
    spawn_platform(commands, Vec2::new(3520.0, GROUND_TOP + TILE * 6.5), 120.0);

    // 玩家：起点站在左侧水域右边的岩岸上
    let player_spawn = Vec2::new(200.0, GROUND_TOP + PLAYER_H * 0.5);
    spawn_player(commands, player_spawn);

    // 猎鹰空投点
    let falcon_marks = vec![
        FalconMark {
            trigger_x: 280.0,
            start: Vec2::new(0.0 - 80.0, GROUND_TOP + TILE * 7.0),
            vx: FALCON_SPEED,
            weapon: Weapon::S,
        },
        FalconMark {
            trigger_x: 1300.0,
            start: Vec2::new(1100.0, GROUND_TOP + TILE * 8.0),
            vx: FALCON_SPEED,
            weapon: Weapon::R,
        },
        FalconMark {
            trigger_x: 2500.0,
            start: Vec2::new(2300.0, GROUND_TOP + TILE * 7.5),
            vx: FALCON_SPEED,
            weapon: Weapon::F,
        },
        FalconMark {
            trigger_x: 3120.0,
            start: Vec2::new(2900.0, GROUND_TOP + TILE * 7.0),
            vx: FALCON_SPEED,
            weapon: Weapon::S,
        },
    ];

    // 敌人触发点（trigger_x = 摄像机右沿到达此 x 后生成）
    let spawn_marks = vec![
        // 段 1
        EnemySpawnMark {
            trigger_x: 200.0,
            pos: Vec2::new(360.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 280.0,
            pos: Vec2::new(560.0, GROUND_TOP + TILE * 4.0 + ENEMY_H * 0.5),
            kind: EnemyKind::Sniper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 360.0,
            pos: Vec2::new(740.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 480.0,
            pos: Vec2::new(820.0, GROUND_TOP + TILE * 6.0 + ENEMY_H * 0.5),
            kind: EnemyKind::Jumper,
            facing: -1.0,
        },
        // 段 2 之前 (跨水)
        EnemySpawnMark {
            trigger_x: 680.0,
            pos: Vec2::new(960.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 880.0,
            pos: Vec2::new(1060.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        // 段 2
        EnemySpawnMark {
            trigger_x: 1100.0,
            pos: Vec2::new(1380.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 1280.0,
            pos: Vec2::new(1660.0, GROUND_TOP + TILE * 3.5 + ENEMY_H * 0.5),
            kind: EnemyKind::Sniper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 1380.0,
            pos: Vec2::new(1780.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 1500.0,
            pos: Vec2::new(1900.0, GROUND_TOP + TILE * 6.0 + ENEMY_H * 0.5),
            kind: EnemyKind::Jumper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 1700.0,
            pos: Vec2::new(2080.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 1840.0,
            pos: Vec2::new(2200.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        // 段 3
        EnemySpawnMark {
            trigger_x: 2200.0,
            pos: Vec2::new(2520.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 2360.0,
            pos: Vec2::new(2680.0, GROUND_TOP + TILE * 5.0 + ENEMY_H * 0.5),
            kind: EnemyKind::Sniper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 2520.0,
            pos: Vec2::new(2820.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 2680.0,
            pos: Vec2::new(2960.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 2800.0,
            pos: Vec2::new(3100.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 2960.0,
            pos: Vec2::new(3220.0, GROUND_TOP + TILE * 4.0 + ENEMY_H * 0.5),
            kind: EnemyKind::Sniper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 3060.0,
            pos: Vec2::new(3360.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 3180.0,
            pos: Vec2::new(3520.0, GROUND_TOP + TILE * 6.5 + ENEMY_H * 0.5),
            kind: EnemyKind::Jumper,
            facing: -1.0,
        },
        EnemySpawnMark {
            trigger_x: 3260.0,
            pos: Vec2::new(3600.0, GROUND_TOP + ENEMY_H * 0.5),
            kind: EnemyKind::Soldier,
            facing: -1.0,
        },
    ];

    // HUD
    spawn_hud(commands, font, level, top_score);

    let stage = ContraStage {
        level,
        world_w: WORLD_W,
        player_spawn,
        spawn_marks,
        spawn_idx: 0,
        falcon_marks,
        falcon_idx: 0,
        boss_x: 4040.0,
        boss_spawned: false,
        boss_dead: false,
        top_score,
    };
    commands.insert_resource(stage);
}

// 背景：FC 1-1 纯黑夜空 + 稀疏星点（sky 跟随相机，星点作为 sky 子节点自动相对屏幕）
fn spawn_background(commands: &mut Commands) {
    let sky = commands
        .spawn((
            Sprite::from_color(COLOR_SKY, Vec2::new(ARENA_W * 1.4, ARENA_H * 1.4)),
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_BG)),
            ContraBackground,
            GameEntity,
        ))
        .id();
    // FC 风格的稀疏星点（贴近顶部，靠在 HUD 下方）
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

// 远景装饰：FC 1-1 白色雪山带（固定世界坐标）
fn spawn_decor(commands: &mut Commands) {
    // 远景雪山高于地平线一点，正好露在草线之上
    let base_y = GROUND_TOP + 32.0;
    // FC 1-1 的山是连绵的尖峰，高度控制在 60~110 让屏幕容得下
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

// FC 风格三角雪山：左白主体 + 右灰阴影斜面 + 红色山顶 + 黑色描边
fn spawn_mountain(commands: &mut Commands, base: Vec2, base_w: f32, peak_h: f32) {
    let step_h = 4.0_f32;
    let layers = (peak_h / step_h).max(1.0) as i32;
    // 顶端 4 层用红色封顶
    let red_layers = 4;
    for i in 0..layers {
        let frac = i as f32 / layers as f32;
        let w = (base_w * (1.0 - frac)).max(2.0);
        let y = base.y + i as f32 * step_h + step_h * 0.5;
        let is_top = i >= layers - red_layers;
        let bright_color = if is_top { COLOR_MOUNTAIN_TOP } else { COLOR_MOUNTAIN };
        let shade_color = if is_top { COLOR_MOUNTAIN_TOP } else { COLOR_MOUNTAIN_SHADE };
        // 左侧白色主体（顶端为红）
        let bright_w = w * 0.55;
        commands.spawn((
            Sprite::from_color(bright_color, Vec2::new(bright_w, step_h + 0.5)),
            Transform::from_translation(Vec3::new(
                base.x - (w - bright_w) * 0.5,
                y,
                Z_BG2,
            )),
            GameEntity,
        ));
        // 右侧灰色阴影（顶端为红）
        let shade_w = w * 0.45;
        commands.spawn((
            Sprite::from_color(shade_color, Vec2::new(shade_w, step_h + 0.5)),
            Transform::from_translation(Vec3::new(
                base.x + (w - shade_w) * 0.5,
                y,
                Z_BG2 + 0.01,
            )),
            GameEntity,
        ));
    }
    // 山脚黑色基线（FC 标志性的硬边）
    commands.spawn((
        Sprite::from_color(COLOR_MOUNTAIN_DARK, Vec2::new(base_w + 2.0, 3.0)),
        Transform::from_translation(Vec3::new(base.x, base.y - 1.5, Z_BG2 + 0.02)),
        GameEntity,
    ));
}

// FC 棕榈树：每棵单独画——树干 + 顶端扇形树叶
fn spawn_palm_tree(commands: &mut Commands, base_x: f32, ground_y: f32, height: f32, mirror: bool) {
    // 树干（深棕，略向上偏）
    let trunk_h = height;
    let trunk_w = 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(trunk_w, trunk_h)),
        Transform::from_translation(Vec3::new(base_x, ground_y + trunk_h * 0.5, Z_BG2 + 0.10)),
        GameEntity,
    ));
    // 树干阴影线（左侧暗色细条）
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(2.0, trunk_h)),
        Transform::from_translation(Vec3::new(base_x - 2.0, ground_y + trunk_h * 0.5, Z_BG2 + 0.11)),
        GameEntity,
    ));
    // 树冠中心（一团暗绿做底）
    let crown_y = ground_y + trunk_h + 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_PALM_DARK, Vec2::new(28.0, 14.0)),
        Transform::from_translation(Vec3::new(base_x, crown_y, Z_BG2 + 0.12)),
        GameEntity,
    ));
    // 椰果（树干顶端深棕小球）
    commands.spawn((
        Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(6.0, 4.0)),
        Transform::from_translation(Vec3::new(base_x + 4.0, crown_y - 4.0, Z_BG2 + 0.13)),
        GameEntity,
    ));
    // 6 片棕榈叶：长方形向四周伸展
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
    // 叶片末端的小亮点
    for (dx, dy) in [(-32.0 * sign, 6.0), (32.0 * sign, -2.0), (-26.0 * sign, 12.0)] {
        commands.spawn((
            Sprite::from_color(COLOR_PALM_LIGHT, Vec2::new(4.0, 3.0)),
            Transform::from_translation(Vec3::new(base_x + dx, crown_y + dy, Z_BG2 + 0.14)),
            GameEntity,
        ));
    }
}

// 在地段顶部撒一排棕榈树（FC 1-1 散布在地表上）
fn spawn_canopy(commands: &mut Commands, x_start: f32, x_end: f32, top_y: f32) {
    let mut x = x_start + 38.0;
    let mut idx = 0i32;
    while x < x_end - 32.0 {
        // 高度与朝向交替，看起来更自然
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

fn spawn_ground_run(commands: &mut Commands, x_start: f32, x_end: f32, top_y: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;

    // 1) 上方棕榈树
    spawn_canopy(commands, x_start, x_end, top_y);

    // 2) FC 草带：上方一条亮绿条（带锯齿草尖）+ 下方暗绿过渡
    let grass_top = top_y;
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
    // 顶端草尖：FC 风格的简单方块凸起（每 12 像素一个 4x3 方块）
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
    // 草地下沿挂着的暗绿色草丛（FC 1-1 草线下垂的"须"）
    let tuft_count = (w / 28.0) as i32;
    for i in 0..tuft_count {
        let tx = x_start + 14.0 + i as f32 * 28.0;
        if tx > x_end - 10.0 {
            break;
        }
        let tuft_y = grass_top - bright_h - dark_h - 2.0;
        // 中段大块
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(8.0, 6.0)),
            Transform::from_translation(Vec3::new(tx, tuft_y, Z_TILE + 0.18)),
            GameEntity,
        ));
        // 中央向下的小尖
        commands.spawn((
            Sprite::from_color(COLOR_GRASS_DARK, Vec2::new(3.0, 4.0)),
            Transform::from_translation(Vec3::new(tx, tuft_y - 4.0, Z_TILE + 0.18)),
            GameEntity,
        ));
        // 两侧细小草丝
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

    // 3) 岩壁断面：FC 1-1 经典的橙色岩壁，延伸到屏幕底部
    let rock_top = grass_top - bright_h - dark_h;
    let rock_h_total = 280.0;
    let rock_bottom = rock_top - rock_h_total;
    // 主体橙色岩壁
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(w, rock_h_total)),
        Transform::from_translation(Vec3::new(cx, (rock_top + rock_bottom) * 0.5, Z_TILE)),
        GameEntity,
    ));
    // 顶部一行较亮的橙色（草线下面的亮带）
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(w, 8.0)),
        Transform::from_translation(Vec3::new(cx, rock_top - 4.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    // 底部暗带
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(w, 12.0)),
        Transform::from_translation(Vec3::new(cx, rock_bottom + 6.0, Z_TILE + 0.03)),
        GameEntity,
    ));

    // 4) FC 1-1 圆石砌墙：错位的大块圆石
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

// FC 1-1 圆石：粗黑描边的椭圆石头（用多层方块近似圆角），中间橙黄色亮带
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
    // 1) 粗黑描边外壳（整块）
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(real_w, h)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.02)),
        GameEntity,
    ));
    // 2) 削掉四角，模拟圆角石头：上下两端用稍窄的橙色块覆盖
    let inner_w = real_w - 4.0;
    let inner_h = h - 2.0;
    // 主体橙色（暗）
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(inner_w, inner_h)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.03)),
        GameEntity,
    ));
    // 上下进一步收窄一点点，做圆角错觉
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_LO, Vec2::new(inner_w - 4.0, inner_h + 2.0)),
        Transform::from_translation(Vec3::new(cx, center.y, Z_TILE + 0.031)),
        GameEntity,
    ));
    // 3) 中段橙色亮带（占石头中部 ~60%）
    let mid_h = (h * 0.55).max(8.0);
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_MID, Vec2::new(inner_w - 2.0, mid_h)),
        Transform::from_translation(Vec3::new(cx, center.y - 1.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    // 4) 顶部高光（左偏一点的小亮黄条）
    let hi_w = (inner_w * 0.65).max(6.0);
    commands.spawn((
        Sprite::from_color(COLOR_ROCK_HI, Vec2::new(hi_w, 3.0)),
        Transform::from_translation(Vec3::new(cx - 2.0, center.y + h * 0.5 - 5.0, Z_TILE + 0.05)),
        GameEntity,
    ));
    // 5) 左上小白点（圆石高光点）
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

fn spawn_water(commands: &mut Commands, x_start: f32, x_end: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;
    let top_y = GROUND_TOP - 16.0;
    let h = 220.0;
    let cy = top_y - h * 0.5;
    // FC 鲜蓝主水面
    commands.spawn((
        Sprite::from_color(COLOR_WATER, Vec2::new(w, h)),
        Transform::from_translation(Vec3::new(cx, cy, Z_TILE)),
        GameEntity,
    ));
    // 顶部亮蓝条
    commands.spawn((
        Sprite::from_color(COLOR_WATER_HI, Vec2::new(w, 6.0)),
        Transform::from_translation(Vec3::new(cx, top_y - 3.0, Z_TILE + 0.04)),
        GameEntity,
    ));
    // FC 风格的水面波纹：白色短横线交错排布
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
    // 顶部白色泡沫线
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

// FC 1-1 标志性的木桥：木板+绳索悬挂，可踩踏（实体碰撞）
fn spawn_bridge(commands: &mut Commands, x_start: f32, x_end: f32) {
    let w = x_end - x_start;
    let cx = (x_start + x_end) * 0.5;
    // 桥下水
    spawn_water(commands, x_start, x_end);
    // 两侧绳索吊柱
    for side_x in [x_start + 4.0, x_end - 4.0] {
        commands.spawn((
            Sprite::from_color(COLOR_PALM_TRUNK, Vec2::new(6.0, 32.0)),
            Transform::from_translation(Vec3::new(side_x, GROUND_TOP + 16.0, Z_TILE + 0.10)),
            GameEntity,
        ));
    }
    // 横向绳索（黄色）从两边柱顶垂下来
    let plank_top = GROUND_TOP - 1.0;
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_ROPE, Vec2::new(w, 2.0)),
        Transform::from_translation(Vec3::new(cx, GROUND_TOP + 28.0, Z_TILE + 0.11)),
        GameEntity,
    ));
    // 桥面木板：暗色底
    let plank_h = 12.0;
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_PLANK_DK, Vec2::new(w, plank_h)),
        Transform::from_translation(Vec3::new(cx, plank_top - plank_h * 0.5, Z_TILE + 0.13)),
        GameEntity,
    ));
    // 木板顶面（亮色）
    commands.spawn((
        Sprite::from_color(COLOR_BRIDGE_PLANK, Vec2::new(w, 5.0)),
        Transform::from_translation(Vec3::new(cx, plank_top - 2.5, Z_TILE + 0.14)),
        GameEntity,
    ));
    // 木板分缝：每 16 像素一道暗线
    let n = (w / 16.0) as i32;
    for i in 1..n {
        let x = x_start + i as f32 * 16.0;
        commands.spawn((
            Sprite::from_color(COLOR_ROCK_OUT, Vec2::new(1.0, plank_h)),
            Transform::from_translation(Vec3::new(x, plank_top - plank_h * 0.5, Z_TILE + 0.15)),
            GameEntity,
        ));
    }
    // 短绳吊线：木板顶面到上方主绳
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
    // 桥面碰撞：与桥面同高，玩家可踏
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(w, plank_h)),
        Transform::from_translation(Vec3::new(cx, plank_top - plank_h * 0.5, Z_TILE - 0.1)),
        ContraSolid {
            size: Vec2::new(w, plank_h),
        },
        GameEntity,
    ));
}

fn spawn_platform(commands: &mut Commands, center: Vec2, width: f32) {
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
    // 实体碰撞：与平台同高
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(width, h)),
        Transform::from_translation(Vec3::new(center.x, center.y - h * 0.5, Z_PLATFORM - 0.1)),
        ContraSolid {
            size: Vec2::new(width, h),
        },
        GameEntity,
    ));
}

fn spawn_player(commands: &mut Commands, pos: Vec2) {
    let parent = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::new(PLAYER_W, PLAYER_H),
            ),
            Transform::from_translation(pos.extend(Z_PLAYER)),
            ContraPlayer {
                vel: Vec2::ZERO,
                on_ground: false,
                prone: false,
                facing: 1.0,
                aim: AimDir::Right,
                weapon: Weapon::M,
                fire_cd: 0.0,
                dead_timer: 0.0,
                invincible: 1.0,
                finish: false,
            },
            GameEntity,
        ))
        .id();
    // FC Bill：橙色头盔 + 白脸 + 绿背心 + 深蓝裤 + 棕靴；朝右
    let parts: [(f32, f32, f32, f32, Color); 14] = [
        // 头盔
        (0.0, 13.0, 12.0, 3.0, COLOR_PLAYER_HELMET),     // 头盔顶橙带
        (0.0, 11.0, 13.0, 2.0, COLOR_PLAYER_HELMET_DK),  // 头盔下沿暗带
        // 脸
        (0.0, 8.0, 10.0, 4.0, COLOR_PLAYER_SKIN),        // 脸
        (3.0, 8.0, 2.0, 2.0, COLOR_PLAYER_HAIR),         // 眼睛点
        // 上身：绿背心
        (0.0, 2.0, 12.0, 7.0, COLOR_PLAYER_BODY),        // 背心主体
        (0.0, 5.0, 12.0, 2.0, COLOR_PLAYER_BODY_DK),     // 背心暗影线
        (-4.0, 0.0, 4.0, 4.0, COLOR_PLAYER_SKIN),        // 左小臂裸露
        (4.0, 0.0, 4.0, 4.0, COLOR_PLAYER_SKIN),         // 右小臂裸露
        // 腰带
        (0.0, -3.0, 12.0, 2.0, COLOR_PLAYER_BOOT),
        // 裤腿
        (-3.0, -8.0, 5.0, 7.0, COLOR_PLAYER_PANTS),
        (3.0, -8.0, 5.0, 7.0, COLOR_PLAYER_PANTS),
        // 靴子
        (-3.0, -14.0, 5.0, 3.0, COLOR_PLAYER_BOOT),
        (3.0, -14.0, 5.0, 3.0, COLOR_PLAYER_BOOT),
        // 步枪（朝右伸出）
        (10.0, 2.0, 14.0, 2.0, COLOR_PLAYER_GUN),
    ];
    for (dx, dy, w, h, color) in parts {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, Z_PLAYER + 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

fn spawn_enemy(commands: &mut Commands, mark: &EnemySpawnMark) {
    let body_color = match mark.kind {
        EnemyKind::Soldier => COLOR_ENEMY_BODY,
        EnemyKind::Sniper => COLOR_ENEMY_RED,
        EnemyKind::Jumper => COLOR_ENEMY_BLUE,
    };
    let vy = if matches!(mark.kind, EnemyKind::Jumper) {
        -120.0
    } else {
        0.0
    };
    let parent = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::new(ENEMY_W, ENEMY_H),
            ),
            Transform::from_translation(mark.pos.extend(Z_ENEMY)),
            ContraEnemy {
                kind: mark.kind,
                vel: Vec2::new(0.0, vy),
                on_ground: false,
                facing: mark.facing,
                fire_cd: match mark.kind {
                    EnemyKind::Sniper => 0.6,
                    _ => 1.2,
                },
                ai_t: 0.0,
                hp: 1,
            },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 10] = [
        (0.0, 11.0, 12.0, 3.0, COLOR_ENEMY_HAT),     // 钢盔顶
        (0.0, 9.0, 13.0, 2.0, COLOR_ROCK_OUT),       // 钢盔下沿黑边
        (0.0, 6.0, 10.0, 3.0, COLOR_ENEMY_SKIN),     // 脸
        (0.0, 1.0, 13.0, 7.0, body_color),           // 红衣
        (0.0, 4.0, 13.0, 2.0, COLOR_ENEMY_RED),      // 衣前暗红横带
        (-3.0, -6.0, 5.0, 6.0, COLOR_ENEMY_PANTS),   // 左裤腿
        (3.0, -6.0, 5.0, 6.0, COLOR_ENEMY_PANTS),    // 右裤腿
        (-3.0, -12.0, 5.0, 4.0, COLOR_PLAYER_BOOT),  // 左靴
        (3.0, -12.0, 5.0, 4.0, COLOR_PLAYER_BOOT),   // 右靴
        (9.0, 2.0, 12.0, 2.0, COLOR_ENEMY_GUN),      // 步枪
    ];
    for (dx, dy, w, h, color) in parts {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, Z_ENEMY + 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

fn spawn_falcon(commands: &mut Commands, mark: &FalconMark) {
    let parent = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::new(FALCON_W, FALCON_H),
            ),
            Transform::from_translation(mark.start.extend(Z_PICKUP + 0.1)),
            ContraFalcon {
                vel: Vec2::new(mark.vx, 0.0),
                weapon: mark.weapon,
            },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 7] = [
        (0.0, 0.0, 16.0, 8.0, COLOR_FALCON),       // 身体
        (-12.0, 1.0, 10.0, 4.0, COLOR_FALCON_DARK), // 尾巴
        (10.0, 1.0, 6.0, 6.0, COLOR_FALCON),        // 头
        (14.0, 1.0, 4.0, 2.0, COLOR_FALCON_BEAK),   // 喙
        (-2.0, 5.0, 14.0, 4.0, COLOR_FALCON_DARK),  // 上翅
        (-2.0, -5.0, 14.0, 4.0, COLOR_FALCON_DARK), // 下翅
        (4.0, 1.0, 4.0, 2.0, mark.weapon.pickup_color()), // 携带物色
    ];
    for (dx, dy, w, h, color) in parts {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, Z_PICKUP + 0.15)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

fn spawn_pickup(commands: &mut Commands, font: &UiFont, pos: Vec2, weapon: Weapon) {
    let parent = commands
        .spawn((
            Sprite::from_color(weapon.pickup_color(), Vec2::new(PICKUP_SIZE, PICKUP_SIZE)),
            Transform::from_translation(pos.extend(Z_PICKUP)),
            ContraPickup {
                weapon,
                vel_y: 0.0,
                on_ground: false,
                pulse: 0.0,
            },
            GameEntity,
        ))
        .id();
    commands
        .spawn((
            Sprite::from_color(COLOR_PICKUP_BG, Vec2::splat(PICKUP_SIZE - 6.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_PICKUP + 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(parent));
    commands
        .spawn((
            Text2d::new(weapon.letter()),
            TextFont::from_font_size(14.0).with_font(font.0.clone()),
            TextColor(weapon.pickup_color()),
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_PICKUP + 0.10)),
            GameEntity,
        ))
        .insert(ChildOf(parent));
}

fn spawn_explosion(commands: &mut Commands, pos: Vec2, size: f32, life: f32) {
    commands.spawn((
        Sprite::from_color(COLOR_EXPL_HOT, Vec2::splat(size)),
        Transform::from_translation(pos.extend(Z_EXPL)),
        ContraExplosion {
            t: 0.0,
            max_t: life,
            size,
        },
        GameEntity,
    ));
}

fn spawn_player_bullet(commands: &mut Commands, pos: Vec2, dir: Vec2, weapon: Weapon) {
    let (size, color, speed, life) = match weapon {
        Weapon::F => (FLAME_SIZE, COLOR_FLAME_CORE, FLAME_SPEED, BULLET_LIFE * 0.8),
        Weapon::S => (
            BULLET_BIG_SIZE,
            COLOR_BULLET_P,
            PLAYER_BULLET_SPEED,
            BULLET_LIFE,
        ),
        _ => (
            BULLET_SIZE,
            COLOR_BULLET_P,
            PLAYER_BULLET_SPEED,
            BULLET_LIFE,
        ),
    };
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(size)),
        Transform::from_translation(pos.extend(Z_BULLET)),
        ContraBullet {
            vel: dir.normalize_or_zero() * speed,
            from_player: true,
            weapon,
            life,
        },
        GameEntity,
    ));
}

fn spawn_enemy_bullet(commands: &mut Commands, pos: Vec2, dir: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_BULLET_E, Vec2::splat(BULLET_SIZE)),
        Transform::from_translation(pos.extend(Z_BULLET)),
        ContraBullet {
            vel: dir.normalize_or_zero() * ENEMY_BULLET_SPEED,
            from_player: false,
            weapon: Weapon::M,
            life: BULLET_LIFE * 1.4,
        },
        GameEntity,
    ));
}

fn spawn_boss(commands: &mut Commands, x: f32) {
    let center_y = GROUND_TOP + BOSS_H * 0.5;
    // 主体墙
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_WALL, Vec2::new(BOSS_W, BOSS_H)),
        Transform::from_translation(Vec3::new(x, center_y, Z_BOSS)),
        ContraBoss {
            hp: BOSS_HP,
            die_t: 0.0,
            flash_t: 0.0,
            spawn_t: 0.0,
        },
        GameEntity,
    ));
    // 边框
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_WALL_DARK, Vec2::new(BOSS_W, 12.0)),
        Transform::from_translation(Vec3::new(x, center_y + BOSS_H * 0.5 - 6.0, Z_BOSS + 0.05)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_WALL_DARK, Vec2::new(BOSS_W, 12.0)),
        Transform::from_translation(Vec3::new(x, center_y - BOSS_H * 0.5 + 6.0, Z_BOSS + 0.05)),
        GameEntity,
    ));
    // 红色装饰条
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_TRIM, Vec2::new(BOSS_W - 24.0, 6.0)),
        Transform::from_translation(Vec3::new(x, center_y + 80.0, Z_BOSS + 0.1)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_TRIM, Vec2::new(BOSS_W - 24.0, 6.0)),
        Transform::from_translation(Vec3::new(x, center_y - 80.0, Z_BOSS + 0.1)),
        GameEntity,
    ));
    // 中心核心
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_CORE, Vec2::splat(BOSS_CORE_SIZE)),
        Transform::from_translation(Vec3::new(x, center_y, Z_BOSS + 0.2)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_BOSS_CORE_HI, Vec2::splat(BOSS_CORE_SIZE * 0.55)),
        Transform::from_translation(Vec3::new(x, center_y, Z_BOSS + 0.25)),
        GameEntity,
    ));
    // 上下两个炮塔
    let turret_offsets = [80.0_f32, -80.0];
    for dy in turret_offsets {
        commands.spawn((
            Sprite::from_color(COLOR_TURRET, Vec2::new(TURRET_W, TURRET_H)),
            Transform::from_translation(Vec3::new(x - 24.0, center_y + dy, Z_BOSS + 0.3)),
            ContraTurret {
                fire_cd: 0.8,
                hp: TURRET_HP,
            },
            GameEntity,
        ));
        commands.spawn((
            Sprite::from_color(COLOR_TURRET_BARREL, Vec2::new(20.0, 8.0)),
            Transform::from_translation(Vec3::new(x - 56.0, center_y + dy, Z_BOSS + 0.32)),
            GameEntity,
        ));
    }
}

// ========== HUD ==========
// HUD 元素会随摄像机移动（位置 = camera.translation + offset）
#[derive(Component)]
pub(super) struct ContraHudPanel {
    pub offset: Vec2,
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8, top_score: u32) {
    // FC 经典：顶部一条黑底窄条，显示 TOP / 1P / 命数 / 武器
    let hud_z = 9.0_f32;
    let top_y = ARENA_H * 0.5 - 14.0;
    let bar_off = Vec2::new(0.0, top_y);
    let bar_size = Vec2::new(ARENA_W, 28.0);
    // 主黑条
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), bar_size),
            Transform::from_translation(bar_off.extend(hud_z)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: bar_off });
    // 底部一条 1px 白线分隔（FC 风格）
    let line_off = Vec2::new(0.0, top_y - 14.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.85, 0.85, 0.85), Vec2::new(ARENA_W, 1.0)),
            Transform::from_translation(line_off.extend(hud_z + 0.05)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: line_off });

    // TOP-000000（左）
    let top_off = Vec2::new(-ARENA_W * 0.5 + 96.0, top_y);
    text(
        commands,
        font,
        &format!("TOP-{:06}", top_score),
        top_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::TopScore,
        offset: top_off,
    });

    // 1P-000000（左中）
    let score_off = Vec2::new(-ARENA_W * 0.5 + 232.0, top_y);
    text(
        commands,
        font,
        "1P-000000",
        score_off,
        14.0,
        Color::srgb(1.0, 0.95, 0.30),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Score,
        offset: score_off,
    });

    // STAGE 1-x（中）
    let world_off = Vec2::new(0.0, top_y);
    text(
        commands,
        font,
        &format!("STAGE 1-{}", level),
        world_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::World,
        offset: world_off,
    });

    // 命数小 Bill 头像（最多渲 4 个，靠右）
    let lives_label_off = Vec2::new(110.0, top_y);
    text(
        commands,
        font,
        "x3",
        lives_label_off,
        14.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Lives,
        offset: lives_label_off,
    });
    for i in 0..4 {
        let icon_off = Vec2::new(140.0 + i as f32 * 14.0, top_y);
        spawn_life_icon(commands, icon_off, hud_z + 0.2, i);
    }

    // 武器字母圆图标（右侧）
    let weapon_off = Vec2::new(ARENA_W * 0.5 - 80.0, top_y);
    // 圆形底（用方形近似）：黑边 + 颜色填充
    commands
        .spawn((
            Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(20.0, 20.0)),
            Transform::from_translation(weapon_off.extend(hud_z + 0.1)),
            GameEntity,
        ))
        .insert(ContraHudPanel { offset: weapon_off });
    commands
        .spawn((
            Sprite::from_color(COLOR_PICKUP_M, Vec2::new(16.0, 16.0)),
            Transform::from_translation(weapon_off.extend(hud_z + 0.15)),
            GameEntity,
        ))
        .insert(ContraHud {
            kind: ContraHudKind::Weapon,
            offset: weapon_off,
        });
    let weapon_letter_off = Vec2::new(ARENA_W * 0.5 - 80.0, top_y);
    text(
        commands,
        font,
        "M",
        weapon_letter_off,
        14.0,
        Color::srgb(0.0, 0.0, 0.0),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::WeaponLetter,
        offset: weapon_letter_off,
    });

    // 居中 Status：仅在暂停 / 结算时显示
    let status_off = Vec2::new(0.0, 0.0);
    text(
        commands,
        font,
        "",
        status_off,
        22.0,
        Color::srgb(1.0, 0.95, 0.30),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::Status,
        offset: status_off,
    });

    // BOSS HP（右）
    let boss_off = Vec2::new(ARENA_W * 0.5 - 30.0, top_y);
    text(
        commands,
        font,
        "",
        boss_off,
        12.0,
        Color::srgb(1.0, 0.4, 0.32),
        GameEntity,
    )
    .insert(ContraHud {
        kind: ContraHudKind::BossHp,
        offset: boss_off,
    });
}

// 顶部 HUD 上的小 Bill 头像（命数图标）
fn spawn_life_icon(commands: &mut Commands, offset: Vec2, z: f32, idx: i32) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(12.0, 12.0)),
            Transform::from_translation(offset.extend(z)),
            ContraHudLifeIcon { idx },
            ContraHudPanel { offset },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 5] = [
        (0.0, 4.0, 9.0, 2.0, COLOR_PLAYER_HELMET),    // 头盔顶
        (0.0, 2.5, 10.0, 1.0, COLOR_PLAYER_HELMET_DK),// 帽沿
        (0.0, 0.0, 8.0, 3.0, COLOR_PLAYER_SKIN),      // 脸
        (-2.0, -2.5, 8.0, 1.0, COLOR_PLAYER_BODY),    // 绿背心
        (2.0, 0.0, 1.0, 1.0, COLOR_PLAYER_HAIR),      // 眼
    ];
    for (dx, dy, w, h, color) in parts {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(parent));
    }
}

// ========== 系统：玩家输入 + 射击 ==========
pub(super) fn contra_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(&mut ContraPlayer, &mut Sprite, &mut Transform)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut sprite, mut tr)) = q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finish {
        return;
    }

    let left = keys.pressed(KeyCode::KeyA);
    let right = keys.pressed(KeyCode::KeyD);
    let up = keys.pressed(KeyCode::KeyW);
    let down = keys.pressed(KeyCode::KeyS);
    let fire = keys.pressed(KeyCode::KeyJ);
    let jump_pressed = keys.just_pressed(KeyCode::KeyK) || keys.just_pressed(KeyCode::Space);

    // 朝向
    if right && !left {
        player.facing = 1.0;
    } else if left && !right {
        player.facing = -1.0;
    }

    // 蹲下：仅地面 && 没在跳
    let want_prone = down && player.on_ground && !up;
    if want_prone != player.prone {
        let dh = (PLAYER_H - PRONE_H) * 0.5;
        if want_prone {
            tr.translation.y -= dh;
        } else {
            tr.translation.y += dh;
        }
        player.prone = want_prone;
        sprite.custom_size = Some(player_size(player.prone));
    }

    // 瞄准方向
    let aim = if up && left && !right {
        AimDir::UpLeft
    } else if up && right && !left {
        AimDir::UpRight
    } else if up {
        AimDir::Up
    } else if down && !player.on_ground && left && !right {
        AimDir::DownLeft
    } else if down && !player.on_ground && right && !left {
        AimDir::DownRight
    } else if down && !player.on_ground {
        AimDir::Down
    } else if left && !right {
        AimDir::Left
    } else if right && !left {
        AimDir::Right
    } else if player.facing >= 0.0 {
        AimDir::Right
    } else {
        AimDir::Left
    };
    player.aim = aim;

    // 水平速度（蹲下时不能移动）
    let target_x = if player.prone {
        0.0
    } else if left && !right {
        -PLAYER_SPEED
    } else if right && !left {
        PLAYER_SPEED
    } else {
        0.0
    };
    player.vel.x = target_x;

    // 跳跃
    if jump_pressed && player.on_ground && !player.prone {
        player.vel.y = JUMP_VEL;
        player.on_ground = false;
    }

    // 射击 CD
    if player.fire_cd > 0.0 {
        player.fire_cd = (player.fire_cd - dt).max(0.0);
    }
    if fire && player.fire_cd <= 0.0 {
        let weapon = player.weapon;
        let aim = player.aim;
        let muzzle = muzzle_offset(player.prone, aim, player.facing);
        let origin = tr.translation.truncate() + muzzle;
        let dir = aim.vec();
        match weapon {
            Weapon::S => {
                // 5 颗散射子弹，角度 ±20°
                for delta in [-0.35_f32, -0.18, 0.0, 0.18, 0.35] {
                    let (sin, cos) = delta.sin_cos();
                    let v = Vec2::new(
                        dir.x * cos - dir.y * sin,
                        dir.x * sin + dir.y * cos,
                    );
                    spawn_player_bullet(&mut commands, origin, v, Weapon::S);
                }
            }
            _ => spawn_player_bullet(&mut commands, origin, dir, weapon),
        }
        player.fire_cd = weapon.fire_cd();
    }

    // X 翻转
    let s = if player.facing >= 0.0 { 1.0 } else { -1.0 };
    tr.scale.x = s;
}

fn muzzle_offset(prone: bool, aim: AimDir, facing: f32) -> Vec2 {
    let body_h = if prone { PRONE_H } else { PLAYER_H };
    let mid_y = if prone { 0.0 } else { 4.0 };
    let dir = aim.vec();
    // 用 aim 方向，偏移到身体外沿
    let half = body_h * 0.5;
    Vec2::new(
        dir.x * (PLAYER_W * 0.5 + 8.0) + if dir.x.abs() < 0.01 { facing * 2.0 } else { 0.0 },
        dir.y * (half + 6.0) + mid_y,
    )
}

// ========== 系统：玩家物理 + 地形碰撞 ==========
pub(super) fn contra_physics(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<ContraStage>,
    mut player_q: Query<(&mut ContraPlayer, &mut Transform), Without<ContraSolid>>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraPlayer>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((mut player, mut tr)) = player_q.single_mut() else {
        return;
    };
    if player.invincible > 0.0 {
        player.invincible = (player.invincible - dt).max(0.0);
    }

    if player.dead_timer > 0.0 {
        // 死亡：抛物线下落
        player.vel.y -= GRAVITY * dt;
        player.vel.y = player.vel.y.max(-FALL_MAX);
        tr.translation.x += player.vel.x * dt;
        tr.translation.y += player.vel.y * dt;
        return;
    }

    // 重力
    player.vel.y -= GRAVITY * dt;
    player.vel.y = player.vel.y.max(-FALL_MAX);

    let mut pos = tr.translation.truncate();
    let p_size = player_size(player.prone);

    // X 轴
    pos.x += player.vel.x * dt;
    // 左侧起点水域 110px，玩家不可走进水里
    let left_min = 110.0 + PLAYER_W * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let mut right_max = WORLD_W - PLAYER_W * 0.5;
    if stage.boss_spawned && !stage.boss_dead {
        right_max = right_max.min(stage.boss_x - BOSS_W * 0.5 - PLAYER_W * 0.5 - 4.0);
    }
    if pos.x > right_max {
        pos.x = right_max;
        if player.vel.x > 0.0 {
            player.vel.x = 0.0;
        }
    }
    for (st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dx = pos.x - sp.x;
            let push = (p_size.x + s.size.x) * 0.5 - dx.abs();
            if push > 0.0 {
                if dx > 0.0 {
                    pos.x += push;
                } else {
                    pos.x -= push;
                }
                player.vel.x = 0.0;
            }
        }
    }

    // Y 轴
    pos.y += player.vel.y * dt;
    let mut on_ground = false;
    for (st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dy = pos.y - sp.y;
            let push = (p_size.y + s.size.y) * 0.5 - dy.abs();
            if push > 0.0 {
                if dy > 0.0 {
                    pos.y += push;
                    if player.vel.y < 0.0 {
                        on_ground = true;
                        player.vel.y = 0.0;
                    }
                } else {
                    pos.y -= push;
                    if player.vel.y > 0.0 {
                        player.vel.y = 0.0;
                    }
                }
            }
        }
    }
    player.on_ground = on_ground;

    tr.translation.x = pos.x;
    tr.translation.y = pos.y;

    // 跌落水/坑：FALL_DEATH_Y
    if pos.y < FALL_DEATH_Y && player.dead_timer <= 0.0 {
        player.dead_timer = RESPAWN_TIME;
        player.vel = Vec2::new(0.0, 0.0);
    }
}

// ========== 系统：摄像机跟随 ==========
pub(super) fn contra_camera_follow(
    stage: Res<ContraStage>,
    player_q: Query<
        &Transform,
        (
            With<ContraPlayer>,
            Without<Camera>,
            Without<ContraBackground>,
            Without<ContraHud>,
            Without<ContraHudPanel>,
        ),
    >,
    mut cam_q: Query<
        &mut Transform,
        (
            With<Camera>,
            Without<ContraPlayer>,
            Without<ContraBackground>,
            Without<ContraHud>,
            Without<ContraHudPanel>,
        ),
    >,
    mut bg_q: Query<
        &mut Transform,
        (
            With<ContraBackground>,
            Without<Camera>,
            Without<ContraPlayer>,
            Without<ContraHud>,
            Without<ContraHudPanel>,
        ),
    >,
    mut hud_q: Query<
        (&ContraHud, &mut Transform),
        (
            Without<ContraBackground>,
            Without<Camera>,
            Without<ContraPlayer>,
            Without<ContraHudPanel>,
        ),
    >,
    mut hudp_q: Query<
        (&ContraHudPanel, &mut Transform),
        (
            Without<ContraBackground>,
            Without<Camera>,
            Without<ContraPlayer>,
            Without<ContraHud>,
        ),
    >,
) {
    let Ok(ptr) = player_q.single() else { return };
    let Ok(mut ctr) = cam_q.single_mut() else {
        return;
    };
    let cam_min = ARENA_W * 0.5;
    let cam_max = stage.world_w - ARENA_W * 0.5;
    let target = (ptr.translation.x + CAMERA_FOLLOW_OFFSET).clamp(cam_min, cam_max);
    if target > ctr.translation.x {
        ctr.translation.x = target;
    } else if ctr.translation.x < cam_min {
        ctr.translation.x = cam_min;
    }
    ctr.translation.y = 0.0;
    for mut btr in &mut bg_q {
        btr.translation.x = ctr.translation.x;
        btr.translation.y = ctr.translation.y;
    }
    for (hud, mut tr) in &mut hud_q {
        tr.translation.x = ctr.translation.x + hud.offset.x;
        tr.translation.y = ctr.translation.y + hud.offset.y;
    }
    for (hudp, mut tr) in &mut hudp_q {
        tr.translation.x = ctr.translation.x + hudp.offset.x;
        tr.translation.y = ctr.translation.y + hudp.offset.y;
    }
}

// ========== 系统：敌人 AI ==========
pub(super) fn contra_enemy_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut enemy_q: Query<(&mut ContraEnemy, &mut Transform), Without<ContraSolid>>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraEnemy>>,
    player_q: Query<(&ContraPlayer, &Transform), Without<ContraEnemy>>,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraEnemy>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok(cam) = cam_q.single() else { return };
    let cam_x = cam.translation.x;
    let player_pos = player_q
        .single()
        .map(|(p, t)| {
            if p.dead_timer > 0.0 {
                None
            } else {
                Some(t.translation.truncate())
            }
        })
        .unwrap_or(None);

    for (mut enemy, mut tr) in &mut enemy_q {
        let mut pos = tr.translation.truncate();
        enemy.ai_t += dt;
        match enemy.kind {
            EnemyKind::Soldier => {
                // 朝玩家方向跑
                if let Some(pp) = player_pos {
                    let dx = pp.x - pos.x;
                    if dx.abs() > 24.0 {
                        enemy.facing = dx.signum();
                    }
                    enemy.vel.x = enemy.facing * ENEMY_SPEED;
                } else {
                    enemy.vel.x = enemy.facing * ENEMY_SPEED;
                }
            }
            EnemyKind::Sniper => {
                enemy.vel.x = 0.0;
                if let Some(pp) = player_pos {
                    enemy.facing = (pp.x - pos.x).signum().max(-1.0).min(1.0);
                    if enemy.facing == 0.0 {
                        enemy.facing = -1.0;
                    }
                }
            }
            EnemyKind::Jumper => {
                if enemy.on_ground {
                    if enemy.ai_t > 1.0 {
                        enemy.vel.y = JUMPER_JUMP_VEL;
                        enemy.on_ground = false;
                        enemy.ai_t = 0.0;
                    }
                    if let Some(pp) = player_pos {
                        let dx = pp.x - pos.x;
                        if dx.abs() > 24.0 {
                            enemy.facing = dx.signum();
                        }
                    }
                    enemy.vel.x = enemy.facing * ENEMY_SPEED * 0.8;
                } else {
                    enemy.vel.x = enemy.facing * ENEMY_SPEED * 0.6;
                }
            }
        }

        // 重力
        enemy.vel.y -= GRAVITY * dt;
        enemy.vel.y = enemy.vel.y.max(-FALL_MAX);

        // 移动 + 碰撞
        let size = Vec2::new(ENEMY_W, ENEMY_H);
        pos.x += enemy.vel.x * dt;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dx = pos.x - sp.x;
                let push = (size.x + s.size.x) * 0.5 - dx.abs();
                if push > 0.0 {
                    if dx > 0.0 {
                        pos.x += push;
                    } else {
                        pos.x -= push;
                    }
                    enemy.vel.x = 0.0;
                    enemy.facing = -enemy.facing;
                }
            }
        }
        pos.y += enemy.vel.y * dt;
        let mut grounded = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if enemy.vel.y < 0.0 {
                            grounded = true;
                            enemy.vel.y = 0.0;
                        }
                    } else {
                        pos.y -= push;
                        if enemy.vel.y > 0.0 {
                            enemy.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        enemy.on_ground = grounded;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;
        let s = if enemy.facing >= 0.0 { 1.0 } else { -1.0 };
        tr.scale.x = s;

        // 射击
        enemy.fire_cd = (enemy.fire_cd - dt).max(0.0);
        if enemy.fire_cd <= 0.0 {
            if let Some(pp) = player_pos {
                let mut dir = pp - pos;
                if dir.x.abs() < 4.0 {
                    dir.x = enemy.facing * 1.0;
                }
                // 不射到摄像机外的玩家也没意思，这里限制可见范围
                if (pos.x - cam_x).abs() < ARENA_W * 0.55 {
                    spawn_enemy_bullet(&mut commands, pos, dir);
                    enemy.fire_cd = match enemy.kind {
                        EnemyKind::Sniper => SNIPER_FIRE_CD,
                        _ => ENEMY_FIRE_CD,
                    };
                } else {
                    enemy.fire_cd = ENEMY_FIRE_CD * 0.6;
                }
            }
        }
    }
}

// ========== 系统：敌人 + 猎鹰 + 道具的镜头外清理（跟随相机方向） ==========
pub(super) fn contra_offscreen_cleanup(
    mut commands: Commands,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraEnemy>)>,
    enemy_q: Query<(Entity, &Transform), With<ContraEnemy>>,
    bullet_q: Query<(Entity, &Transform), With<ContraBullet>>,
    pickup_q: Query<(Entity, &Transform), With<ContraPickup>>,
) {
    let Ok(cam) = cam_q.single() else { return };
    let cam_x = cam.translation.x;
    let cull = cam_x - ARENA_W * 0.5 - ENEMY_DESPAWN_BACK;
    for (e, t) in &enemy_q {
        if t.translation.x < cull {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &bullet_q {
        if t.translation.x < cull || t.translation.x > cam_x + ARENA_W * 0.5 + 200.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &pickup_q {
        if t.translation.x < cull {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：子弹移动 + 命中 ==========
pub(super) fn contra_bullets_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    font: Res<UiFont>,
    mut commands: Commands,
    mut bullet_q: Query<(Entity, &mut ContraBullet, &mut Transform)>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraBullet>>,
    mut enemy_q: Query<
        (Entity, &mut ContraEnemy, &Transform),
        (Without<ContraBullet>, Without<ContraSolid>),
    >,
    mut player_q: Query<
        (&mut ContraPlayer, &Transform),
        (Without<ContraBullet>, Without<ContraSolid>, Without<ContraEnemy>),
    >,
    mut turret_q: Query<
        (Entity, &mut ContraTurret, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
        ),
    >,
    mut boss_q: Query<
        (Entity, &mut ContraBoss, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
            Without<ContraTurret>,
        ),
    >,
    mut falcon_q: Query<
        (Entity, &ContraFalcon, &Transform),
        (
            Without<ContraBullet>,
            Without<ContraSolid>,
            Without<ContraEnemy>,
            Without<ContraPlayer>,
            Without<ContraTurret>,
            Without<ContraBoss>,
        ),
    >,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let mut score_gain: u32 = 0;
    let mut killed_falcons: Vec<(Vec2, Weapon)> = Vec::new();

    for (be, mut bullet, mut btr) in &mut bullet_q {
        bullet.life -= dt;
        if bullet.life <= 0.0 {
            commands.entity(be).despawn();
            continue;
        }
        btr.translation.x += bullet.vel.x * dt;
        btr.translation.y += bullet.vel.y * dt;
        let bp = btr.translation.truncate();
        let bs = if matches!(bullet.weapon, Weapon::F) {
            Vec2::splat(FLAME_SIZE)
        } else if matches!(bullet.weapon, Weapon::S) {
            Vec2::splat(BULLET_BIG_SIZE)
        } else {
            Vec2::splat(BULLET_SIZE)
        };

        // 撞地形
        let mut blocked = false;
        for (st, s) in &solid_q {
            if aabb_overlap(bp, bs, st.translation.truncate(), s.size) {
                blocked = true;
                break;
            }
        }
        if blocked {
            spawn_explosion(&mut commands, bp, 14.0, 0.18);
            commands.entity(be).despawn();
            continue;
        }

        if bullet.from_player {
            // 命中敌兵
            let mut hit_enemy = false;
            for (ee, mut enemy, etr) in &mut enemy_q {
                if aabb_overlap(bp, bs, etr.translation.truncate(), Vec2::new(ENEMY_W, ENEMY_H)) {
                    enemy.hp -= 1;
                    if enemy.hp <= 0 {
                        spawn_explosion(&mut commands, etr.translation.truncate(), 26.0, 0.32);
                        commands.entity(ee).despawn();
                        score_gain += 100;
                    }
                    hit_enemy = true;
                    break;
                }
            }
            if hit_enemy {
                if !matches!(bullet.weapon, Weapon::F | Weapon::S) {
                    commands.entity(be).despawn();
                }
                continue;
            }
            // 命中炮塔
            let mut hit_t = false;
            for (te, mut turret, ttr) in &mut turret_q {
                if aabb_overlap(
                    bp,
                    bs,
                    ttr.translation.truncate(),
                    Vec2::new(TURRET_W, TURRET_H),
                ) {
                    turret.hp -= 1;
                    if turret.hp <= 0 {
                        spawn_explosion(&mut commands, ttr.translation.truncate(), 36.0, 0.5);
                        commands.entity(te).despawn();
                        score_gain += 500;
                    }
                    hit_t = true;
                    break;
                }
            }
            if hit_t {
                if !matches!(bullet.weapon, Weapon::F) {
                    commands.entity(be).despawn();
                }
                continue;
            }
            // 命中 BOSS（整堵墙都算受击）
            let mut hit_boss = false;
            for (_be2, mut boss, btr2) in &mut boss_q {
                if boss.die_t > 0.0 {
                    continue;
                }
                let bpos = btr2.translation.truncate();
                if aabb_overlap(bp, bs, bpos, Vec2::new(BOSS_W, BOSS_H)) {
                    boss.hp -= 1;
                    boss.flash_t = 0.12;
                    score_gain += 50;
                    if boss.hp <= 0 {
                        boss.die_t = 1.4;
                    }
                    hit_boss = true;
                    spawn_explosion(&mut commands, bp, 14.0, 0.18);
                    break;
                }
            }
            if hit_boss {
                if !matches!(bullet.weapon, Weapon::F) {
                    commands.entity(be).despawn();
                }
                continue;
            }
            // 命中猎鹰
            let mut hit_f = false;
            for (fe, falcon, ftr) in &mut falcon_q {
                if aabb_overlap(
                    bp,
                    bs,
                    ftr.translation.truncate(),
                    Vec2::new(FALCON_W, FALCON_H),
                ) {
                    spawn_explosion(&mut commands, ftr.translation.truncate(), 30.0, 0.3);
                    killed_falcons.push((ftr.translation.truncate(), falcon.weapon));
                    commands.entity(fe).despawn();
                    score_gain += 200;
                    hit_f = true;
                    break;
                }
            }
            if hit_f {
                if !matches!(bullet.weapon, Weapon::F) {
                    commands.entity(be).despawn();
                }
                continue;
            }
        } else {
            // 敌弹打玩家
            if let Ok((mut player, ptr)) = player_q.single_mut() {
                if player.dead_timer <= 0.0 && player.invincible <= 0.0 {
                    let pp = ptr.translation.truncate();
                    let ps = player_size(player.prone);
                    if aabb_overlap(bp, bs, pp, ps) {
                        kill_player(&mut player, ptr.translation.truncate(), &mut commands);
                        commands.entity(be).despawn();
                        if session.lives > 0 {
                            session.lives -= 1;
                        }
                        continue;
                    }
                }
            }
        }
    }

    if score_gain > 0 {
        session.score += score_gain;
    }
    for (pos, w) in killed_falcons {
        spawn_pickup(&mut commands, &font, pos, w);
    }
}

fn kill_player(player: &mut ContraPlayer, pos: Vec2, commands: &mut Commands) {
    player.dead_timer = RESPAWN_TIME;
    player.vel = Vec2::new(0.0, 320.0);
    spawn_explosion(commands, pos, 32.0, 0.45);
}

// ========== 系统：玩家 vs 敌人接触 ==========
pub(super) fn contra_player_vs_enemy(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut ContraPlayer, &Transform)>,
    enemy_q: Query<(Entity, &Transform), With<ContraEnemy>>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.invincible > 0.0 {
        return;
    }
    let pp = ptr.translation.truncate();
    let ps = player_size(player.prone);
    for (ee, etr) in &enemy_q {
        let ep = etr.translation.truncate();
        if aabb_overlap(pp, ps, ep, Vec2::new(ENEMY_W, ENEMY_H)) {
            kill_player(&mut player, pp, &mut commands);
            spawn_explosion(&mut commands, ep, 24.0, 0.3);
            commands.entity(ee).despawn();
            if session.lives > 0 {
                session.lives -= 1;
            }
            return;
        }
    }
}

// ========== 系统：玩家拾取 ==========
pub(super) fn contra_pickup_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut commands: Commands,
    mut pickup_q: Query<(Entity, &mut ContraPickup, &mut Transform, &mut Sprite), Without<ContraPlayer>>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraPickup>>,
    mut player_q: Query<(&mut ContraPlayer, &Transform), Without<ContraPickup>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (e, mut p, mut tr, mut sprite) in &mut pickup_q {
        if !p.on_ground {
            p.vel_y = (p.vel_y - GRAVITY * 0.6 * dt).max(-PICKUP_FALL_SPEED * 1.5);
            tr.translation.y += p.vel_y * dt;
            // 着地检测
            let pos = tr.translation.truncate();
            for (st, s) in &solid_q {
                let sp = st.translation.truncate();
                if aabb_overlap(pos, Vec2::splat(PICKUP_SIZE), sp, s.size) {
                    let dy = pos.y - sp.y;
                    if dy > 0.0 && p.vel_y < 0.0 {
                        let push = (PICKUP_SIZE + s.size.y) * 0.5 - dy.abs();
                        if push > 0.0 {
                            tr.translation.y += push;
                            p.vel_y = 0.0;
                            p.on_ground = true;
                        }
                    }
                }
            }
        }
        // 闪烁
        p.pulse += dt;
        let blink = (p.pulse * 6.0).sin() * 0.5 + 0.5;
        let mut col = p.weapon.pickup_color().to_srgba();
        col.red = (col.red * (0.6 + 0.4 * blink)).min(1.0);
        col.green = (col.green * (0.6 + 0.4 * blink)).min(1.0);
        col.blue = (col.blue * (0.6 + 0.4 * blink)).min(1.0);
        sprite.color = Color::Srgba(col);

        // 玩家拾取
        if let Ok((mut player, ptr)) = player_q.single_mut() {
            if player.dead_timer <= 0.0 {
                let pp = ptr.translation.truncate();
                let ps = player_size(player.prone);
                if aabb_overlap(pp, ps, tr.translation.truncate(), Vec2::splat(PICKUP_SIZE)) {
                    player.weapon = p.weapon;
                    commands.entity(e).despawn();
                    session.score += 300;
                }
            }
        }
    }
}

// ========== 系统：猎鹰移动 ==========
pub(super) fn contra_falcon_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraFalcon, &mut Transform, &mut Sprite)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    for (e, f, mut tr, mut sprite) in &mut q {
        tr.translation.x += f.vel.x * dt;
        tr.translation.y += (f.vel.x.signum() * 6.0 * (time.elapsed_secs() * 4.0).sin()) * dt;
        let mut col = COLOR_FALCON.to_srgba();
        let phase = (time.elapsed_secs() * 8.0).sin() * 0.5 + 0.5;
        col.red = 0.7 + 0.3 * phase;
        col.green = 0.7 + 0.3 * phase;
        col.blue = 0.85;
        sprite.color = Color::Srgba(col);
        if tr.translation.x > WORLD_W + 80.0 || tr.translation.x < -80.0 {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：spawner（按相机推进生成敌人/猎鹰） ==========
pub(super) fn contra_spawner(
    session: Res<GameSession>,
    mut commands: Commands,
    cam_q: Query<&Transform, With<Camera>>,
    mut stage: ResMut<ContraStage>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(cam) = cam_q.single() else { return };
    let cam_right = cam.translation.x + ARENA_W * 0.5;
    while stage.spawn_idx < stage.spawn_marks.len()
        && stage.spawn_marks[stage.spawn_idx].trigger_x <= cam_right
    {
        let mark_pos = stage.spawn_marks[stage.spawn_idx].pos;
        let mark_kind = stage.spawn_marks[stage.spawn_idx].kind;
        let mark_facing = stage.spawn_marks[stage.spawn_idx].facing;
        spawn_enemy(
            &mut commands,
            &EnemySpawnMark {
                trigger_x: 0.0,
                pos: mark_pos,
                kind: mark_kind,
                facing: mark_facing,
            },
        );
        stage.spawn_idx += 1;
    }
    while stage.falcon_idx < stage.falcon_marks.len()
        && stage.falcon_marks[stage.falcon_idx].trigger_x <= cam_right
    {
        let m = &stage.falcon_marks[stage.falcon_idx];
        spawn_falcon(
            &mut commands,
            &FalconMark {
                trigger_x: 0.0,
                start: m.start,
                vx: m.vx,
                weapon: m.weapon,
            },
        );
        stage.falcon_idx += 1;
    }
    // BOSS
    if !stage.boss_spawned && cam_right >= stage.boss_x - 60.0 {
        spawn_boss(&mut commands, stage.boss_x);
        stage.boss_spawned = true;
    }
}

// ========== 系统：BOSS AI ==========
pub(super) fn contra_boss_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut commands: Commands,
    mut stage: ResMut<ContraStage>,
    mut boss_q: Query<(Entity, &mut ContraBoss, &Transform)>,
    mut turret_q: Query<(&mut ContraTurret, &Transform), Without<ContraBoss>>,
    player_q: Query<&Transform, (With<ContraPlayer>, Without<ContraBoss>, Without<ContraTurret>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let player_pos = player_q.single().ok().map(|t| t.translation.truncate());
    for (be, mut boss, btr) in &mut boss_q {
        boss.spawn_t += dt;
        if boss.flash_t > 0.0 {
            boss.flash_t = (boss.flash_t - dt).max(0.0);
        }
        if boss.die_t > 0.0 {
            boss.die_t -= dt;
            // 频繁爆炸
            if (boss.die_t * 8.0).fract() < dt * 8.0 {
                let dx = ((boss.die_t * 13.0).sin()) * BOSS_W * 0.4;
                let dy = ((boss.die_t * 17.0).cos()) * BOSS_H * 0.4;
                spawn_explosion(
                    &mut commands,
                    btr.translation.truncate() + Vec2::new(dx, dy),
                    36.0,
                    0.4,
                );
            }
            if boss.die_t <= 0.0 {
                spawn_explosion(&mut commands, btr.translation.truncate(), 80.0, 0.7);
                commands.entity(be).despawn();
                stage.boss_dead = true;
                session.finished = true;
                session.won = true;
                session.status =
                    "STAGE CLEAR！Enter 重玩 / Backspace 返回菜单".to_string();
                session.score += 5000;
            }
            continue;
        }
        // 炮塔射击
        for (mut turret, ttr) in &mut turret_q {
            turret.fire_cd = (turret.fire_cd - dt).max(0.0);
            if turret.fire_cd <= 0.0 {
                if let Some(pp) = player_pos {
                    let origin = ttr.translation.truncate() + Vec2::new(-22.0, 0.0);
                    let mut dir = pp - origin;
                    if dir.length_squared() < 4.0 {
                        dir = Vec2::new(-1.0, 0.0);
                    }
                    spawn_enemy_bullet(&mut commands, origin, dir);
                    turret.fire_cd = TURRET_FIRE_CD;
                }
            }
        }
    }
}

// ========== 系统：玩家闪烁（无敌） ==========
pub(super) fn contra_player_blink(
    time: Res<Time>,
    mut q: Query<(&ContraPlayer, &mut Visibility)>,
) {
    for (player, mut visibility) in &mut q {
        let show = if player.invincible > 0.0 {
            (time.elapsed_secs() * 24.0).sin() > 0.0
        } else {
            true
        };
        *visibility = if show {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

// ========== 系统：爆炸生命周期 ==========
pub(super) fn contra_explosion_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraExplosion, &mut Sprite, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (e, mut x, mut sprite, mut tr) in &mut q {
        x.t += dt;
        let p = (x.t / x.max_t).clamp(0.0, 1.0);
        let sz = x.size * (1.0 + p * 0.8);
        sprite.custom_size = Some(Vec2::splat(sz));
        let col = if p < 0.4 {
            COLOR_EXPL_HOT
        } else if p < 0.75 {
            COLOR_EXPL_MID
        } else {
            COLOR_EXPL_OUT
        };
        let mut c = col.to_srgba();
        c.alpha = 1.0 - p;
        sprite.color = Color::Srgba(c);
        tr.scale = Vec3::splat(1.0);
        if x.t >= x.max_t {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：玩家死亡 / 复活 ==========
pub(super) fn contra_player_respawn(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    stage: Res<ContraStage>,
    mut player_q: Query<
        (&mut ContraPlayer, &mut Transform, &mut Sprite),
        Without<Camera>,
    >,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraPlayer>)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut tr, mut sprite)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= dt;
        if player.dead_timer <= 0.0 {
            if session.lives <= 0 {
                session.finished = true;
                session.won = false;
                session.status = "GAME OVER  Enter 重玩 / Backspace 返回菜单".to_string();
                let idx = session.kind.index();
                if session.score > save.high_scores[idx] {
                    save.high_scores[idx] = session.score;
                }
                save.store();
                return;
            }
            // 在当前镜头内从天而降，但被 BOSS 卡墙时落到 BOSS 前
            let cam_x = cam_q
                .single()
                .map(|t| t.translation.x)
                .unwrap_or(stage.player_spawn.x);
            let mut x = (cam_x - ARENA_W * 0.35).max(stage.player_spawn.x);
            if stage.boss_spawned && !stage.boss_dead {
                x = x.min(stage.boss_x - BOSS_W * 0.5 - PLAYER_W * 0.5 - 16.0);
            }
            tr.translation.x = x;
            tr.translation.y = ARENA_H * 0.5 - 40.0;
            player.vel = Vec2::ZERO;
            player.dead_timer = 0.0;
            player.invincible = INVINCIBLE_TIME;
            player.weapon = Weapon::M;
            player.fire_cd = 0.0;
            player.prone = false;
            sprite.custom_size = Some(player_size(false));
        }
    }
}

// ========== 系统：HUD 更新 ==========
pub(super) fn contra_hud_update(
    session: Res<GameSession>,
    stage: Res<ContraStage>,
    player_q: Query<&ContraPlayer>,
    boss_q: Query<&ContraBoss>,
    mut hud_text_q: Query<
        (&mut Text2d, &mut TextColor, &ContraHud),
        Without<Sprite>,
    >,
    mut hud_sprite_q: Query<(&mut Sprite, &ContraHud), Without<Text2d>>,
    mut life_icons_q: Query<(&ContraHudLifeIcon, &mut Visibility)>,
) {
    let weapon = player_q.single().map(|p| p.weapon).unwrap_or(Weapon::M);
    let boss_hp = boss_q.single().ok().map(|b| (b.hp, b.die_t));
    let top_shown = stage.top_score.max(session.score);

    for (mut t, mut tc, hud) in &mut hud_text_q {
        match hud.kind {
            ContraHudKind::Score => {
                *t = Text2d::new(format!("1P-{:06}", session.score));
                tc.0 = Color::srgb(1.0, 0.95, 0.30);
            }
            ContraHudKind::TopScore => {
                *t = Text2d::new(format!("TOP-{:06}", top_shown));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::Lives => {
                *t = Text2d::new(format!("x{}", session.lives.max(0)));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::WeaponLetter => {
                *t = Text2d::new(weapon.letter().to_string());
                tc.0 = Color::srgb(0.0, 0.0, 0.0);
            }
            ContraHudKind::World => {
                *t = Text2d::new(format!("STAGE 1-{}", stage.level));
                tc.0 = Color::srgb(1.0, 1.0, 1.0);
            }
            ContraHudKind::BossHp => match boss_hp {
                Some((hp, die_t)) if die_t <= 0.0 => {
                    *t = Text2d::new(format!("BOSS {:>2}", hp.max(0)));
                    tc.0 = Color::srgb(1.0, 0.4, 0.32);
                }
                _ => *t = Text2d::new(String::new()),
            },
            ContraHudKind::Status => {
                if session.finished {
                    *t = Text2d::new(session.status.clone());
                    tc.0 = Color::srgb(1.0, 0.95, 0.30);
                } else if session.paused {
                    *t = Text2d::new("PAUSED  Esc 继续 / Backspace 返回".to_string());
                    tc.0 = Color::srgb(1.0, 1.0, 1.0);
                } else {
                    *t = Text2d::new(String::new());
                }
            }
            ContraHudKind::Weapon => {} // Sprite 元素，不在文本路径处理
        }
    }

    for (mut sprite, hud) in &mut hud_sprite_q {
        if hud.kind == ContraHudKind::Weapon {
            sprite.color = weapon.pickup_color();
        }
    }

    // 命数小头像：根据剩余命数控制可见性（idx < lives 才显示）
    let lives = session.lives.max(0);
    for (icon, mut vis) in &mut life_icons_q {
        *vis = if icon.idx < lives {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
