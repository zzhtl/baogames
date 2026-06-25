use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::render::{UiFont, rect, text};
use crate::common::sprite_def::SpriteDef;
use crate::game::model::{Collider, GameEntity, Lifetime, Velocity};

use super::components::*;
use super::constants::*;
use super::geometry::{subtile_center, tile_center};
use super::resources::TankStage;
use super::sprites::{
    BASE_EAGLE, BRICK_SUBTILE, BUSH_TILE, ICE_TILE, PU_CLOCK, PU_GRENADE, PU_HELMET, PU_SHOVEL,
    PU_STAR, PU_TANK, STEEL_TILE, TANK_ENEMY_ARMOR, TANK_ENEMY_BASIC, TANK_ENEMY_FAST,
    TANK_ENEMY_POWER, TANK_P1, TANK_P2, WATER_TILE,
};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    spawn_play_field(commands);
    spawn_map(commands, level);
    spawn_hud(commands, font, level);
    spawn_mode_select_ui(commands, font);

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
        two_player: false,
        mode_selected: false,
        freeze_timer: 0.0,
    });
}

fn spawn_mode_select_ui(commands: &mut Commands, font: &UiFont) {
    let cx = PLAY_OFFSET_X;
    let cy = PLAY_OFFSET_Y;
    // 半透明黑底覆盖游戏区，避免地形分散注意力
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.72), Vec2::splat(PLAY_SIZE)),
        Transform::from_translation(Vec3::new(cx, cy, 50.0)),
        ModeSelectUi,
        GameEntity,
    ));
    text(
        commands,
        font,
        "选择模式",
        Vec2::new(cx, cy + 80.0),
        34.0,
        Color::srgb(1.0, 0.92, 0.5),
        ModeSelectUi,
    )
    .insert(GameEntity);
    text(
        commands,
        font,
        "1  单人模式",
        Vec2::new(cx, cy + 20.0),
        24.0,
        Color::srgb(0.85, 0.78, 0.36),
        ModeSelectUi,
    )
    .insert(GameEntity);
    text(
        commands,
        font,
        "2  双人模式",
        Vec2::new(cx, cy - 20.0),
        24.0,
        Color::srgb(0.46, 0.7, 0.95),
        ModeSelectUi,
    )
    .insert(GameEntity);
    text(
        commands,
        font,
        "按 1 或 2 确认",
        Vec2::new(cx, cy - 80.0),
        16.0,
        Color::srgb(0.75, 0.82, 0.92),
        ModeSelectUi,
    )
    .insert(GameEntity);
}

pub fn spawn_initial_players_for_mode(commands: &mut Commands, two_player: bool) {
    let p1_pos = tile_center(PLAYER1_SPAWN.0, PLAYER1_SPAWN.1);
    spawn_player_tank(commands, 0, p1_pos);
    if two_player {
        let p2_pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
        spawn_player_tank(commands, 1, p2_pos);
    }
}

fn spawn_play_field(commands: &mut Commands) {
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
    let frame_color = Color::srgb(0.45, 0.45, 0.5);
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_OFFSET_Y + PLAY_SIZE * 0.5 + frame_thickness * 0.5),
        Vec2::new(outer, frame_thickness),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X, PLAY_OFFSET_Y - PLAY_SIZE * 0.5 - frame_thickness * 0.5),
        Vec2::new(outer, frame_thickness),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X - PLAY_SIZE * 0.5 - frame_thickness * 0.5, PLAY_OFFSET_Y),
        Vec2::new(frame_thickness, PLAY_SIZE),
        frame_color,
        GameEntity,
    );
    rect(
        commands,
        Vec2::new(PLAY_OFFSET_X + PLAY_SIZE * 0.5 + frame_thickness * 0.5, PLAY_OFFSET_Y),
        Vec2::new(frame_thickness, PLAY_SIZE),
        frame_color,
        GameEntity,
    );
}

fn spawn_map(commands: &mut Commands, level: u8) {
    let map = &STAGE_MAPS[(level.max(1) as usize - 1) % STAGE_MAPS.len()];
    for (row, line) in map.iter().enumerate() {
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
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
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
    )
    .insert(P2Hud);
}


// ========== 地形 ==========
fn spawn_brick_tile(commands: &mut Commands, col: i32, row: i32) {
    // 一个全块拆成 2x2 个子格
    let sx0 = col * 2;
    let sy0 = row * 2;
    for dy in 0..2 {
        for dx in 0..2 {
            let pos = subtile_center(sx0 + dx, sy0 + dy);
            let parent = commands
                .spawn((
                    Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(SUBTILE)),
                    Transform::from_translation(pos.extend(Z_TILE)),
                    BrickFC,
                    Collider {
                        size: Vec2::splat(SUBTILE),
                    },
                    GameEntity,
                ))
                .id();
            spawn_def_children(commands, parent, &BRICK_SUBTILE);
        }
    }
}

fn spawn_steel(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(32.0)),
            Transform::from_translation(pos.extend(Z_TILE)),
            SteelFC,
            Collider {
                size: Vec2::splat(32.0),
            },
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &STEEL_TILE);
}

fn spawn_water(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(32.0)),
            Transform::from_translation(pos.extend(Z_TILE)),
            WaterFC,
            Collider {
                size: Vec2::splat(32.0),
            },
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &WATER_TILE);
}

