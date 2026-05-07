use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::common::render::{UiFont, text};

use super::model::*;

// ========== 常量 ==========
// 1 个 tile = 32 像素，关卡纵向 14 行（0 = 底, 13 = 顶）
pub(super) const TILE: f32 = 32.0;
// 地砖底排中心 y（让最底排在画面底部留出 8px 边距）
pub(super) const FLOOR_Y: f32 = -ARENA_H * 0.5 + TILE * 0.5 + 8.0;

// 玩家（小马里奥）
pub(super) const PLAYER_W: f32 = 22.0;
pub(super) const PLAYER_H: f32 = 28.0;
pub(super) const BIG_PLAYER_H: f32 = 48.0;
pub(super) const WALK_SPEED: f32 = 150.0;
pub(super) const RUN_SPEED: f32 = 260.0;
pub(super) const ACCEL: f32 = 700.0;
pub(super) const DECEL: f32 = 900.0;
pub(super) const AIR_ACCEL: f32 = 500.0;
pub(super) const GRAVITY: f32 = 1700.0;
pub(super) const JUMP_HOLD_GRAVITY: f32 = 700.0;
pub(super) const JUMP_VEL_BASE: f32 = 520.0;
pub(super) const JUMP_VEL_BONUS: f32 = 80.0; // 跑跳加成
pub(super) const FALL_MAX: f32 = 700.0;
pub(super) const STOMP_BOUNCE: f32 = 380.0;

// Goomba
pub(super) const GOOMBA_W: f32 = 26.0;
pub(super) const GOOMBA_H: f32 = 26.0;
pub(super) const GOOMBA_SPEED: f32 = 55.0;
pub(super) const GOOMBA_SQUASH_TIME: f32 = 0.45;

// 道具
pub(super) const POWERUP_SIZE: f32 = 26.0;
pub(super) const MUSHROOM_SPEED: f32 = 90.0;
pub(super) const POWERUP_EMERGE_SPEED: f32 = 36.0;

// 火球
pub(super) const FIREBALL_SIZE: f32 = 12.0;
pub(super) const FIREBALL_SPEED: f32 = 380.0;
pub(super) const FIREBALL_BOUNCE: f32 = 320.0;
pub(super) const FIREBALL_LIFE: f32 = 2.0;
pub(super) const FIRE_CD: f32 = 0.25;

// 砖块碎片
pub(super) const SHARD_SIZE: f32 = 10.0;
pub(super) const SHARD_LIFE: f32 = 1.0;

// Koopa
pub(super) const KOOPA_W: f32 = 26.0;
pub(super) const KOOPA_H: f32 = 38.0;
pub(super) const KOOPA_SPEED: f32 = 50.0;
pub(super) const SHELL_SPEED: f32 = 360.0;
pub(super) const SHELL_W: f32 = 26.0;
pub(super) const SHELL_H: f32 = 24.0;

// 移动平台
pub(super) const PLATFORM_W: f32 = 84.0;
pub(super) const PLATFORM_H: f32 = 12.0;
pub(super) const PLATFORM_SPEED: f32 = 80.0;
pub(super) const PLATFORM_RANGE: f32 = 120.0;

// Bowser
pub(super) const BOWSER_W: f32 = 56.0;
pub(super) const BOWSER_H: f32 = 60.0;
pub(super) const BOWSER_SPEED: f32 = 40.0;
pub(super) const BOWSER_HP: i32 = 5;
pub(super) const BOWSER_FIRE_CD: f32 = 2.5;
pub(super) const BOWSER_FIREBALL_SPEED: f32 = 240.0;

// 斧头（通关道具）
pub(super) const AXE_SIZE: f32 = 24.0;
// Lava
pub(super) const LAVA_DAMAGE: f32 = 0.0; // 占位

// 摄像机
pub(super) const CAMERA_FOLLOW_OFFSET: f32 = 0.0;

// 关卡
pub(super) const LEVEL_TIME: f32 = 300.0;
pub(super) const FALL_DEATH_Y: f32 = -ARENA_H * 0.5 - 80.0;

// Z 层
pub(super) const Z_BG_FAR: f32 = -8.0;
pub(super) const Z_BG_MID: f32 = -6.0;
pub(super) const Z_TILE: f32 = 0.4;
pub(super) const Z_PIPE: f32 = 0.5;
pub(super) const Z_DECOR: f32 = 0.8;
pub(super) const Z_ENEMY: f32 = 1.0;
pub(super) const Z_PLAYER: f32 = 1.2;
pub(super) const Z_FLAG: f32 = 0.6;
pub(super) const Z_COIN: f32 = 1.4;
pub(super) const Z_POWERUP: f32 = 1.05;
pub(super) const Z_FIREBALL: f32 = 1.3;
pub(super) const Z_SHARD: f32 = 1.5;

// FC 风格调色板
const COLOR_SKY: Color = Color::srgb(0.36, 0.58, 0.99);
const COLOR_GROUND_TOP: Color = Color::srgb(0.91, 0.55, 0.20);
const COLOR_GROUND_DARK: Color = Color::srgb(0.66, 0.31, 0.06);
const COLOR_GROUND_LIGHT: Color = Color::srgb(0.97, 0.71, 0.36);
const COLOR_BRICK: Color = Color::srgb(0.78, 0.34, 0.10);
const COLOR_BRICK_DARK: Color = Color::srgb(0.48, 0.18, 0.02);
const COLOR_QUESTION: Color = Color::srgb(0.95, 0.65, 0.10);
const COLOR_QUESTION_DARK: Color = Color::srgb(0.66, 0.42, 0.03);
const COLOR_QUESTION_USED: Color = Color::srgb(0.55, 0.36, 0.02);
const COLOR_PIPE: Color = Color::srgb(0.05, 0.66, 0.16);
const COLOR_PIPE_DARK: Color = Color::srgb(0.02, 0.39, 0.05);
const COLOR_PIPE_LIGHT: Color = Color::srgb(0.45, 0.94, 0.40);
const COLOR_CLOUD: Color = Color::srgb(0.97, 0.98, 1.00);
const COLOR_HILL: Color = Color::srgb(0.05, 0.62, 0.10);
const COLOR_HILL_DARK: Color = Color::srgb(0.02, 0.42, 0.05);
const COLOR_BUSH: Color = Color::srgb(0.05, 0.62, 0.10);
const COLOR_FLAG_POLE: Color = Color::srgb(0.83, 0.86, 0.86);
const COLOR_FLAG: Color = Color::srgb(0.18, 0.78, 0.18);
const COLOR_CASTLE: Color = Color::srgb(0.55, 0.55, 0.62);
const COLOR_CASTLE_DARK: Color = Color::srgb(0.34, 0.34, 0.40);
const COLOR_COIN: Color = Color::srgb(1.00, 0.85, 0.20);

// Mario 像素艺术（小马里奥）颜色
const COLOR_M_RED: Color = Color::srgb(0.91, 0.18, 0.10);
const COLOR_M_SKIN: Color = Color::srgb(0.99, 0.78, 0.55);
const COLOR_M_BROWN: Color = Color::srgb(0.43, 0.20, 0.04);
const COLOR_M_BLUE: Color = Color::srgb(0.16, 0.36, 0.92);
const COLOR_M_BLACK: Color = Color::srgb(0.06, 0.06, 0.10);

// Goomba 像素艺术颜色
const COLOR_G_BROWN: Color = Color::srgb(0.74, 0.45, 0.16);
const COLOR_G_DARK: Color = Color::srgb(0.43, 0.20, 0.04);
const COLOR_G_LIGHT: Color = Color::srgb(0.99, 0.78, 0.55);

// 火力马里奥配色
const COLOR_M_WHITE: Color = Color::srgb(0.98, 0.98, 0.98);
const COLOR_M_FIRE_RED: Color = Color::srgb(0.91, 0.18, 0.10);

// 蘑菇 / 火花
const COLOR_MUSHROOM_RED: Color = Color::srgb(0.92, 0.20, 0.18);
const COLOR_MUSHROOM_SPOT: Color = Color::srgb(0.99, 0.99, 0.99);
const COLOR_MUSHROOM_STEM: Color = Color::srgb(0.99, 0.86, 0.65);
const COLOR_FLOWER_RED: Color = Color::srgb(0.95, 0.20, 0.18);
const COLOR_FLOWER_YELLOW: Color = Color::srgb(0.98, 0.86, 0.18);
const COLOR_FLOWER_GREEN: Color = Color::srgb(0.10, 0.74, 0.18);
const COLOR_ONEUP_GREEN: Color = Color::srgb(0.18, 0.78, 0.20);
const COLOR_FIREBALL_CORE: Color = Color::srgb(1.0, 0.85, 0.20);
const COLOR_FIREBALL_EDGE: Color = Color::srgb(0.95, 0.35, 0.10);

// Koopa
const COLOR_KOOPA_GREEN: Color = Color::srgb(0.20, 0.78, 0.18);
const COLOR_KOOPA_GREEN_DARK: Color = Color::srgb(0.05, 0.46, 0.08);
const COLOR_KOOPA_RED: Color = Color::srgb(0.95, 0.20, 0.18);
const COLOR_KOOPA_RED_DARK: Color = Color::srgb(0.55, 0.08, 0.05);
const COLOR_KOOPA_BELLY: Color = Color::srgb(0.99, 0.86, 0.55);
const COLOR_SHELL_RIM: Color = Color::srgb(0.99, 0.78, 0.18);

// 城堡 / 岩浆
const COLOR_DARK_BRICK: Color = Color::srgb(0.30, 0.30, 0.34);
const COLOR_DARK_BRICK_LIGHT: Color = Color::srgb(0.46, 0.46, 0.52);
const COLOR_DARK_BRICK_DARK: Color = Color::srgb(0.16, 0.16, 0.20);
const COLOR_LAVA: Color = Color::srgb(0.95, 0.30, 0.10);
const COLOR_LAVA_BRIGHT: Color = Color::srgb(1.0, 0.78, 0.25);
const COLOR_PLATFORM: Color = Color::srgb(0.90, 0.62, 0.18);
const COLOR_PLATFORM_DARK: Color = Color::srgb(0.55, 0.34, 0.04);

// Bowser
const COLOR_BOWSER_BODY: Color = Color::srgb(0.05, 0.66, 0.16);
const COLOR_BOWSER_DARK: Color = Color::srgb(0.02, 0.39, 0.05);
const COLOR_BOWSER_BELLY: Color = Color::srgb(0.99, 0.86, 0.55);
const COLOR_BOWSER_HORN: Color = Color::srgb(0.99, 0.92, 0.55);
const COLOR_BOWSER_HAIR: Color = Color::srgb(0.92, 0.20, 0.10);

// 斧头
const COLOR_AXE_HANDLE: Color = Color::srgb(0.62, 0.42, 0.18);
const COLOR_AXE_HEAD: Color = Color::srgb(0.78, 0.78, 0.85);

// 月亮
const COLOR_MOON: Color = Color::srgb(0.99, 0.97, 0.78);

// ========== 关卡数据 ==========
// 字符约定：
//   ' ' 空气
//   'G' 地砖
//   'B' 砖块（可破坏）
//   'Q' 问号砖（含金币）
//   'S' 实心石砖（不可破坏，用于阶梯）
//   'p' 管道单 tile（高 2 行拼出 2x2 管道）
//   'F' 旗杆位置标记（实际旗杆由 spawn_flag 绘制）
//   'M' 玩家出生点
//   'C' 城堡
//   'g' Goomba 出生点
const LEVEL_COLS: i32 = 160;
const LEVEL_ROWS: i32 = 14;

fn level_cols() -> i32 {
    LEVEL_COLS
}

