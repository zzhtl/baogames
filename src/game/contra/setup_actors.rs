use bevy::prelude::*;

use crate::common::render::UiFont;
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;
use super::resources::*;

pub fn spawn_player(commands: &mut Commands, pos: Vec2) {
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
    let parts: [(f32, f32, f32, f32, Color); 14] = [
        (0.0, 13.0, 12.0, 3.0, COLOR_PLAYER_HELMET),
        (0.0, 11.0, 13.0, 2.0, COLOR_PLAYER_HELMET_DK),
        (0.0, 8.0, 10.0, 4.0, COLOR_PLAYER_SKIN),
        (3.0, 8.0, 2.0, 2.0, COLOR_PLAYER_HAIR),
        (0.0, 2.0, 12.0, 7.0, COLOR_PLAYER_BODY),
        (0.0, 5.0, 12.0, 2.0, COLOR_PLAYER_BODY_DK),
        (-4.0, 0.0, 4.0, 4.0, COLOR_PLAYER_SKIN),
        (4.0, 0.0, 4.0, 4.0, COLOR_PLAYER_SKIN),
        (0.0, -3.0, 12.0, 2.0, COLOR_PLAYER_BOOT),
        (-3.0, -8.0, 5.0, 7.0, COLOR_PLAYER_PANTS),
        (3.0, -8.0, 5.0, 7.0, COLOR_PLAYER_PANTS),
        (-3.0, -14.0, 5.0, 3.0, COLOR_PLAYER_BOOT),
        (3.0, -14.0, 5.0, 3.0, COLOR_PLAYER_BOOT),
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

pub fn spawn_enemy(commands: &mut Commands, mark: &EnemySpawnMark) {
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
        (0.0, 11.0, 12.0, 3.0, COLOR_ENEMY_HAT),
        (0.0, 9.0, 13.0, 2.0, COLOR_ROCK_OUT),
        (0.0, 6.0, 10.0, 3.0, COLOR_ENEMY_SKIN),
        (0.0, 1.0, 13.0, 7.0, body_color),
        (0.0, 4.0, 13.0, 2.0, COLOR_ENEMY_RED),
        (-3.0, -6.0, 5.0, 6.0, COLOR_ENEMY_PANTS),
        (3.0, -6.0, 5.0, 6.0, COLOR_ENEMY_PANTS),
        (-3.0, -12.0, 5.0, 4.0, COLOR_PLAYER_BOOT),
        (3.0, -12.0, 5.0, 4.0, COLOR_PLAYER_BOOT),
        (9.0, 2.0, 12.0, 2.0, COLOR_ENEMY_GUN),
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

pub fn spawn_falcon(commands: &mut Commands, mark: &FalconMark) {
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
        (0.0, 0.0, 16.0, 8.0, COLOR_FALCON),
        (-12.0, 1.0, 10.0, 4.0, COLOR_FALCON_DARK),
        (10.0, 1.0, 6.0, 6.0, COLOR_FALCON),
        (14.0, 1.0, 4.0, 2.0, COLOR_FALCON_BEAK),
        (-2.0, 5.0, 14.0, 4.0, COLOR_FALCON_DARK),
        (-2.0, -5.0, 14.0, 4.0, COLOR_FALCON_DARK),
        (4.0, 1.0, 4.0, 2.0, mark.weapon.pickup_color()),
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

pub fn spawn_boss(commands: &mut Commands, x: f32) {
    let center_y = GROUND_TOP + BOSS_H * 0.5;
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