fn spawn_bush(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(32.0)),
            Transform::from_translation(pos.extend(Z_BUSH)),
            BushFC,
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &BUSH_TILE);
}

fn spawn_ice(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(32.0)),
            Transform::from_translation(pos.extend(Z_TILE - 0.1)),
            IceFC,
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &ICE_TILE);
}

fn spawn_base(commands: &mut Commands, col: i32, row: i32) {
    let pos = tile_center(col, row);
    // 底座 + 鹰徽（子块随 base despawn 一起清理）
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(28.0)),
            Transform::from_translation(pos.extend(Z_BASE)),
            BaseFC,
            Collider {
                size: Vec2::splat(28.0),
            },
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &BASE_EAGLE);
}

// ========== 坦克与特效 ==========
/// 按 SpriteDef 给坦克父实体挂子块（与离线预览同源）。子块 z 用 part 的 dz（相对父）。
fn spawn_def_children(commands: &mut Commands, parent: Entity, def: &SpriteDef) {
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

pub fn spawn_player_tank(commands: &mut Commands, id: usize, pos: Vec2) {
    let def = if id == 0 { &TANK_P1 } else { &TANK_P2 };
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(TANK_SIZE)),
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
        ))
        .id();
    spawn_def_children(commands, parent, def);
}

pub fn spawn_enemy_tank(commands: &mut Commands, pos: Vec2, kind: EnemyTankKind) {
    // 按兵种配精灵 + (速度, 子弹速度, 血量)
    let (def, speed, bullet_speed, hp) = match kind {
        EnemyTankKind::Basic => (&TANK_ENEMY_BASIC, ENEMY_SPEED_BASE, ENEMY_BULLET_SPEED, 1),
        EnemyTankKind::Fast => (&TANK_ENEMY_FAST, ENEMY_SPEED_BASE * 1.8, ENEMY_BULLET_SPEED, 1),
        EnemyTankKind::Power => (
            &TANK_ENEMY_POWER,
            ENEMY_SPEED_BASE,
            ENEMY_BULLET_SPEED * 1.6,
            1,
        ),
        EnemyTankKind::Armor => (
            &TANK_ENEMY_ARMOR,
            ENEMY_SPEED_BASE * 0.85,
            ENEMY_BULLET_SPEED,
            3,
        ),
    };
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(TANK_SIZE)),
            Transform {
                translation: pos.extend(Z_TANK),
                rotation: Quat::from_rotation_z(TankDir::Down.rotation()),
                ..default()
            },
            GameEntity,
            TankFC {
                side: TankSide::Enemy,
                speed,
                fire_cd: ENEMY_FIRE_CD,
                fire_cd_left: rand::thread_rng().gen_range(0.4..1.2),
                bullet_speed,
                max_bullets: 1,
                bullets_alive: 0,
                hp,
                shield_left: 0.0,
            },
            TankDir::Down,
            EnemyTankFC {
                turn_timer: 0.0,
                kind,
            },
            Collider {
                size: Vec2::splat(TANK_SIZE - 2.0),
            },
        ))
        .id();
    spawn_def_children(commands, parent, def);
}

pub fn spawn_spawn_effect(
    commands: &mut Commands,
    pos: Vec2,
    side: TankSide,
    player_id: Option<usize>,
    enemy_kind: Option<EnemyTankKind>,
) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.95, 1.0), Vec2::splat(TANK_SIZE)),
        Transform::from_translation(pos.extend(Z_TANK + 0.5)),
        SpawnEffect {
            timer: Timer::new(Duration::from_millis(900), TimerMode::Once),
            spawn_pos: pos,
            side,
            player_id,
            enemy_kind,
        },
        GameEntity,
    ));
}

pub fn spawn_explosion(commands: &mut Commands, pos: Vec2, big: bool) {
    let size = if big { 36.0 } else { 18.0 };
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.78, 0.22), Vec2::splat(size)),
        Transform::from_translation(pos.extend(Z_BULLET + 0.1)),
        Lifetime(Timer::new(Duration::from_millis(220), TimerMode::Once)),
        GameEntity,
    ));
}

pub fn spawn_bullet(
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

pub fn spawn_muzzle_flash(commands: &mut Commands, pos: Vec2, dir: TankDir) {
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

pub fn spawn_powerup(commands: &mut Commands, pos: Vec2, kind: PowerUpKind) {
    let def = match kind {
        PowerUpKind::Star => &PU_STAR,
        PowerUpKind::Grenade => &PU_GRENADE,
        PowerUpKind::Helmet => &PU_HELMET,
        PowerUpKind::Tank => &PU_TANK,
        PowerUpKind::Clock => &PU_CLOCK,
        PowerUpKind::Shovel => &PU_SHOVEL,
    };
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(POWERUP_SIZE)),
            Transform::from_translation(pos.extend(Z_BUSH + 0.5)),
            PowerUp { kind },
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, def);
}

/// 在世界坐标处生成一块钢墙（铲子道具给基地筑墙用）。
pub fn spawn_steel_at(commands: &mut Commands, pos: Vec2) {
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(32.0)),
            Transform::from_translation(pos.extend(Z_TILE)),
            SteelFC,
            Collider {
                size: Vec2::splat(32.0),
            },
            GameEntity,
        ))
        .id();
    spawn_def_children(commands, parent, &STEEL_TILE);
}