fn build_level_1_1() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    // 地砖（最底行）
    for c in 0..cols {
        g[13][c] = 'G';
    }
    // 落坑：去掉两段地砖
    for c in 84..86 {
        g[13][c] = ' ';
    }
    for c in 124..126 {
        g[13][c] = ' ';
    }

    // 玩家、敌人、管道身、城堡（倒数第 2 行）
    g[12][1] = 'M';
    for c in [14usize, 28, 50, 52, 110, 154] {
        g[12][c] = 'g';
    }
    for top in [66usize, 76, 86, 96] {
        g[12][top] = 'p';
        g[12][top + 1] = 'p';
        g[11][top] = 'p';
        g[11][top + 1] = 'p';
    }
    g[12][146] = 'C';

    // 问号砖 + 砖块（中段悬浮）
    g[10][30] = 'Q'; // 金币
    g[10][36] = 'B';
    g[10][38] = 'U'; // 蘑菇 / 火花
    g[10][40] = 'B';
    // 第二组悬浮砖
    g[10][101] = 'B';
    g[10][103] = 'U'; // 蘑菇 / 火花
    g[10][105] = 'B';

    // 阶梯（向上堆 4 阶）
    g[9][131] = 'S';
    g[9][132] = 'S';
    g[9][133] = 'S';
    g[9][134] = 'S';
    g[8][132] = 'S';
    g[8][133] = 'S';
    g[8][134] = 'S';
    g[7][133] = 'S';
    g[7][134] = 'S';
    g[6][134] = 'S';

    // 旗杆位置标记
    g[7][138] = 'F';

    g
}

// ========== 组件 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Facing {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum PowerState {
    Small,
    Big,
    Fire,
}

impl PowerState {
    pub(super) fn size(self) -> Vec2 {
        match self {
            PowerState::Small => Vec2::new(PLAYER_W, PLAYER_H),
            PowerState::Big | PowerState::Fire => Vec2::new(PLAYER_W + 2.0, BIG_PLAYER_H),
        }
    }
}

#[derive(Component)]
pub(super) struct MarioPlayer {
    pub vel: Vec2,
    pub on_ground: bool,
    pub jumping: bool,
    pub facing: Facing,
    pub invincible: f32, // 受伤无敌
    pub dead_timer: f32, // > 0 表示死亡动画中
    pub finished: bool,  // 已触旗杆
    pub state: PowerState,
    pub transform_t: f32, // 变身动画计时
    pub fire_cd: f32,     // 火球冷却
}

#[derive(Component)]
pub(super) struct MarioVisual; // 玩家身上的像素小方块

#[derive(Component)]
pub(super) struct MarioBackground; // 跟随相机的纯色背景

#[derive(Component, Clone, Copy)]
pub(super) struct Solid {
    pub size: Vec2,
}

#[derive(Component)]
pub(super) struct GroundTile;

#[derive(Component)]
pub(super) struct BrickTile;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)] // Star / OneUp 预留给第三阶段
pub(super) enum QuestionContent {
    Coin,
    Mushroom, // Small → Mushroom; Big/Fire → FireFlower
    Star,
    OneUp,
}

#[derive(Component)]
pub(super) struct QuestionBlock {
    pub used: bool,
    pub bump_t: f32, // 顶起动画
    pub base_y: f32,
    pub content: QuestionContent,
    pub spawn_done: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum PowerUpKind {
    Mushroom,
    FireFlower,
    OneUp,
    Star,
}

#[derive(Component)]
pub(super) struct PowerUp {
    pub kind: PowerUpKind,
    pub vel: Vec2,
    pub emerge_y_target: f32, // 从砖里升起到的目标 y
    pub emerged: bool,
    pub on_ground: bool,
}

#[derive(Component)]
pub(super) struct Fireball {
    pub vel: Vec2,
    pub life: f32,
}

#[derive(Component)]
pub(super) struct BrickShard {
    pub vel: Vec2,
    pub life: f32,
    pub spin: f32,
}

// ===== 第三阶段新增组件 =====

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum KoopaKind {
    Green,
    Red,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum KoopaState {
    Walking,
    Shell,        // 静止龟壳
    SlidingShell, // 被踢的龟壳，高速滑动
}

#[derive(Component)]
pub(super) struct Koopa {
    pub kind: KoopaKind,
    pub vel: Vec2,
    pub on_ground: bool,
    pub state: KoopaState,
    pub state_t: f32, // 进入 Shell 后的计时
    pub dead: bool,
}

#[derive(Component)]
pub(super) struct KoopaVisual;

#[derive(Component)]
pub(super) struct MovingPlatform {
    pub vertical: bool,
    pub center: Vec2,
    pub vel: Vec2,
    pub last_dx: f32,
    pub last_dy: f32,
}

#[derive(Component)]
pub(super) struct LavaTile;

#[derive(Component)]
pub(super) struct DarkBrick;

#[derive(Component)]
pub(super) struct Bowser {
    pub vel: Vec2,
    pub on_ground: bool,
    pub hp: i32,
    pub fire_cd: f32,
    pub dir_t: f32,
    pub dead: bool,
}

#[derive(Component)]
pub(super) struct BowserFireball {
    pub vel: Vec2,
    pub life: f32,
}

#[derive(Component)]
pub(super) struct Axe;

#[derive(Component)]
pub(super) struct StoneTile;

#[derive(Component)]
pub(super) struct PipeTile;

#[derive(Component)]
pub(super) struct Goomba {
    pub vel: Vec2,
    pub on_ground: bool,
    pub squashed: f32, // 被踩后倒计时
    pub dead: bool,
}

#[derive(Component)]
pub(super) struct GoombaVisual;

#[derive(Component)]
pub(super) struct CoinPopup {
    pub timer: f32,
    pub vy: f32,
}

#[derive(Component)]
pub(super) struct FlagPole;

#[derive(Component)]
pub(super) struct FlagBanner {
    pub y_target: f32,
    pub speed: f32,
}

#[derive(Component)]
pub(super) struct MarioHud {
    pub kind: HudKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum HudKind {
    Score,
    Coins,
    Time,
    World,
    Status,
}

// ========== 资源 ==========
#[derive(Resource)]
pub(super) struct MarioStage {
    pub time_left: f32,
    pub coins: u32,
    pub level: u8,
    pub finish_timer: f32,
    pub player_spawn: Vec2,
}

// ========== 工具 ==========
fn aabb_overlap(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() * 2.0 < a_size.x + b_size.x
        && (a_pos.y - b_pos.y).abs() * 2.0 < a_size.y + b_size.y
}

fn tile_center(col: i32, row_from_bottom: i32) -> Vec2 {
    Vec2::new(
        col as f32 * TILE + TILE * 0.5,
        FLOOR_Y + row_from_bottom as f32 * TILE,
    )
}

fn level_world_max_x() -> f32 {
    level_cols() as f32 * TILE
}

// ========== 关卡建图 ==========
pub(super) fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    // 背景色
    let bg_color = match level {
        2 => Color::srgb(0.02, 0.02, 0.03), // 地下黑
        3 => Color::srgb(0.13, 0.20, 0.42), // 夜空（高空）
        4 => Color::srgb(0.10, 0.04, 0.04), // 城堡暗红
        _ => COLOR_SKY,
    };
    commands.spawn((
        Sprite::from_color(bg_color, Vec2::new(ARENA_W * 1.4, ARENA_H * 1.4)),
        Transform::from_translation(Vec3::new(0.0, 0.0, Z_BG_FAR)),
        MarioBackground,
        GameEntity,
    ));

    // 远景装饰仅在地表关
    if level == 1 {
        spawn_background_decor(commands);
    } else if level == 3 {
        // 1-3：高空云 + 月亮
        spawn_high_sky_decor(commands);
    }

    // 解析关卡 ASCII
    let mut player_spawn = Vec2::new(2.0 * TILE, FLOOR_Y + TILE);
    let mut goomba_spawns: Vec<Vec2> = Vec::new();
    let mut koopa_green_spawns: Vec<Vec2> = Vec::new();
    let mut koopa_red_spawns: Vec<Vec2> = Vec::new();
    let mut bowser_spawn: Option<Vec2> = None;
    let mut flag_col = 0i32;
    let mut axe_pos: Option<Vec2> = None;
    let map = match level {
        2 => build_level_1_2(),
        3 => build_level_1_3(),
        4 => build_level_1_4(),
        _ => build_level_1_1(),
    };
    let h = map.len() as i32;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            let col = col_idx as i32;
            let row_from_bottom = h - 1 - row_idx as i32;
            let pos = tile_center(col, row_from_bottom);
            match ch {
                'G' => spawn_ground(commands, pos),
                'B' => spawn_brick(commands, pos),
                'Q' => spawn_question(commands, pos, QuestionContent::Coin),
                'U' => spawn_question(commands, pos, QuestionContent::Mushroom),
                '*' => spawn_question(commands, pos, QuestionContent::Star),
                '1' => spawn_question(commands, pos, QuestionContent::OneUp),
                'S' => spawn_stone(commands, pos),
                'D' => spawn_dark_block(commands, pos), // 城堡石砖
                'p' => spawn_pipe_tile(commands, pos),
                'l' => spawn_lava(commands, pos),
                'L' => spawn_moving_platform(commands, pos, true), // 上下移动
                'H' => spawn_moving_platform(commands, pos, false), // 左右移动
                'F' => {
                    flag_col = col;
                }
                'A' => {
                    axe_pos = Some(pos);
                }
                'M' => {
                    player_spawn = Vec2::new(pos.x, pos.y + (PLAYER_H - TILE) * 0.5);
                }
                'g' => {
                    goomba_spawns.push(Vec2::new(pos.x, pos.y - (TILE - GOOMBA_H) * 0.5));
                }
                'k' => {
                    koopa_green_spawns
                        .push(Vec2::new(pos.x, pos.y - (TILE - KOOPA_H) * 0.5 + 6.0));
                }
                'r' => {
                    koopa_red_spawns
                        .push(Vec2::new(pos.x, pos.y - (TILE - KOOPA_H) * 0.5 + 6.0));
                }
                'b' => {
                    bowser_spawn = Some(Vec2::new(pos.x, pos.y));
                }
                'C' => spawn_castle(commands, pos),
                _ => {}
            }
        }
    }

    // 旗杆 + 旗帜
    if flag_col != 0 {
        spawn_flag(commands, flag_col);
    }
    // 城堡斧头（1-4）
    if let Some(ap) = axe_pos {
        spawn_axe(commands, ap);
    }

    // 玩家
    spawn_player(commands, player_spawn);

    // 敌人
    for p in goomba_spawns {
        spawn_goomba(commands, p);
    }
    for p in koopa_green_spawns {
        spawn_koopa(commands, p, KoopaKind::Green);
    }
    for p in koopa_red_spawns {
        spawn_koopa(commands, p, KoopaKind::Red);
    }
    if let Some(p) = bowser_spawn {
        spawn_bowser(commands, p);
    }

    // HUD
    spawn_hud(commands, font, level);

    commands.insert_resource(MarioStage {
        time_left: LEVEL_TIME,
        coins: 0,
        level,
        finish_timer: 0.0,
        player_spawn,
    });
}

fn spawn_background_decor(commands: &mut Commands) {
    // 几朵云（fixed pos in world）
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
    // 山
    let hill_positions: [(f32, bool); 4] = [
        (520.0, true),
        (1900.0, false),
        (2700.0, true),
        (4100.0, false),
    ];
    for (x, big) in hill_positions {
        spawn_hill(commands, x, big);
    }
    // 灌木
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

fn spawn_cloud(commands: &mut Commands, center: Vec2) {
    // 简单 3 块 + 顶部 2 块的像素云
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

fn spawn_hill(commands: &mut Commands, x_center: f32, big: bool) {
    let base_y = FLOOR_Y + TILE * 0.5;
    let scale = if big { 1.4 } else { 1.0 };
    // 用 4 层不同宽度的矩形堆出三角形山
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
    // 山顶亮点
    let tip_y = base_y + (layers as f32) * 18.0 * scale - 6.0;
    commands.spawn((
        Sprite::from_color(COLOR_CLOUD, Vec2::new(8.0, 6.0)),
        Transform::from_translation(Vec3::new(x_center - 6.0, tip_y, Z_BG_MID + 0.3)),
        GameEntity,
    ));
}

fn spawn_bush(commands: &mut Commands, x_center: f32, segments: i32) {
    let base_y = FLOOR_Y + TILE * 0.5 + 6.0;
    let total_w = (segments as f32) * 32.0;
    commands.spawn((
        Sprite::from_color(COLOR_BUSH, Vec2::new(total_w, 18.0)),
        Transform::from_translation(Vec3::new(x_center, base_y, Z_BG_MID + 0.4)),
        GameEntity,
    ));
    // 顶部小圆头
    for i in 0..segments {
        let dx = -total_w * 0.5 + 16.0 + (i as f32) * 32.0;
        commands.spawn((
            Sprite::from_color(COLOR_BUSH, Vec2::new(22.0, 14.0)),
            Transform::from_translation(Vec3::new(x_center + dx, base_y + 14.0, Z_BG_MID + 0.4)),
            GameEntity,
        ));
    }
    // 高光
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_LIGHT, Vec2::new(total_w * 0.7, 3.0)),
        Transform::from_translation(Vec3::new(x_center - 4.0, base_y + 5.0, Z_BG_MID + 0.5)),
        GameEntity,
    ));
}

