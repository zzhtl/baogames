use bevy::prelude::*;

use crate::common::render::{UiFont, spawn_sprite_def};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;
use super::resources::*;
use super::sprites::{
    BOSS_BODY, ENEMY_GUNNER, ENEMY_HEAVY, ENEMY_JUMPER, ENEMY_SNIPER, ENEMY_SOLDIER, FALCON,
    PLAYER_BILL,
};

pub fn spawn_player(commands: &mut Commands, pos: Vec2) {
    // 精灵几何见 sprites::PLAYER_BILL（游戏与离线预览同源）。
    spawn_sprite_def(
        commands,
        &PLAYER_BILL,
        pos,
        Z_PLAYER,
        GameEntity,
        (
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
        ),
    );
}

pub fn spawn_enemy(commands: &mut Commands, mark: &EnemySpawnMark) {
    let vy = if matches!(mark.kind, EnemyKind::Jumper) {
        -120.0
    } else {
        0.0
    };
    // 精灵几何见 sprites::ENEMY_*（兵种仅躯干配色不同）。
    let def = match mark.kind {
        EnemyKind::Soldier => &ENEMY_SOLDIER,
        EnemyKind::Sniper => &ENEMY_SNIPER,
        EnemyKind::Jumper => &ENEMY_JUMPER,
        EnemyKind::Heavy => &ENEMY_HEAVY,
        EnemyKind::Gunner => &ENEMY_GUNNER,
    };
    let hp = match mark.kind {
        EnemyKind::Heavy => 3,
        EnemyKind::Gunner => 2,
        _ => 1,
    };
    spawn_sprite_def(
        commands,
        def,
        mark.pos,
        Z_ENEMY,
        GameEntity,
        (
            ContraEnemy {
                kind: mark.kind,
                vel: Vec2::new(0.0, vy),
                on_ground: false,
                facing: mark.facing,
                fire_cd: match mark.kind {
                    EnemyKind::Sniper => 0.6,
                    EnemyKind::Gunner => 0.5,
                    _ => 1.2,
                },
                ai_t: 0.0,
                hp,
            },
            GameEntity,
        ),
    );
}

pub fn spawn_falcon(commands: &mut Commands, mark: &FalconMark) {
    // 精灵几何见 sprites::FALCON。
    let parent = spawn_sprite_def(
        commands,
        &FALCON,
        mark.start,
        Z_PICKUP + 0.1,
        GameEntity,
        (
            ContraFalcon {
                vel: Vec2::new(mark.vx, 0.0),
                weapon: mark.weapon,
            },
            GameEntity,
        ),
    );
    // 爪间携带的武器色圆点（动态，照所投武器染色）
    commands
        .spawn((
            Sprite::from_color(mark.weapon.pickup_color(), Vec2::new(5.0, 3.0)),
            Transform::from_translation(Vec3::new(4.0, -1.0, 0.08)),
            GameEntity,
        ))
        .insert(ChildOf(parent));
}

pub fn spawn_pickup(commands: &mut Commands, font: &UiFont, pos: Vec2, weapon: Weapon) {
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

pub fn spawn_explosion(commands: &mut Commands, pos: Vec2, size: f32, life: f32) {
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

pub fn spawn_player_bullet(commands: &mut Commands, pos: Vec2, dir: Vec2, weapon: Weapon) {
    let (size, color, speed, life) = match weapon {
        Weapon::F => (FLAME_SIZE, COLOR_FLAME_CORE, FLAME_SPEED, BULLET_LIFE * 0.8),
        Weapon::L => (LASER_SIZE, COLOR_LASER, LASER_SPEED, BULLET_LIFE),
        Weapon::S => (
            BULLET_BIG_SIZE,
            COLOR_BULLET_P,
            PLAYER_BULLET_SPEED,
            BULLET_LIFE,
        ),
        _ => (BULLET_SIZE, COLOR_BULLET_P, PLAYER_BULLET_SPEED, BULLET_LIFE),
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

pub fn spawn_enemy_bullet(commands: &mut Commands, pos: Vec2, dir: Vec2) {
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

pub fn spawn_boss(commands: &mut Commands, x: f32, hp: i32) {
    let center_y = GROUND_TOP + BOSS_H * 0.5;
    // 要塞主体见 sprites::BOSS_BODY（机械装甲墙 + 中央能量核弱点）。
    spawn_sprite_def(
        commands,
        &BOSS_BODY,
        Vec2::new(x, center_y),
        Z_BOSS,
        GameEntity,
        (
            ContraBoss {
                hp,
                die_t: 0.0,
                flash_t: 0.0,
                spawn_t: 0.0,
            },
            GameEntity,
        ),
    );
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
