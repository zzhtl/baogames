use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::common::render::{UiFont, rect, text};
use crate::game::model::{Collider, GameEntity, Lifetime, Velocity};

use super::components::*;
use super::constants::*;
use super::geometry::{subtile_center, tile_center};
use super::resources::TankStage;

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
    spawn_play_field(commands);
    spawn_map(commands);
    spawn_hud(commands, font, level);
    spawn_initial_players(commands);

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

fn spawn_map(commands: &mut Commands) {
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
    );
}

fn spawn_initial_players(commands: &mut Commands) {
    let p1_pos = tile_center(PLAYER1_SPAWN.0, PLAYER1_SPAWN.1);
    let p2_pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
    spawn_player_tank(commands, 0, p1_pos);
    spawn_player_tank(commands, 1, p2_pos);
}

// ========== 地形 ==========
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

// ========== 坦克与特效 ==========
pub fn spawn_player_tank(commands: &mut Commands, id: usize, pos: Vec2) {
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

pub fn spawn_enemy_tank(commands: &mut Commands, pos: Vec2) {
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

pub fn spawn_spawn_effect(
    commands: &mut Commands,
    pos: Vec2,
    side: TankSide,
    player_id: Option<usize>,
) {
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