fn spawn_ground(commands: &mut Commands, center: Vec2) {
    // 实体 collider
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_TOP, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        Solid { size: Vec2::splat(TILE) },
        GroundTile,
        GameEntity,
    ));
    // 顶部高光线
    commands.spawn((
        Sprite::from_color(COLOR_GROUND_LIGHT, Vec2::new(TILE, 4.0)),
        Transform::from_translation((center + Vec2::new(0.0, TILE * 0.5 - 2.0)).extend(Z_TILE + 0.05)),
        GameEntity,
    ));
    // 暗色斑点
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

fn spawn_brick(commands: &mut Commands, center: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(COLOR_BRICK, Vec2::splat(TILE)),
            Transform::from_translation(center.extend(Z_TILE)),
            Solid { size: Vec2::splat(TILE) },
            BrickTile,
            GameEntity,
        ))
        .id();
    // 砖纹（4 道暗线）
    for i in 0..2 {
        let y_off = -TILE * 0.5 + 8.0 + (i as f32) * 16.0;
        commands
            .spawn((
                Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(TILE, 2.0)),
                Transform::from_translation((center + Vec2::new(0.0, y_off)).extend(Z_TILE + 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
    // 竖直砖缝错位
    commands
        .spawn((
            Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(2.0, 14.0)),
            Transform::from_translation((center + Vec2::new(-8.0, 8.0)).extend(Z_TILE + 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    commands
        .spawn((
            Sprite::from_color(COLOR_BRICK_DARK, Vec2::new(2.0, 14.0)),
            Transform::from_translation((center + Vec2::new(8.0, -8.0)).extend(Z_TILE + 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
}

fn spawn_question(commands: &mut Commands, center: Vec2, content: QuestionContent) {
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
    // 边框
    commands
        .spawn((
            Sprite::from_color(COLOR_QUESTION_DARK, Vec2::new(TILE, 4.0)),
            Transform::from_translation((center + Vec2::new(0.0, TILE * 0.5 - 2.0)).extend(Z_TILE + 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    commands
        .spawn((
            Sprite::from_color(COLOR_QUESTION_DARK, Vec2::new(TILE, 4.0)),
            Transform::from_translation((center + Vec2::new(0.0, -TILE * 0.5 + 2.0)).extend(Z_TILE + 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    // ? 像素艺术（用 5 个小方块拼）
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
                Transform::from_translation((center + Vec2::new(dx, dy)).extend(Z_TILE + 0.06)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

fn spawn_stone(commands: &mut Commands, center: Vec2) {
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

fn spawn_pipe_tile(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_PIPE, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_PIPE)),
        Solid { size: Vec2::splat(TILE) },
        PipeTile,
        GameEntity,
    ));
    // 高光带
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_LIGHT, Vec2::new(4.0, TILE - 4.0)),
        Transform::from_translation((center + Vec2::new(-TILE * 0.5 + 6.0, 0.0)).extend(Z_PIPE + 0.05)),
        GameEntity,
    ));
    // 暗影带
    commands.spawn((
        Sprite::from_color(COLOR_PIPE_DARK, Vec2::new(4.0, TILE - 4.0)),
        Transform::from_translation((center + Vec2::new(TILE * 0.5 - 6.0, 0.0)).extend(Z_PIPE + 0.05)),
        GameEntity,
    ));
}

fn spawn_castle(commands: &mut Commands, base_center: Vec2) {
    // 城堡底中心放在地砖上方一点
    let cx = base_center.x;
    let cy = base_center.y + 32.0;
    // 主体
    commands.spawn((
        Sprite::from_color(COLOR_CASTLE, Vec2::new(112.0, 96.0)),
        Transform::from_translation(Vec3::new(cx, cy, Z_DECOR)),
        GameEntity,
    ));
    // 顶部凹齿
    for i in 0..5 {
        let dx = -48.0 + (i as f32) * 24.0;
        commands.spawn((
            Sprite::from_color(COLOR_CASTLE, Vec2::new(16.0, 12.0)),
            Transform::from_translation(Vec3::new(cx + dx, cy + 54.0, Z_DECOR)),
            GameEntity,
        ));
    }
    // 中间塔
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
    // 城门
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
    // 城窗
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

fn spawn_flag(commands: &mut Commands, col: i32) {
    let pole_x = col as f32 * TILE + TILE * 0.5;
    // 旗杆从地砖上方一直到顶部第 5 行
    let pole_bottom_y = FLOOR_Y + TILE; // 地砖正上方
    let pole_top_y = FLOOR_Y + 10.0 * TILE;
    let pole_h = pole_top_y - pole_bottom_y;
    let pole_cy = (pole_top_y + pole_bottom_y) * 0.5;
    commands.spawn((
        Sprite::from_color(COLOR_FLAG_POLE, Vec2::new(4.0, pole_h)),
        Transform::from_translation(Vec3::new(pole_x, pole_cy, Z_FLAG)),
        FlagPole,
        GameEntity,
    ));
    // 球顶
    commands.spawn((
        Sprite::from_color(COLOR_FLAG, Vec2::new(12.0, 12.0)),
        Transform::from_translation(Vec3::new(pole_x, pole_top_y + 6.0, Z_FLAG + 0.05)),
        GameEntity,
    ));
    // 旗帜（初始在顶端，触杆后会下落到底部）
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

fn spawn_player(commands: &mut Commands, pos: Vec2) {
    let state = PowerState::Small;
    let size = state.size();
    let player = commands
        .spawn((
            // 身体框（透明），纯靠子节点拼像素艺术
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), size),
            Transform::from_translation(pos.extend(Z_PLAYER)),
            MarioPlayer {
                vel: Vec2::ZERO,
                on_ground: false,
                jumping: false,
                facing: Facing::Right,
                invincible: 0.0,
                dead_timer: 0.0,
                finished: false,
                state,
                transform_t: 0.0,
                fire_cd: 0.0,
            },
            GameEntity,
        ))
        .id();
    build_player_visual(commands, player, state);
}

fn build_player_visual(commands: &mut Commands, player: Entity, state: PowerState) {
    let parts: Vec<(f32, f32, f32, f32, Color)> = match state {
        PowerState::Small => small_mario_parts().to_vec(),
        PowerState::Big => big_mario_parts(false),
        PowerState::Fire => big_mario_parts(true),
    };
    for (dx, dy, w, h, c) in parts {
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                MarioVisual,
                GameEntity,
            ))
            .insert(ChildOf(player));
    }
}

fn small_mario_parts() -> [(f32, f32, f32, f32, Color); 18] {
    // 小马里奥（22w × 28h）
    [
        // 帽子
        (-6.0, 11.0, 14.0, 4.0, COLOR_M_RED),
        (0.0, 13.0, 12.0, 2.0, COLOR_M_RED),
        // 头发/鬓角
        (-8.0, 7.0, 4.0, 4.0, COLOR_M_BROWN),
        (8.0, 7.0, 2.0, 4.0, COLOR_M_BROWN),
        // 脸
        (-2.0, 7.0, 8.0, 4.0, COLOR_M_SKIN),
        (-2.0, 4.0, 10.0, 2.0, COLOR_M_SKIN),
        // 眼睛
        (3.0, 7.0, 2.0, 2.0, COLOR_M_BLACK),
        // 胡子
        (-2.0, 5.0, 8.0, 1.0, COLOR_M_BROWN),
        // 上衣（红色）
        (-4.0, 0.0, 14.0, 4.0, COLOR_M_RED),
        (0.0, 2.0, 6.0, 2.0, COLOR_M_RED),
        // 背带（蓝色）
        (-3.0, 0.0, 2.0, 4.0, COLOR_M_BLUE),
        (3.0, 0.0, 2.0, 4.0, COLOR_M_BLUE),
        // 裤子
        (-4.0, -4.0, 14.0, 4.0, COLOR_M_BLUE),
        (-4.0, -8.0, 6.0, 4.0, COLOR_M_BLUE),
        (4.0, -8.0, 6.0, 4.0, COLOR_M_BLUE),
        // 鞋
        (-5.0, -12.0, 6.0, 2.0, COLOR_M_BROWN),
        (5.0, -12.0, 6.0, 2.0, COLOR_M_BROWN),
        // 手
        (-9.0, 1.0, 3.0, 3.0, COLOR_M_SKIN),
    ]
}

fn big_mario_parts(fire: bool) -> Vec<(f32, f32, f32, f32, Color)> {
    // 大/火力马里奥（24w × 48h）
    let cap = if fire { COLOR_M_WHITE } else { COLOR_M_FIRE_RED };
    let shirt = if fire { COLOR_M_FIRE_RED } else { COLOR_M_RED };
    let overall = if fire { COLOR_M_FIRE_RED } else { COLOR_M_BLUE };
    vec![
        // 帽子（顶部）
        (-7.0, 21.0, 16.0, 4.0, cap),
        (1.0, 23.0, 12.0, 2.0, cap),
        // 帽檐
        (-9.0, 17.0, 6.0, 2.0, cap),
        // 头发/鬓角
        (-9.0, 13.0, 4.0, 4.0, COLOR_M_BROWN),
        (9.0, 13.0, 2.0, 4.0, COLOR_M_BROWN),
        // 脸
        (-2.0, 13.0, 10.0, 6.0, COLOR_M_SKIN),
        (-1.0, 9.0, 12.0, 2.0, COLOR_M_SKIN),
        // 眼睛
        (4.0, 13.0, 2.0, 3.0, COLOR_M_BLACK),
        // 胡子
        (-1.0, 10.0, 10.0, 1.5, COLOR_M_BROWN),
        // 脖子
        (0.0, 7.0, 8.0, 2.0, COLOR_M_SKIN),
        // 上衣
        (-4.0, 3.0, 16.0, 6.0, shirt),
        (0.0, 5.0, 6.0, 2.0, shirt),
        // 背带
        (-3.0, 1.0, 2.0, 6.0, overall),
        (3.0, 1.0, 2.0, 6.0, overall),
        (0.0, 4.0, 2.0, 2.0, COLOR_QUESTION), // 背带扣
        // 上衣袖
        (-10.0, 4.0, 4.0, 5.0, shirt),
        (10.0, 4.0, 4.0, 5.0, shirt),
        // 手
        (-11.0, 1.0, 4.0, 4.0, COLOR_M_SKIN),
        (11.0, 1.0, 4.0, 4.0, COLOR_M_SKIN),
        // 裤子
        (-4.0, -4.0, 16.0, 6.0, overall),
        (-5.0, -10.0, 6.0, 6.0, overall),
        (5.0, -10.0, 6.0, 6.0, overall),
        // 中缝
        (0.0, -6.0, 1.5, 8.0, COLOR_M_BLACK),
        // 鞋
        (-6.0, -16.0, 8.0, 4.0, COLOR_M_BROWN),
        (6.0, -16.0, 8.0, 4.0, COLOR_M_BROWN),
        // 鞋底
        (-6.0, -19.0, 9.0, 2.0, COLOR_M_BLACK),
        (6.0, -19.0, 9.0, 2.0, COLOR_M_BLACK),
    ]
}

fn spawn_goomba(commands: &mut Commands, pos: Vec2) {
    let goomba = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(GOOMBA_W, GOOMBA_H)),
            Transform::from_translation(pos.extend(Z_ENEMY)),
            Goomba {
                vel: Vec2::new(-GOOMBA_SPEED, 0.0),
                on_ground: false,
                squashed: 0.0,
                dead: false,
            },
            GameEntity,
        ))
        .id();
    // 像素艺术：身体（褐色）+ 眼睛（白+黑）+ 脚（深褐）
    let parts: [(f32, f32, f32, f32, Color); 9] = [
        (0.0, 4.0, 22.0, 14.0, COLOR_G_BROWN),
        (0.0, 12.0, 16.0, 4.0, COLOR_G_DARK),
        (-5.0, 6.0, 4.0, 5.0, COLOR_G_LIGHT),
        (5.0, 6.0, 4.0, 5.0, COLOR_G_LIGHT),
        (-5.0, 6.0, 2.0, 3.0, COLOR_M_BLACK),
        (5.0, 6.0, 2.0, 3.0, COLOR_M_BLACK),
        // 牙齿
        (-2.0, 0.0, 2.0, 2.0, COLOR_CLOUD),
        (2.0, 0.0, 2.0, 2.0, COLOR_CLOUD),
        // 脚
        (0.0, -10.0, 22.0, 6.0, COLOR_G_DARK),
    ];
    for (dx, dy, w, h, c) in parts {
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                GoombaVisual,
                GameEntity,
            ))
            .insert(ChildOf(goomba));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    // 顶部 HUD：MARIO score | COINS | WORLD 1-1 | TIME
    let y = ARENA_H * 0.5 - 24.0;
    text(
        commands,
        font,
        "MARIO",
        Vec2::new(-ARENA_W * 0.5 + 70.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let score_e = text(
        commands,
        font,
        "000000",
        Vec2::new(-ARENA_W * 0.5 + 70.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(score_e).insert(MarioHud { kind: HudKind::Score });

    text(
        commands,
        font,
        "★",
        Vec2::new(-100.0, y - 22.0),
        20.0,
        COLOR_COIN,
        GameEntity,
    );
    let coin_e = text(
        commands,
        font,
        "x00",
        Vec2::new(-60.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(coin_e).insert(MarioHud { kind: HudKind::Coins });

    text(
        commands,
        font,
        "WORLD",
        Vec2::new(120.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let world_e = text(
        commands,
        font,
        &format!("1-{}", level.max(1)),
        Vec2::new(120.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(world_e).insert(MarioHud { kind: HudKind::World });

    text(
        commands,
        font,
        "TIME",
        Vec2::new(ARENA_W * 0.5 - 80.0, y),
        16.0,
        Color::srgb(0.95, 0.95, 0.95),
        GameEntity,
    );
    let time_e = text(
        commands,
        font,
        "300",
        Vec2::new(ARENA_W * 0.5 - 80.0, y - 22.0),
        18.0,
        Color::srgb(1.0, 1.0, 1.0),
        GameEntity,
    )
    .id();
    commands.entity(time_e).insert(MarioHud { kind: HudKind::Time });

    // 状态文字（关底通关 / GAME OVER 提示）
    let status_e = text(
        commands,
        font,
        "",
        Vec2::new(0.0, 0.0),
        28.0,
        Color::srgb(1.0, 0.95, 0.4),
        GameEntity,
    )
    .id();
    commands.entity(status_e).insert(MarioHud { kind: HudKind::Status });
}

// ========== 系统：玩家输入 ==========
pub(super) fn mario_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(&mut MarioPlayer, &mut Transform)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut tr)) = q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }

    let left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    let run = keys.pressed(KeyCode::KeyJ) || keys.pressed(KeyCode::ShiftLeft);
    let jump_pressed = keys.just_pressed(KeyCode::KeyK)
        || keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::KeyW)
        || keys.just_pressed(KeyCode::ArrowUp);
    let jump_held = keys.pressed(KeyCode::KeyK)
        || keys.pressed(KeyCode::Space)
        || keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::ArrowUp);

    let max_speed = if run { RUN_SPEED } else { WALK_SPEED };
    let target_x = if left && !right {
        -max_speed
    } else if right && !left {
        max_speed
    } else {
        0.0
    };

    let accel = if player.on_ground {
        if target_x == 0.0 { DECEL } else { ACCEL }
    } else {
        AIR_ACCEL
    };
    let dvx = (target_x - player.vel.x).clamp(-accel * dt, accel * dt);
    player.vel.x += dvx;

    // 跑超出走速时不强行限制（按住跑可以达到 RUN_SPEED）
    if !run && player.vel.x.abs() > WALK_SPEED && player.on_ground {
        let s = player.vel.x.signum();
        player.vel.x = (player.vel.x.abs().min(WALK_SPEED + 20.0)) * s;
    }

    if right {
        player.facing = Facing::Right;
    } else if left {
        player.facing = Facing::Left;
    }

    // 跳跃：起跳速度根据当前水平速度增加（FC 风格的跑跳更高）
    if jump_pressed && player.on_ground {
        let extra = (player.vel.x.abs() / RUN_SPEED) * JUMP_VEL_BONUS;
        player.vel.y = JUMP_VEL_BASE + extra;
        player.on_ground = false;
        player.jumping = true;
    }

    // 按住跳跃 → 上升时降低重力（跳得更高）
    if !jump_held && player.vel.y > 0.0 {
        player.jumping = false;
    }

    // 朝向：通过缩放 X 翻转（保留像素艺术）
    let scale_x = if player.facing == Facing::Right { 1.0 } else { -1.0 };
    tr.scale.x = scale_x;

    // 火球（Fire 状态 + 按 J 一下）
    if player.fire_cd > 0.0 {
        player.fire_cd = (player.fire_cd - dt).max(0.0);
    }
    if matches!(player.state, PowerState::Fire) && keys.just_pressed(KeyCode::KeyJ) && player.fire_cd <= 0.0 {
        let dir = if player.facing == Facing::Right { 1.0 } else { -1.0 };
        let muzzle = Vec2::new(tr.translation.x + dir * 14.0, tr.translation.y + 4.0);
        spawn_fireball(&mut commands, muzzle, dir);
        player.fire_cd = FIRE_CD;
    }

    // 变身闪烁计时
    if player.transform_t > 0.0 {
        player.transform_t = (player.transform_t - dt).max(0.0);
    }
}

// ========== 系统：玩家物理（重力 + 碰撞） ==========
pub(super) fn mario_physics(
    time: Res<Time>,
    session: Res<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &mut Transform), Without<Solid>>,
    solid_q: Query<(Entity, &Transform, &Solid), Without<MarioPlayer>>,
    brick_q: Query<&BrickTile>,
    mut question_q: Query<&mut QuestionBlock>,
    pipe_q: Query<&PipeTile>,
    stone_q: Query<&StoneTile>,
    ground_q: Query<&GroundTile>,
    flag_q: Query<&FlagPole>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((mut player, mut tr)) = player_q.single_mut() else {
        return;
    };

    if player.dead_timer > 0.0 {
        // 死亡动画：先小弹跳，再下落
        player.vel.y -= GRAVITY * dt;
        player.vel.y = player.vel.y.max(-FALL_MAX);
        tr.translation.y += player.vel.y * dt;
        return;
    }

    if player.finished {
        // 通关后向右走一段
        let mut v = player.vel;
        v.x = WALK_SPEED * 0.7;
        v.y -= GRAVITY * dt;
        v.y = v.y.max(-FALL_MAX);
        // 简单 y 碰撞
        let mut np = tr.translation.truncate() + v * dt;
        let p_size = player.state.size();
        let mut on_ground = false;
        for (_e, st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(np, p_size, sp, s.size) {
                let dx = np.x - sp.x;
                let dy = np.y - sp.y;
                let px = (p_size.x + s.size.x) * 0.5 - dx.abs();
                let py = (p_size.y + s.size.y) * 0.5 - dy.abs();
                if px > 0.0 && py > 0.0 {
                    if py < px {
                        if dy > 0.0 {
                            np.y += py;
                            v.y = 0.0;
                            on_ground = true;
                        } else {
                            np.y -= py;
                            v.y = 0.0;
                        }
                    } else if dx > 0.0 {
                        np.x += px;
                        v.x = 0.0;
                    } else {
                        np.x -= px;
                        v.x = 0.0;
                    }
                }
            }
        }
        tr.translation.x = np.x;
        tr.translation.y = np.y;
        player.vel = v;
        player.on_ground = on_ground;
        return;
    }

    // 应用重力（按住跳 + 上升时减半）
    let g = if player.jumping && player.vel.y > 0.0 {
        JUMP_HOLD_GRAVITY
    } else {
        GRAVITY
    };
    player.vel.y -= g * dt;
    player.vel.y = player.vel.y.max(-FALL_MAX);

    let mut pos = tr.translation.truncate();
    let p_size = player.state.size();

    // —— X 轴扫描 ——
    pos.x += player.vel.x * dt;
    // 限制左侧不能退出关卡左边界
    let left_min = PLAYER_W * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let right_max = level_world_max_x() - PLAYER_W * 0.5;
    if pos.x > right_max {
        pos.x = right_max;
    }
    for (_e, st, s) in &solid_q {
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

    // —— Y 轴扫描 ——
    pos.y += player.vel.y * dt;
    let mut on_ground = false;
    let mut hit_above: Option<Entity> = None;
    for (e, st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dy = pos.y - sp.y;
            let push = (p_size.y + s.size.y) * 0.5 - dy.abs();
            if push > 0.0 {
                if dy > 0.0 {
                    // 玩家在上方落到方块上
                    pos.y += push;
                    if player.vel.y < 0.0 {
                        player.vel.y = 0.0;
                    }
                    on_ground = true;
                } else {
                    // 玩家头顶撞块
                    pos.y -= push;
                    if player.vel.y > 0.0 {
                        player.vel.y = 0.0;
                    }
                    hit_above = Some(e);
                }
            }
        }
    }
    player.on_ground = on_ground;

    // 头顶撞块响应：
    //   - 问号砖：触发顶起动画，由 mario_block_anim 派生金币 / 道具
    //   - 砖块：小马里奥下仅顶起视觉，大/火力马里奥击碎（在 mario_brick_break 处理）
    if let Some(e) = hit_above {
        if let Ok(mut q) = question_q.get_mut(e) {
            if !q.used {
                q.used = true;
                q.bump_t = 0.18;
            } else if q.bump_t <= 0.0 {
                q.bump_t = 0.10;
            }
        } else if brick_q.get(e).is_ok() {
            // 由独立的 mario_brick_break 系统处理（基于玩家状态）
        } else if pipe_q.get(e).is_ok() || stone_q.get(e).is_ok() || ground_q.get(e).is_ok() {
            // 不响应
        } else if flag_q.get(e).is_ok() {
            // 旗杆不响应顶
        }
    }

    tr.translation.x = pos.x;
    tr.translation.y = pos.y;

    // 掉出关卡 → 死亡
    if pos.y < FALL_DEATH_Y && player.dead_timer <= 0.0 {
        player.dead_timer = 2.0;
        player.vel = Vec2::new(0.0, 380.0);
    }

    // 无敌时间衰减
    if player.invincible > 0.0 {
        player.invincible -= dt;
    }
}

// ========== 系统：问号砖动画 + 内容生成 ==========
pub(super) fn mario_block_anim(
    time: Res<Time>,
    mut q_blocks: Query<(&mut Transform, &mut QuestionBlock, &mut Sprite, &Children)>,
    mut child_sprites: Query<&mut Sprite, Without<QuestionBlock>>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<&MarioPlayer>,
    mut commands: Commands,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs();
    let player_state = player_q
        .single()
        .ok()
        .map(|p| p.state)
        .unwrap_or(PowerState::Small);

    for (mut tr, mut qb, mut sprite, children) in &mut q_blocks {
        // 顶起小动画
        if qb.bump_t > 0.0 {
            qb.bump_t -= dt;
            let total = 0.18_f32;
            let t = (total - qb.bump_t.max(0.0)) / total;
            let off = (t * std::f32::consts::PI).sin() * 10.0;
            tr.translation.y = qb.base_y + off;
            if qb.bump_t <= 0.0 {
                tr.translation.y = qb.base_y;
            }
        }
        // 已使用 → 涂色为 used
        if qb.used {
            sprite.color = COLOR_QUESTION_USED;
            for c in children {
                if let Ok(mut cs) = child_sprites.get_mut(*c) {
                    if cs.color == COLOR_M_BLACK {
                        cs.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
                    } else if cs.color == COLOR_QUESTION_DARK {
                        cs.color = Color::srgb(0.32, 0.20, 0.02);
                    }
                }
            }
        }
        // 第一次顶起 → 派生内容
        if qb.used && !qb.spawn_done {
            qb.spawn_done = true;
            let block_pos = Vec2::new(tr.translation.x, qb.base_y);
            match qb.content {
                QuestionContent::Coin => {
                    stage.coins += 1;
                    session.score = session.score.saturating_add(200);
                    commands.spawn((
                        Sprite::from_color(COLOR_COIN, Vec2::new(10.0, 14.0)),
                        Transform::from_translation(Vec3::new(
                            block_pos.x,
                            qb.base_y + 32.0,
                            Z_COIN,
                        )),
                        CoinPopup {
                            timer: 0.5,
                            vy: 220.0,
                        },
                        GameEntity,
                    ));
                }
                QuestionContent::Mushroom => {
                    let kind = if matches!(player_state, PowerState::Small) {
                        PowerUpKind::Mushroom
                    } else {
                        PowerUpKind::FireFlower
                    };
                    spawn_powerup(&mut commands, block_pos, kind);
                }
                QuestionContent::Star => {
                    spawn_powerup(&mut commands, block_pos, PowerUpKind::Star);
                }
                QuestionContent::OneUp => {
                    spawn_powerup(&mut commands, block_pos, PowerUpKind::OneUp);
                }
            }
        }
    }
}

pub(super) fn mario_coin_popup(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut CoinPopup)>,
) {
    let dt = time.delta_secs();
    for (e, mut tr, mut c) in &mut q {
        c.timer -= dt;
        c.vy -= 600.0 * dt;
        tr.translation.y += c.vy * dt;
        if c.timer <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：Goomba AI + 物理 ==========
pub(super) fn mario_goomba_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_goomba: Query<(Entity, &mut Goomba, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<Goomba>)>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let g_size = Vec2::new(GOOMBA_W, GOOMBA_H);
    for (e, mut g, mut tr) in &mut q_goomba {
        if g.dead {
            commands.entity(e).despawn();
            continue;
        }
        if g.squashed > 0.0 {
            g.squashed -= dt;
            tr.scale.y = (g.squashed / GOOMBA_SQUASH_TIME).max(0.1);
            if g.squashed <= 0.0 {
                g.dead = true;
            }
            continue;
        }
        // 重力
        g.vel.y -= GRAVITY * dt;
        g.vel.y = g.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();
        // X 轴
        pos.x += g.vel.x * dt;
        let mut bumped_x = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, g_size, sp, s.size) {
                let dx = pos.x - sp.x;
                let push = (g_size.x + s.size.x) * 0.5 - dx.abs();
                if push > 0.0 {
                    if dx > 0.0 {
                        pos.x += push;
                    } else {
                        pos.x -= push;
                    }
                    bumped_x = true;
                }
            }
        }
        if bumped_x {
            g.vel.x = -g.vel.x;
        }
        // Y 轴
        pos.y += g.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, g_size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (g_size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if g.vel.y < 0.0 {
                            g.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if g.vel.y > 0.0 {
                            g.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        g.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        // 掉出关卡 → 删除
        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：玩家与 Goomba 碰撞 ==========
pub(super) fn mario_player_vs_goomba(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<Goomba>>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<MarioPlayer>>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let star = player.invincible > 5.0;
    for (mut g, gtr) in &mut goomba_q {
        if g.dead || g.squashed > 0.0 {
            continue;
        }
        let g_pos = gtr.translation.truncate();
        let g_size = Vec2::new(GOOMBA_W, GOOMBA_H);
        if !aabb_overlap(p_pos, p_size, g_pos, g_size) {
            continue;
        }
        if star {
            g.squashed = GOOMBA_SQUASH_TIME;
            g.vel.x = 0.0;
            session.score = session.score.saturating_add(200);
            continue;
        }
        // 判断踩头：玩家脚底高于 Goomba 中心，且不是上升中（跳起来头撞 Goomba 不算踩）
        let p_foot = p_pos.y - p_size.y * 0.5;
        if p_foot > g_pos.y - 4.0 && player.vel.y <= 30.0 {
            g.squashed = GOOMBA_SQUASH_TIME;
            g.vel.x = 0.0;
            player.vel.y = STOMP_BOUNCE;
            session.score = session.score.saturating_add(100);
        } else if player.invincible <= 0.0 && player.transform_t <= 0.0 {
            // 受伤：根据状态降级或死亡
            match player.state {
                PowerState::Fire => {
                    player.state = PowerState::Big;
                    player.transform_t = 0.6;
                    player.invincible = 1.6;
                    commands
                        .entity(player_e)
                        .despawn_related::<Children>();
                    build_player_visual(&mut commands, player_e, PowerState::Big);
                }
                PowerState::Big => {
                    player.state = PowerState::Small;
                    player.transform_t = 0.6;
                    player.invincible = 1.6;
                    commands
                        .entity(player_e)
                        .despawn_related::<Children>();
                    build_player_visual(&mut commands, player_e, PowerState::Small);
                }
                PowerState::Small => {
                    player.dead_timer = 2.2;
                    player.vel = Vec2::new(0.0, 420.0);
                    session.lives -= 1;
                }
            }
        }
    }
}

// ========== 系统：玩家与旗杆 ==========
pub(super) fn mario_flag_check(
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform)>,
    flag_q: Query<&Transform, (With<FlagPole>, Without<MarioPlayer>)>,
    mut banner_q: Query<&mut FlagBanner>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.finished || player.dead_timer > 0.0 {
        return;
    }
    let Ok(ftr) = flag_q.single() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    if (p_pos.x - ftr.translation.x).abs() < 16.0 && p_pos.y > FLOOR_Y {
        player.finished = true;
        // 触杆得分根据高度
        let h_ratio = ((p_pos.y - FLOOR_Y) / (10.0 * TILE)).clamp(0.0, 1.0);
        let pts = (h_ratio * 5000.0) as u32 + 100;
        session.score = session.score.saturating_add(pts);
        // 旗帜下降
        for mut b in &mut banner_q {
            b.y_target = FLOOR_Y + TILE + 8.0;
            b.speed = 90.0;
        }
    }
}

pub(super) fn mario_flag_anim(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &FlagBanner)>,
) {
    let dt = time.delta_secs();
    for (mut tr, b) in &mut q {
        if tr.translation.y > b.y_target {
            tr.translation.y = (tr.translation.y - b.speed * dt).max(b.y_target);
        }
    }
}

// ========== 系统：通关倒计时 ==========
pub(super) fn mario_finish_seq(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<&MarioPlayer>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(player) = player_q.single() else {
        return;
    };
    if !player.finished {
        return;
    }
    stage.finish_timer += time.delta_secs();
    // 时间转分数
    if stage.time_left > 0.0 {
        let take = (200.0 * time.delta_secs()).min(stage.time_left);
        stage.time_left -= take;
        session.score = session.score.saturating_add(take.ceil() as u32 * 50);
    }
    if stage.finish_timer > 3.5 {
        session.finished = true;
        session.won = true;
        session.status = "🏰 通关！按 Enter 再来一次，Esc / Backspace 回菜单".to_string();
        let idx = session.kind.index();
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level as u8 + 1).min(4);
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
    }
}

// ========== 系统：玩家死亡 / 重生 ==========
pub(super) fn mario_respawn(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &mut Transform)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, mut tr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= time.delta_secs();
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
            // 复活到出生点 + 重置成 Small
            tr.translation.x = stage.player_spawn.x;
            tr.translation.y = stage.player_spawn.y;
            player.vel = Vec2::ZERO;
            player.on_ground = false;
            player.invincible = 1.6;
            player.dead_timer = 0.0;
            player.state = PowerState::Small;
            player.transform_t = 0.0;
            player.fire_cd = 0.0;
            commands.entity(player_e).despawn_related::<Children>();
            build_player_visual(&mut commands, player_e, PowerState::Small);
            stage.time_left = LEVEL_TIME;
        }
    }
}

// ========== 系统：相机跟随 ==========
pub(super) fn mario_camera_follow(
    player_q: Query<&Transform, (With<MarioPlayer>, Without<Camera>, Without<MarioBackground>)>,
    mut cam_q: Query<&mut Transform, (With<Camera>, Without<MarioPlayer>, Without<MarioBackground>)>,
    mut bg_q: Query<&mut Transform, (With<MarioBackground>, Without<Camera>, Without<MarioPlayer>)>,
) {
    let Ok(ptr) = player_q.single() else { return };
    let Ok(mut ctr) = cam_q.single_mut() else {
        return;
    };
    // 只跟随 X，Y 固定在 0；相机不能往左退（FC 风格）
    let cam_min = ARENA_W * 0.5;
    let cam_max = level_world_max_x() - ARENA_W * 0.5;
    let target = (ptr.translation.x + CAMERA_FOLLOW_OFFSET).clamp(cam_min, cam_max);
    if target > ctr.translation.x {
        ctr.translation.x = target;
    } else if ctr.translation.x < cam_min {
        ctr.translation.x = cam_min;
    }
    ctr.translation.y = 0.0;
    // 背景跟随相机，避免相机移出固定背景区域后露出黑色
    for mut btr in &mut bg_q {
        btr.translation.x = ctr.translation.x;
        btr.translation.y = ctr.translation.y;
    }
}

// ========== 系统：玩家无敌闪烁 ==========
pub(super) fn mario_player_blink(
    player_q: Query<(&MarioPlayer, &Children)>,
    mut sprite_q: Query<&mut Sprite, (With<MarioVisual>, Without<MarioPlayer>)>,
    time: Res<Time>,
) {
    for (player, children) in &player_q {
        let visible = if player.invincible > 0.0 || player.transform_t > 0.0 {
            (time.elapsed_secs() * 24.0).sin() > 0.0
        } else {
            true
        };
        for c in children {
            if let Ok(mut s) = sprite_q.get_mut(*c) {
                let mut col = s.color.to_srgba();
                col.alpha = if visible { 1.0 } else { 0.25 };
                s.color = Color::Srgba(col);
            }
        }
    }
}

// ========== 系统：HUD 更新 ==========
pub(super) fn mario_hud_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut hud_q: Query<(&mut Text2d, &MarioHud)>,
    player_q: Query<&MarioPlayer>,
) {
    if !session.paused
        && !session.finished
        && player_q.single().map(|p| p.dead_timer <= 0.0 && !p.finished).unwrap_or(false)
    {
        stage.time_left = (stage.time_left - time.delta_secs()).max(0.0);
    }
    for (mut t, hud) in &mut hud_q {
        match hud.kind {
            HudKind::Score => *t = Text2d::new(format!("{:06}", session.score)),
            HudKind::Coins => *t = Text2d::new(format!("x{:02}", stage.coins)),
            HudKind::Time => *t = Text2d::new(format!("{:03}", stage.time_left.ceil() as i32)),
            HudKind::World => *t = Text2d::new(format!("1-{}", stage.level)),
            HudKind::Status => {
                if session.finished {
                    *t = Text2d::new(session.status.clone());
                } else {
                    *t = Text2d::new(String::new());
                }
            }
        }
    }
}

// ========== 系统：时间到 ==========
pub(super) fn mario_time_check(
    mut session: ResMut<GameSession>,
    stage: Res<MarioStage>,
    mut player_q: Query<&mut MarioPlayer>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(mut player) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer <= 0.0 && !player.finished && stage.time_left <= 0.0 {
        player.dead_timer = 2.2;
        player.vel = Vec2::new(0.0, 380.0);
        session.lives -= 1;
    }
}

// ========== 道具生成 ==========
fn spawn_powerup(commands: &mut Commands, block_pos: Vec2, kind: PowerUpKind) {
    let size = POWERUP_SIZE;
    let entity = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(size)),
            Transform::from_translation(Vec3::new(block_pos.x, block_pos.y, Z_POWERUP)),
            PowerUp {
                kind,
                vel: Vec2::ZERO,
                emerge_y_target: block_pos.y + TILE,
                emerged: false,
                on_ground: false,
            },
            GameEntity,
        ))
        .id();

    let parts: Vec<(f32, f32, f32, f32, Color)> = match kind {
        PowerUpKind::Mushroom => mushroom_parts(COLOR_MUSHROOM_RED),
        PowerUpKind::OneUp => mushroom_parts(COLOR_ONEUP_GREEN),
        PowerUpKind::FireFlower => flower_parts(),
        PowerUpKind::Star => star_parts(),
    };
    for (dx, dy, w, h, c) in parts {
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

fn mushroom_parts(cap: Color) -> Vec<(f32, f32, f32, f32, Color)> {
    vec![
        // 头顶（半圆形）
        (0.0, 9.0, 12.0, 4.0, cap),
        (0.0, 5.0, 22.0, 6.0, cap),
        (0.0, 0.0, 24.0, 4.0, cap),
        // 白点
        (-7.0, 5.0, 6.0, 6.0, COLOR_MUSHROOM_SPOT),
        (7.0, 5.0, 6.0, 6.0, COLOR_MUSHROOM_SPOT),
        // 头底分隔
        (0.0, -3.0, 22.0, 2.0, COLOR_M_BLACK),
        // 脚（米色）
        (0.0, -8.0, 20.0, 8.0, COLOR_MUSHROOM_STEM),
        // 眼
        (-4.0, -7.0, 3.0, 5.0, COLOR_M_BLACK),
        (4.0, -7.0, 3.0, 5.0, COLOR_M_BLACK),
    ]
}

fn flower_parts() -> Vec<(f32, f32, f32, f32, Color)> {
    vec![
        // 茎
        (0.0, -8.0, 4.0, 12.0, COLOR_FLOWER_GREEN),
        // 叶
        (-7.0, -6.0, 8.0, 4.0, COLOR_FLOWER_GREEN),
        (7.0, -6.0, 8.0, 4.0, COLOR_FLOWER_GREEN),
        // 花瓣（红十字）
        (0.0, 6.0, 18.0, 8.0, COLOR_FLOWER_RED),
        (0.0, 6.0, 8.0, 18.0, COLOR_FLOWER_RED),
        // 花瓣内圈（黄）
        (0.0, 6.0, 12.0, 6.0, COLOR_FLOWER_YELLOW),
        (0.0, 6.0, 6.0, 12.0, COLOR_FLOWER_YELLOW),
        // 花蕊
        (0.0, 6.0, 4.0, 4.0, COLOR_M_BLACK),
    ]
}

fn star_parts() -> Vec<(f32, f32, f32, f32, Color)> {
    vec![
        // 中心
        (0.0, 0.0, 14.0, 14.0, COLOR_FLOWER_YELLOW),
        // 上下尖
        (0.0, 9.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (0.0, -9.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        // 左右尖
        (-9.0, 0.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (9.0, 0.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        // 黑边眼
        (-3.0, 1.0, 2.0, 3.0, COLOR_M_BLACK),
        (3.0, 1.0, 2.0, 3.0, COLOR_M_BLACK),
    ]
}

// ========== 系统：道具升起 + 移动 + 重力 + 碰撞 ==========
pub(super) fn mario_powerup_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut PowerUp, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<PowerUp>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let size = Vec2::splat(POWERUP_SIZE);
    for (e, mut pu, mut tr) in &mut q {
        let mut pos = tr.translation.truncate();

        // 升起阶段
        if !pu.emerged {
            pos.y += POWERUP_EMERGE_SPEED * dt;
            if pos.y >= pu.emerge_y_target {
                pos.y = pu.emerge_y_target;
                pu.emerged = true;
                // 蘑菇 / 1-up 出场就开始走，火花 / 星星不动（火花静止，星星跳）
                pu.vel = match pu.kind {
                    PowerUpKind::Mushroom | PowerUpKind::OneUp => Vec2::new(MUSHROOM_SPEED, 0.0),
                    PowerUpKind::Star => Vec2::new(MUSHROOM_SPEED * 0.9, 320.0),
                    PowerUpKind::FireFlower => Vec2::ZERO,
                };
            }
            tr.translation.x = pos.x;
            tr.translation.y = pos.y;
            continue;
        }

        // 火花原地不动；其他做物理
        if matches!(pu.kind, PowerUpKind::FireFlower) {
            // 仅做位置同步，等玩家拾取
            tr.translation.x = pos.x;
            tr.translation.y = pos.y;
            // 掉出关卡的安全删除
            if pos.y < FALL_DEATH_Y {
                commands.entity(e).despawn();
            }
            continue;
        }

        // 重力
        pu.vel.y -= GRAVITY * dt;
        pu.vel.y = pu.vel.y.max(-FALL_MAX);

        // X 轴
        pos.x += pu.vel.x * dt;
        let mut bumped_x = false;
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
                    bumped_x = true;
                }
            }
        }
        if bumped_x {
            pu.vel.x = -pu.vel.x;
        }

        // Y 轴
        pos.y += pu.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        // 星星持续弹跳；其他静稳
                        if matches!(pu.kind, PowerUpKind::Star) {
                            pu.vel.y = 320.0;
                        } else if pu.vel.y < 0.0 {
                            pu.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if pu.vel.y > 0.0 {
                            pu.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        pu.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

// ========== 系统：玩家拾取道具 ==========
pub(super) fn mario_player_vs_powerup(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<PowerUp>>,
    powerup_q: Query<(Entity, &PowerUp, &Transform), Without<MarioPlayer>>,
) {
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let pu_size = Vec2::splat(POWERUP_SIZE);
    for (e, pu, ptr2) in &powerup_q {
        if !pu.emerged && !matches!(pu.kind, PowerUpKind::FireFlower) {
            // 升起阶段也允许拾取火花（玩家还没看见就吃到不合理，但保持简洁）
        }
        let pp = ptr2.translation.truncate();
        if !aabb_overlap(p_pos, p_size, pp, pu_size) {
            continue;
        }
        match pu.kind {
            PowerUpKind::Mushroom => {
                if matches!(player.state, PowerState::Small) {
                    upgrade_player(&mut commands, player_e, &mut player, PowerState::Big);
                }
                session.score = session.score.saturating_add(1000);
            }
            PowerUpKind::FireFlower => {
                let next = match player.state {
                    PowerState::Small => PowerState::Big,
                    PowerState::Big | PowerState::Fire => PowerState::Fire,
                };
                if next != player.state {
                    upgrade_player(&mut commands, player_e, &mut player, next);
                }
                session.score = session.score.saturating_add(1000);
            }
            PowerUpKind::OneUp => {
                session.lives += 1;
            }
            PowerUpKind::Star => {
                player.invincible = 9.0;
                session.score = session.score.saturating_add(1000);
            }
        }
        commands.entity(e).despawn();
    }
}

fn upgrade_player(
    commands: &mut Commands,
    player_e: Entity,
    player: &mut MarioPlayer,
    target: PowerState,
) {
    player.state = target;
    player.transform_t = 0.6;
    // 删旧子节点 → 重生新视觉
    commands
        .entity(player_e)
        .despawn_related::<Children>();
    build_player_visual(commands, player_e, target);
}

// ========== 火球 ==========
fn spawn_fireball(commands: &mut Commands, pos: Vec2, dir: f32) {
    let size = FIREBALL_SIZE;
    let entity = commands
        .spawn((
            Sprite::from_color(COLOR_FIREBALL_EDGE, Vec2::splat(size)),
            Transform::from_translation(Vec3::new(pos.x, pos.y, Z_FIREBALL)),
            Fireball {
                vel: Vec2::new(FIREBALL_SPEED * dir, -120.0),
                life: FIREBALL_LIFE,
            },
            GameEntity,
        ))
        .id();
    // 火心（白热）
    commands
        .spawn((
            Sprite::from_color(COLOR_FIREBALL_CORE, Vec2::splat(size * 0.5)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
}

pub(super) fn mario_fireball_update(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut q: Query<(Entity, &mut Fireball, &mut Transform), Without<Goomba>>,
    solid_q: Query<(&Transform, &Solid), (Without<Fireball>, Without<Goomba>)>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<Fireball>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let size = Vec2::splat(FIREBALL_SIZE);
    for (e, mut fb, mut tr) in &mut q {
        fb.life -= dt;
        if fb.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        // 重力
        fb.vel.y -= GRAVITY * 0.6 * dt;
        fb.vel.y = fb.vel.y.max(-FALL_MAX);

        let mut pos = tr.translation.truncate();
        // X 轴
        pos.x += fb.vel.x * dt;
        let mut hit_wall = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                hit_wall = true;
                break;
            }
        }
        if hit_wall {
            commands.entity(e).despawn();
            continue;
        }
        // Y 轴
        pos.y += fb.vel.y * dt;
        let mut bounced = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        fb.vel.y = FIREBALL_BOUNCE;
                        bounced = true;
                    } else {
                        // 顶到下方块 → 灭
                        commands.entity(e).despawn();
                        bounced = true;
                        break;
                    }
                }
            }
        }
        let _ = bounced;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
            continue;
        }

        // 击中 Goomba
        for (mut g, gtr) in &mut goomba_q {
            if g.dead || g.squashed > 0.0 {
                continue;
            }
            if aabb_overlap(pos, size, gtr.translation.truncate(), Vec2::new(GOOMBA_W, GOOMBA_H)) {
                g.squashed = GOOMBA_SQUASH_TIME;
                g.vel.x = 0.0;
                session.score = session.score.saturating_add(200);
                commands.entity(e).despawn();
                break;
            }
        }
    }
}

// ========== 砖块敲碎（大/火力马里奥撞砖）==========
pub(super) fn mario_brick_break(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform), Without<BrickTile>>,
    brick_q: Query<(Entity, &Transform, &Solid), (With<BrickTile>, Without<MarioPlayer>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    if matches!(player.state, PowerState::Small) {
        return;
    }
    if player.vel.y <= 0.0 {
        return; // 仅向上撞才砸碎
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let p_top = p_pos.y + p_size.y * 0.5;
    for (e, btr, s) in &brick_q {
        let bp = btr.translation.truncate();
        if !aabb_overlap(p_pos, p_size, bp, s.size) {
            continue;
        }
        let b_bottom = bp.y - s.size.y * 0.5;
        if p_top > b_bottom - 4.0 && p_top < bp.y {
            // 玩家头顶贴近砖块底部 → 碎裂
            commands.entity(e).despawn();
            spawn_brick_shards(&mut commands, bp);
            session.score = session.score.saturating_add(50);
            player.vel.y = -120.0; // 反弹一点
        }
    }
}

fn spawn_brick_shards(commands: &mut Commands, center: Vec2) {
    // 4 块斜射飞出的小砖
    let dirs: [(f32, f32); 4] = [
        (-140.0, 360.0),
        (-90.0, 240.0),
        (90.0, 240.0),
        (140.0, 360.0),
    ];
    for (vx, vy) in dirs {
        commands.spawn((
            Sprite::from_color(COLOR_BRICK, Vec2::splat(SHARD_SIZE)),
            Transform::from_translation(Vec3::new(center.x, center.y, Z_SHARD)),
            BrickShard {
                vel: Vec2::new(vx, vy),
                life: SHARD_LIFE,
                spin: vx.signum() * 14.0,
            },
            GameEntity,
        ));
    }
}

pub(super) fn mario_shard_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut BrickShard)>,
) {
    let dt = time.delta_secs().min(0.033);
    for (e, mut tr, mut s) in &mut q {
        s.life -= dt;
        if s.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        s.vel.y -= GRAVITY * dt;
        s.vel.y = s.vel.y.max(-FALL_MAX);
        tr.translation.x += s.vel.x * dt;
        tr.translation.y += s.vel.y * dt;
        tr.rotation *= Quat::from_rotation_z(s.spin * dt);
    }
}

// ========== 第三阶段：关卡数据 ==========
fn build_level_1_2() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    // 底层地砖
    for c in 0..cols {
        g[13][c] = 'G';
    }
    // 顶部砖天花板（前 145 列），最后留出口
    for c in 0..145 {
        g[0][c] = 'B';
        g[1][c] = 'B';
    }

    // 玩家
    g[12][1] = 'M';

    // 地下散布问号砖（贴近天花板下方 row 3）
    g[3][20] = 'Q';
    g[3][24] = 'U';
    g[3][50] = 'B';
    g[3][52] = 'Q';
    g[3][54] = 'B';
    g[3][86] = 'B';
    g[3][88] = '*'; // 星星
    g[3][90] = 'B';

    // Koopa 巡逻
    for &c in [16usize, 36, 60, 90, 120].iter() {
        g[12][c] = 'k';
    }

    // 中段管道（视觉）
    for &top in [70usize, 100].iter() {
        g[12][top] = 'p';
        g[12][top + 1] = 'p';
        g[11][top] = 'p';
        g[11][top + 1] = 'p';
    }

    // 末段：旗杆 + 城堡
    g[7][148] = 'F';
    g[12][156] = 'C';

    g
}

fn build_level_1_3() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    // 起始平台
    for c in 0..12 {
        g[13][c] = 'G';
    }
    // 末段平台（含旗杆区）
    for c in 138..cols {
        g[13][c] = 'G';
    }

    // 玩家
    g[12][1] = 'M';

    // 浮空跳跃岛
    let small_islands: [(usize, usize, usize); 12] = [
        (15, 18, 6),
        (22, 24, 4),
        (28, 30, 6),
        (38, 42, 7),
        (50, 52, 5),
        (60, 64, 7),
        (74, 78, 6),
        (86, 88, 4),
        (94, 98, 6),
        (108, 112, 5),
        (118, 122, 4),
        (128, 134, 6),
    ];
    for (start, end, h) in small_islands {
        for c in start..=end {
            g[h][c] = 'S';
        }
    }

    // 移动平台（上下）
    g[7][33] = 'L';
    g[7][68] = 'L';
    g[7][102] = 'L';

    // 红龟（在小岛上不会掉下）
    g[5][39] = 'r';
    g[6][75] = 'r';
    g[5][95] = 'r';
    g[12][140] = 'r';
    g[12][148] = 'r';

    // 隐藏问号砖
    g[6][26] = 'U'; // 火花砖
    g[5][125] = '*'; // 星星

    // 末段旗杆 + 城堡
    g[7][156] = 'F';

    g
}

fn build_level_1_4() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    // 暗砖底层
    for c in 0..cols {
        g[13][c] = 'D';
    }

    // 岩浆坑（替换底层 + 上一行）
    for &(start, end) in [(35usize, 39usize), (70, 75), (110, 115)].iter() {
        for c in start..=end {
            g[13][c] = 'l';
            g[12][c] = 'l';
        }
    }

    // 玩家
    g[12][1] = 'M';

    // 跳跃平台
    for c in 28..32 {
        g[10][c] = 'D';
    }
    for c in 44..48 {
        g[8][c] = 'D';
    }
    for c in 60..64 {
        g[10][c] = 'D';
    }
    for c in 80..84 {
        g[10][c] = 'D';
    }
    for c in 92..98 {
        g[8][c] = 'D';
    }

    // 上方天花板暗砖（前后段）
    for c in 0..30 {
        g[2][c] = 'D';
    }
    for c in 100..145 {
        g[2][c] = 'D';
    }

    // 敌人
    g[12][20] = 'g';
    g[12][55] = 'k';
    g[12][88] = 'g';
    g[12][125] = 'k';

    // Bowser
    g[12][140] = 'b';

    // 斧头（通关道具）
    g[12][156] = 'A';

    g
}

// ========== 第三阶段：spawn 函数 ==========
fn spawn_dark_block(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_DARK_BRICK, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        Solid {
            size: Vec2::splat(TILE),
        },
        DarkBrick,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(
            COLOR_DARK_BRICK_LIGHT,
            Vec2::new(TILE - 4.0, 3.0),
        ),
        Transform::from_translation(
            (center + Vec2::new(0.0, TILE * 0.5 - 3.0)).extend(Z_TILE + 0.05),
        ),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(
            COLOR_DARK_BRICK_DARK,
            Vec2::new(TILE - 4.0, 3.0),
        ),
        Transform::from_translation(
            (center + Vec2::new(0.0, -TILE * 0.5 + 3.0)).extend(Z_TILE + 0.05),
        ),
        GameEntity,
    ));
}

fn spawn_lava(commands: &mut Commands, center: Vec2) {
    commands.spawn((
        Sprite::from_color(COLOR_LAVA, Vec2::splat(TILE)),
        Transform::from_translation(center.extend(Z_TILE)),
        LavaTile,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(COLOR_LAVA_BRIGHT, Vec2::new(TILE, 4.0)),
        Transform::from_translation(
            (center + Vec2::new(0.0, TILE * 0.4)).extend(Z_TILE + 0.05),
        ),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(
            COLOR_LAVA_BRIGHT,
            Vec2::new(TILE * 0.4, 3.0),
        ),
        Transform::from_translation(
            (center + Vec2::new(-6.0, TILE * 0.2)).extend(Z_TILE + 0.05),
        ),
        GameEntity,
    ));
    let _ = LAVA_DAMAGE;
}

fn spawn_moving_platform(commands: &mut Commands, center: Vec2, vertical: bool) {
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
            Sprite::from_color(
                COLOR_PLATFORM_DARK,
                Vec2::new(PLATFORM_W, 2.0),
            ),
            Transform::from_translation(Vec3::new(0.0, -PLATFORM_H * 0.5 + 1.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    // 链条像素
    for i in 0..3 {
        commands
            .spawn((
                Sprite::from_color(
                    COLOR_PLATFORM_DARK,
                    Vec2::new(2.0, 4.0),
                ),
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

fn spawn_axe(commands: &mut Commands, center: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::splat(AXE_SIZE),
            ),
            Transform::from_translation(center.extend(Z_DECOR)),
            Axe,
            GameEntity,
        ))
        .id();
    // 斧柄
    commands
        .spawn((
            Sprite::from_color(COLOR_AXE_HANDLE, Vec2::new(3.0, 18.0)),
            Transform::from_translation(Vec3::new(0.0, -3.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    // 斧刃
    commands
        .spawn((
            Sprite::from_color(COLOR_AXE_HEAD, Vec2::new(14.0, 8.0)),
            Transform::from_translation(Vec3::new(2.0, 8.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
    commands
        .spawn((
            Sprite::from_color(COLOR_AXE_HEAD, Vec2::new(8.0, 4.0)),
            Transform::from_translation(Vec3::new(5.0, 12.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
}

fn spawn_high_sky_decor(commands: &mut Commands) {
    // 月亮
    commands.spawn((
        Sprite::from_color(COLOR_MOON, Vec2::splat(40.0)),
        Transform::from_translation(Vec3::new(160.0, 200.0, Z_BG_MID)),
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.4),
            Vec2::splat(8.0),
        ),
        Transform::from_translation(Vec3::new(166.0, 206.0, Z_BG_MID + 0.05)),
        GameEntity,
    ));
    let cloud_positions = [(420.0, 180.0), (1000.0, 220.0), (1700.0, 170.0)];
    for (x, y) in cloud_positions {
        spawn_cloud(commands, Vec2::new(x, y));
    }
}

fn spawn_koopa(commands: &mut Commands, pos: Vec2, kind: KoopaKind) {
    let body = if matches!(kind, KoopaKind::Green) {
        COLOR_KOOPA_GREEN
    } else {
        COLOR_KOOPA_RED
    };
    let dark = if matches!(kind, KoopaKind::Green) {
        COLOR_KOOPA_GREEN_DARK
    } else {
        COLOR_KOOPA_RED_DARK
    };
    let entity = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::new(KOOPA_W, KOOPA_H),
            ),
            Transform::from_translation(pos.extend(Z_ENEMY)),
            Koopa {
                kind,
                vel: Vec2::new(-KOOPA_SPEED, 0.0),
                on_ground: false,
                state: KoopaState::Walking,
                state_t: 0.0,
                dead: false,
            },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 13] = [
        // 龟壳主体
        (0.0, 2.0, 22.0, 18.0, body),
        (0.0, 10.0, 18.0, 4.0, dark),
        (-8.0, 4.0, 4.0, 10.0, COLOR_SHELL_RIM),
        (8.0, 4.0, 4.0, 10.0, COLOR_SHELL_RIM),
        (0.0, -6.0, 22.0, 4.0, COLOR_SHELL_RIM),
        // 头
        (0.0, 14.0, 12.0, 8.0, COLOR_KOOPA_BELLY),
        (-3.0, 16.0, 2.0, 2.0, COLOR_M_BLACK),
        (3.0, 16.0, 2.0, 2.0, COLOR_M_BLACK),
        // 嘴
        (3.0, 12.0, 4.0, 2.0, COLOR_KOOPA_RED_DARK),
        // 脚
        (-6.0, -16.0, 6.0, 4.0, COLOR_KOOPA_BELLY),
        (6.0, -16.0, 6.0, 4.0, COLOR_KOOPA_BELLY),
        // 肚子
        (0.0, -10.0, 14.0, 6.0, COLOR_KOOPA_BELLY),
        // 尾巴
        (-10.0, -2.0, 4.0, 4.0, body),
    ];
    for (dx, dy, w, h, c) in parts {
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                KoopaVisual,
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

fn spawn_bowser(commands: &mut Commands, pos: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Vec2::new(BOWSER_W, BOWSER_H),
            ),
            Transform::from_translation(pos.extend(Z_ENEMY)),
            Bowser {
                vel: Vec2::new(-BOWSER_SPEED, 0.0),
                on_ground: false,
                hp: BOWSER_HP,
                fire_cd: BOWSER_FIRE_CD,
                dir_t: 1.5,
                dead: false,
            },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 18] = [
        // 龟壳主体
        (0.0, -4.0, 50.0, 32.0, COLOR_BOWSER_BODY),
        (0.0, 4.0, 50.0, 4.0, COLOR_BOWSER_DARK),
        (0.0, -12.0, 50.0, 4.0, COLOR_BOWSER_DARK),
        // 龟壳尖刺
        (-18.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (-6.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (6.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (18.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        // 肚子
        (0.0, -8.0, 28.0, 20.0, COLOR_BOWSER_BELLY),
        (0.0, -8.0, 24.0, 2.0, COLOR_BOWSER_DARK),
        // 头
        (-2.0, 18.0, 24.0, 14.0, COLOR_BOWSER_BODY),
        // 角
        (-9.0, 26.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (9.0, 26.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        // 头发
        (0.0, 24.0, 14.0, 4.0, COLOR_BOWSER_HAIR),
        // 眼
        (-5.0, 19.0, 3.0, 4.0, COLOR_M_WHITE),
        (5.0, 19.0, 3.0, 4.0, COLOR_M_WHITE),
        (-5.0, 19.0, 2.0, 3.0, COLOR_M_BLACK),
        (5.0, 19.0, 2.0, 3.0, COLOR_M_BLACK),
        // 嘴
        (0.0, 12.0, 12.0, 3.0, COLOR_M_BLACK),
    ];
    for (dx, dy, w, h, c) in parts {
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, 0.05)),
                GameEntity,
            ))
            .insert(ChildOf(entity));
    }
}

// ========== 第三阶段：系统 ==========
pub(super) fn mario_koopa_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_koopa: Query<(Entity, &mut Koopa, &mut Transform), Without<MarioPlayer>>,
    solid_q: Query<(&Transform, &Solid), (Without<MarioPlayer>, Without<Koopa>)>,
) {
    if session.paused {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (e, mut k, mut tr) in &mut q_koopa {
        if k.dead {
            commands.entity(e).despawn();
            continue;
        }
        let size = match k.state {
            KoopaState::Walking => Vec2::new(KOOPA_W, KOOPA_H),
            _ => Vec2::new(SHELL_W, SHELL_H),
        };
        // Shell 静止超时变回 Walking
        if matches!(k.state, KoopaState::Shell) {
            k.state_t -= dt;
            if k.state_t <= 0.0 {
                k.state = KoopaState::Walking;
                k.vel.x = -KOOPA_SPEED;
            }
        }

        // 重力
        k.vel.y -= GRAVITY * dt;
        k.vel.y = k.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();

        // X 轴
        pos.x += k.vel.x * dt;
        let mut bumped_x = false;
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
                    bumped_x = true;
                }
            }
        }
        if bumped_x {
            k.vel.x = -k.vel.x;
        }

        // 红龟 / SlidingShell 边缘检测：在地面时检查前方下方是否有平台
        let edge_check = matches!(k.state, KoopaState::Walking)
            && matches!(k.kind, KoopaKind::Red)
            && k.on_ground;
        if edge_check {
            let probe = Vec2::new(
                pos.x + k.vel.x.signum() * (size.x * 0.5 + 4.0),
                pos.y - size.y * 0.5 - 4.0,
            );
            let mut on_floor = false;
            for (st, s) in &solid_q {
                let sp = st.translation.truncate();
                if (probe.x - sp.x).abs() < s.size.x * 0.5
                    && (probe.y - sp.y).abs() < s.size.y * 0.5 + 6.0
                {
                    on_floor = true;
                    break;
                }
            }
            if !on_floor {
                k.vel.x = -k.vel.x;
            }
        }

        // Y 轴
        pos.y += k.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if k.vel.y < 0.0 {
                            k.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if k.vel.y > 0.0 {
                            k.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        k.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
        }
    }
}

fn apply_player_damage(
    commands: &mut Commands,
    player_e: Entity,
    player: &mut MarioPlayer,
    session: &mut GameSession,
) {
    match player.state {
        PowerState::Fire => {
            player.state = PowerState::Big;
            player.transform_t = 0.6;
            player.invincible = 1.6;
            commands
                .entity(player_e)
                .despawn_related::<Children>();
            build_player_visual(commands, player_e, PowerState::Big);
        }
        PowerState::Big => {
            player.state = PowerState::Small;
            player.transform_t = 0.6;
            player.invincible = 1.6;
            commands
                .entity(player_e)
                .despawn_related::<Children>();
            build_player_visual(commands, player_e, PowerState::Small);
        }
        PowerState::Small => {
            player.dead_timer = 2.2;
            player.vel = Vec2::new(0.0, 420.0);
            session.lives -= 1;
        }
    }
}

pub(super) fn mario_player_vs_koopa(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<Koopa>>,
    mut koopa_q: Query<(&mut Koopa, &Transform), Without<MarioPlayer>>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    let star = player.invincible > 5.0;
    for (mut k, ktr) in &mut koopa_q {
        if k.dead {
            continue;
        }
        let k_pos = ktr.translation.truncate();
        let k_size = match k.state {
            KoopaState::Walking => Vec2::new(KOOPA_W, KOOPA_H),
            _ => Vec2::new(SHELL_W, SHELL_H),
        };
        if !aabb_overlap(p_pos, p_size, k_pos, k_size) {
            continue;
        }
        let p_foot = p_pos.y - p_size.y * 0.5;
        let stomping = p_foot > k_pos.y - 4.0 && player.vel.y <= 30.0;
        if star {
            k.dead = true;
            session.score = session.score.saturating_add(200);
            continue;
        }
        match k.state {
            KoopaState::Walking => {
                if stomping {
                    k.state = KoopaState::Shell;
                    k.state_t = 5.0;
                    k.vel.x = 0.0;
                    player.vel.y = STOMP_BOUNCE;
                    session.score = session.score.saturating_add(100);
                } else if player.invincible <= 0.0 && player.transform_t <= 0.0 {
                    apply_player_damage(&mut commands, player_e, &mut player, &mut session);
                }
            }
            KoopaState::Shell => {
                let dir = if p_pos.x < k_pos.x { 1.0 } else { -1.0 };
                k.state = KoopaState::SlidingShell;
                k.vel.x = SHELL_SPEED * dir;
                player.vel.y = STOMP_BOUNCE * 0.5;
                session.score = session.score.saturating_add(400);
            }
            KoopaState::SlidingShell => {
                if stomping {
                    k.state = KoopaState::Shell;
                    k.state_t = 5.0;
                    k.vel.x = 0.0;
                    player.vel.y = STOMP_BOUNCE;
                } else if player.invincible <= 0.0 && player.transform_t <= 0.0 {
                    apply_player_damage(&mut commands, player_e, &mut player, &mut session);
                }
            }
        }
    }
}

pub(super) fn mario_shell_kills(
    mut session: ResMut<GameSession>,
    koopa_q: Query<(&Koopa, &Transform)>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<Koopa>>,
) {
    if session.paused {
        return;
    }
    let mut shells: Vec<Vec2> = Vec::new();
    for (k, tr) in &koopa_q {
        if matches!(k.state, KoopaState::SlidingShell) && !k.dead {
            shells.push(tr.translation.truncate());
        }
    }
    if shells.is_empty() {
        return;
    }
    let shell_size = Vec2::new(SHELL_W, SHELL_H);
    for (mut g, gtr) in &mut goomba_q {
        if g.dead || g.squashed > 0.0 {
            continue;
        }
        let gp = gtr.translation.truncate();
        for sp in &shells {
            if aabb_overlap(*sp, shell_size, gp, Vec2::new(GOOMBA_W, GOOMBA_H)) {
                g.squashed = GOOMBA_SQUASH_TIME;
                g.vel.x = 0.0;
                session.score = session.score.saturating_add(200);
                break;
            }
        }
    }
}

pub(super) fn mario_platform_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(&mut MovingPlatform, &mut Transform)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (mut plat, mut tr) in &mut q {
        let mut new_pos = tr.translation.truncate() + plat.vel * dt;
        let off = new_pos - plat.center;
        if plat.vertical {
            if off.y > PLATFORM_RANGE * 0.5 {
                new_pos.y = plat.center.y + PLATFORM_RANGE * 0.5;
                plat.vel.y = -plat.vel.y.abs();
            } else if off.y < -PLATFORM_RANGE * 0.5 {
                new_pos.y = plat.center.y - PLATFORM_RANGE * 0.5;
                plat.vel.y = plat.vel.y.abs();
            }
        } else {
            if off.x > PLATFORM_RANGE * 0.5 {
                new_pos.x = plat.center.x + PLATFORM_RANGE * 0.5;
                plat.vel.x = -plat.vel.x.abs();
            } else if off.x < -PLATFORM_RANGE * 0.5 {
                new_pos.x = plat.center.x - PLATFORM_RANGE * 0.5;
                plat.vel.x = plat.vel.x.abs();
            }
        }
        plat.last_dx = new_pos.x - tr.translation.x;
        plat.last_dy = new_pos.y - tr.translation.y;
        tr.translation.x = new_pos.x;
        tr.translation.y = new_pos.y;
    }
}

pub(super) fn mario_lava_check(
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform), Without<LavaTile>>,
    lava_q: Query<&Transform, (With<LavaTile>, Without<MarioPlayer>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();
    if player.invincible > 5.0 {
        return; // 星星无敌可踏岩浆
    }
    for ltr in &lava_q {
        let lp = ltr.translation.truncate();
        if aabb_overlap(p_pos, p_size, lp, Vec2::splat(TILE)) {
            player.dead_timer = 2.2;
            player.vel = Vec2::new(0.0, 420.0);
            session.lives -= 1;
            return;
        }
    }
}

pub(super) fn mario_bowser_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q_bow: Query<(&mut Bowser, &mut Transform), (Without<MarioPlayer>, Without<Solid>)>,
    solid_q: Query<(&Transform, &Solid), (Without<Bowser>, Without<MarioPlayer>)>,
    player_q: Query<&Transform, (With<MarioPlayer>, Without<Bowser>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok(ptr) = player_q.single() else { return };
    let p_pos = ptr.translation.truncate();

    for (mut bow, mut tr) in &mut q_bow {
        if bow.dead {
            continue;
        }
        bow.dir_t -= dt;
        if bow.dir_t <= 0.0 {
            bow.vel.x = -bow.vel.x;
            bow.dir_t = 2.0 + (tr.translation.x.abs().fract() * 1.3);
        }

        // 重力
        bow.vel.y -= GRAVITY * dt;
        bow.vel.y = bow.vel.y.max(-FALL_MAX);
        let mut pos = tr.translation.truncate();
        let size = Vec2::new(BOWSER_W, BOWSER_H);

        // X 轴
        pos.x += bow.vel.x * dt;
        let mut bumped_x = false;
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
                    bumped_x = true;
                }
            }
        }
        if bumped_x {
            bow.vel.x = -bow.vel.x;
        }

        // Y 轴
        pos.y += bow.vel.y * dt;
        let mut on_ground = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if bow.vel.y < 0.0 {
                            bow.vel.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        pos.y -= push;
                        if bow.vel.y > 0.0 {
                            bow.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        bow.on_ground = on_ground;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        // 偶尔跳跃
        if on_ground && (bow.dir_t * 1.7).fract() > 0.85 {
            bow.vel.y = 360.0;
        }

        // 火球
        bow.fire_cd -= dt;
        if bow.fire_cd <= 0.0 {
            bow.fire_cd = BOWSER_FIRE_CD;
            let dir = if p_pos.x < pos.x { -1.0 } else { 1.0 };
            commands.spawn((
                Sprite::from_color(COLOR_FIREBALL_EDGE, Vec2::splat(16.0)),
                Transform::from_translation(Vec3::new(
                    pos.x + dir * 28.0,
                    pos.y + 4.0,
                    Z_FIREBALL,
                )),
                BowserFireball {
                    vel: Vec2::new(BOWSER_FIREBALL_SPEED * dir, 0.0),
                    life: 4.0,
                },
                GameEntity,
            ));
        }
    }
}

pub(super) fn mario_bowser_fireball_update(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut q: Query<(Entity, &mut BowserFireball, &mut Transform), Without<MarioPlayer>>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &Transform), Without<BowserFireball>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((player_e, mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    let p_size = player.state.size();

    for (e, mut fb, mut tr) in &mut q {
        fb.life -= dt;
        if fb.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        tr.translation.x += fb.vel.x * dt;
        let pos = tr.translation.truncate();
        if aabb_overlap(pos, Vec2::splat(16.0), p_pos, p_size) {
            commands.entity(e).despawn();
            if player.invincible <= 0.0
                && player.transform_t <= 0.0
                && !(player.invincible > 5.0)
            {
                apply_player_damage(&mut commands, player_e, &mut player, &mut session);
            }
        }
    }
}

pub(super) fn mario_player_fire_vs_bowser(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    fire_q: Query<(Entity, &Transform), (With<Fireball>, Without<Bowser>)>,
    mut bow_q: Query<(&mut Bowser, &Transform), Without<Fireball>>,
) {
    let bowser_size = Vec2::new(BOWSER_W, BOWSER_H);
    for (mut bow, btr) in &mut bow_q {
        if bow.dead {
            continue;
        }
        let bp = btr.translation.truncate();
        for (fe, ftr) in &fire_q {
            let fp = ftr.translation.truncate();
            if aabb_overlap(fp, Vec2::splat(FIREBALL_SIZE), bp, bowser_size) {
                bow.hp -= 1;
                commands.entity(fe).despawn();
                session.score = session.score.saturating_add(200);
                if bow.hp <= 0 {
                    bow.dead = true;
                    session.score = session.score.saturating_add(5000);
                }
            }
        }
    }
}

pub(super) fn mario_bowser_cleanup(mut commands: Commands, q: Query<(Entity, &Bowser)>) {
    for (e, b) in &q {
        if b.dead {
            commands.entity(e).despawn();
        }
    }
}

pub(super) fn mario_axe_check(
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut player_q: Query<(&mut MarioPlayer, &Transform), Without<Axe>>,
    axe_q: Query<&Transform, (With<Axe>, Without<MarioPlayer>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        return;
    }
    let Ok(atr) = axe_q.single() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    let a_pos = atr.translation.truncate();
    if aabb_overlap(p_pos, player.state.size(), a_pos, Vec2::splat(AXE_SIZE)) {
        player.finished = true;
        stage.finish_timer = 0.0;
        session.score = session.score.saturating_add(2000);
    }
}
