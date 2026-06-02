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
    // 像素分层（局部坐标，Z 从下到上递增）：
    //   描边层在 +0.04，色块在 +0.05~+0.09，装备细节在 +0.10
    let parts: &[(f32, f32, f32, f32, Color, f32)] = &[
        // ===== 头部 =====
        // 头巾（红黄）— 横向贯穿头部上方
        (0.0, 13.0, 14.0, 3.0, COLOR_PLAYER_HELMET, 0.06),
        (0.0, 11.5, 14.0, 1.0, COLOR_PLAYER_HELMET_DK, 0.07),
        // 头巾飘带（左侧）
        (-7.0, 12.0, 3.0, 1.0, COLOR_PLAYER_HELMET, 0.07),
        (-8.0, 11.0, 3.0, 1.0, COLOR_PLAYER_HELMET_DK, 0.07),
        // 脸
        (0.0, 9.0, 10.0, 4.0, COLOR_PLAYER_SKIN, 0.06),
        // 头发（额前 + 鬓角）
        (-3.0, 11.0, 4.0, 1.0, COLOR_PLAYER_HAIR, 0.07),
        (-5.0, 9.5, 1.0, 2.0, COLOR_PLAYER_HAIR, 0.07),
        // 脸阴影（右侧）
        (3.0, 8.5, 4.0, 1.0, COLOR_PLAYER_SKIN_DK, 0.07),
        // 眼睛（一格黑点）
        (2.0, 10.0, 1.0, 1.0, COLOR_PLAYER_OUTLINE, 0.08),
        // 颈部
        (-1.0, 6.5, 4.0, 2.0, COLOR_PLAYER_SKIN, 0.06),

        // ===== 躯干 =====
        // 主体（绿背心）
        (0.0, 2.5, 12.0, 7.0, COLOR_PLAYER_BODY, 0.06),
        // 胸口阴影（右下）
        (2.0, 1.0, 7.0, 4.0, COLOR_PLAYER_BODY_DK, 0.07),
        // 腰带（横）
        (0.0, -1.0, 12.0, 2.0, COLOR_PLAYER_BOOT, 0.08),
        (0.0, -1.5, 12.0, 1.0, COLOR_PLAYER_OUTLINE, 0.085),
        // 弹链（斜跨胸口的黄带）
        (-4.0, 4.0, 3.0, 2.0, COLOR_PLAYER_BANDOLIER, 0.09),
        (-2.0, 2.5, 3.0, 2.0, COLOR_PLAYER_BANDOLIER, 0.09),
        (0.0, 1.0, 3.0, 2.0, COLOR_PLAYER_BANDOLIER, 0.09),
        (2.0, -0.5, 3.0, 2.0, COLOR_PLAYER_BANDOLIER, 0.09),
        // 弹链上的子弹（小黑点）
        (-4.0, 4.0, 1.0, 1.0, COLOR_PLAYER_OUTLINE, 0.095),
        (-2.0, 2.5, 1.0, 1.0, COLOR_PLAYER_OUTLINE, 0.095),
        (0.0, 1.0, 1.0, 1.0, COLOR_PLAYER_OUTLINE, 0.095),
        (2.0, -0.5, 1.0, 1.0, COLOR_PLAYER_OUTLINE, 0.095),
        // 露出的胳膊：左臂自然垂下、右臂前伸持枪
        (-6.0, 2.0, 3.0, 5.0, COLOR_PLAYER_SKIN, 0.07),
        (-6.0, 0.0, 3.0, 2.0, COLOR_PLAYER_SKIN_DK, 0.075),
        (6.0, 2.0, 3.0, 3.0, COLOR_PLAYER_SKIN, 0.07),

        // ===== 腿 =====
        // 裤子主体
        (-3.0, -7.0, 5.0, 8.0, COLOR_PLAYER_PANTS, 0.06),
        (3.0, -7.0, 5.0, 8.0, COLOR_PLAYER_PANTS, 0.06),
        // 裤腿外侧阴影
        (-4.5, -7.0, 1.5, 8.0, COLOR_PLAYER_PANTS_DK, 0.07),
        (4.5, -7.0, 1.5, 8.0, COLOR_PLAYER_PANTS_DK, 0.07),
        // 膝盖高光
        (-3.0, -6.0, 3.0, 1.0, COLOR_PLAYER_PANTS_DK, 0.075),
        (3.0, -6.0, 3.0, 1.0, COLOR_PLAYER_PANTS_DK, 0.075),
        // 战靴
        (-3.0, -13.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
        (3.0, -13.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
        (-3.0, -14.0, 5.0, 1.0, COLOR_PLAYER_OUTLINE, 0.07),
        (3.0, -14.0, 5.0, 1.0, COLOR_PLAYER_OUTLINE, 0.07),

        // ===== 步枪（朝右）：枪托 + 枪身 + 弹匣 + 枪管 =====
        (8.5, 2.5, 3.0, 4.0, COLOR_PLAYER_BOOT, 0.10),    // 枪托
        (12.0, 3.0, 6.0, 2.0, COLOR_PLAYER_GUN, 0.10),    // 枪身
        (12.0, 3.5, 6.0, 1.0, COLOR_PLAYER_GUN_HI, 0.105),
        (12.0, 1.5, 1.0, 2.0, COLOR_PLAYER_GUN, 0.10),    // 弹匣
        (16.5, 3.0, 6.0, 1.0, COLOR_PLAYER_GUN, 0.10),    // 枪管
        (19.5, 3.0, 2.0, 1.0, COLOR_PLAYER_OUTLINE, 0.105), // 枪口

        // ===== 黑色轮廓描边（贴在最底层 +0.04）=====
        // 头顶
        (0.0, 14.5, 14.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04),
        // 脸两侧
        (-5.5, 9.5, 1.0, 5.0, COLOR_PLAYER_OUTLINE, 0.04),
        (5.5, 9.5, 1.0, 5.0, COLOR_PLAYER_OUTLINE, 0.04),
        // 躯干两侧
        (-6.5, 3.0, 1.0, 7.0, COLOR_PLAYER_OUTLINE, 0.04),
        (6.5, 3.0, 1.0, 7.0, COLOR_PLAYER_OUTLINE, 0.04),
        // 腿外侧
        (-5.5, -7.0, 1.0, 8.0, COLOR_PLAYER_OUTLINE, 0.04),
        (5.5, -7.0, 1.0, 8.0, COLOR_PLAYER_OUTLINE, 0.04),
        // 靴底
        (-3.0, -15.5, 5.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04),
        (3.0, -15.5, 5.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04),
    ];
    for (dx, dy, w, h, color, dz) in parts.iter().copied() {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, Z_PLAYER + dz)),
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
    let body_dark = match mark.kind {
        EnemyKind::Soldier => COLOR_ENEMY_BODY_DK,
        EnemyKind::Sniper => Color::srgb(0.40, 0.06, 0.06),
        EnemyKind::Jumper => Color::srgb(0.10, 0.18, 0.60),
    };
    let parts: &[(f32, f32, f32, f32, Color, f32)] = &[
        // 头盔
        (0.0, 11.5, 12.0, 3.0, COLOR_ENEMY_HAT, 0.06),
        (0.0, 13.0, 12.0, 1.0, COLOR_ENEMY_OUTLINE, 0.065),
        (0.0, 10.0, 13.0, 1.0, COLOR_ENEMY_OUTLINE, 0.07),
        // 脸
        (0.0, 7.0, 10.0, 3.0, COLOR_ENEMY_SKIN, 0.06),
        (3.0, 7.0, 4.0, 1.0, COLOR_PLAYER_SKIN_DK, 0.065),
        (2.0, 8.0, 1.0, 1.0, COLOR_ENEMY_OUTLINE, 0.07),
        // 颈
        (-1.0, 5.0, 4.0, 1.0, COLOR_ENEMY_SKIN, 0.06),
        // 躯干主体（红 / 蓝制服）
        (0.0, 1.5, 13.0, 7.0, body_color, 0.06),
        (3.0, 0.0, 6.0, 4.0, body_dark, 0.07),
        // 肩章
        (-5.0, 4.5, 3.0, 1.0, COLOR_ENEMY_HAT, 0.08),
        (5.0, 4.5, 3.0, 1.0, COLOR_ENEMY_HAT, 0.08),
        // 腰带
        (0.0, -2.0, 13.0, 1.5, COLOR_PLAYER_BOOT, 0.08),
        (0.0, -2.5, 13.0, 1.0, COLOR_ENEMY_OUTLINE, 0.085),
        // 腿
        (-3.0, -6.5, 5.0, 7.0, COLOR_ENEMY_PANTS, 0.06),
        (3.0, -6.5, 5.0, 7.0, COLOR_ENEMY_PANTS, 0.06),
        (-4.5, -6.5, 1.5, 7.0, COLOR_ENEMY_PANTS_DK, 0.07),
        (4.5, -6.5, 1.5, 7.0, COLOR_ENEMY_PANTS_DK, 0.07),
        // 靴
        (-3.0, -12.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
        (3.0, -12.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
        (-3.0, -13.0, 5.0, 1.0, COLOR_ENEMY_OUTLINE, 0.07),
        (3.0, -13.0, 5.0, 1.0, COLOR_ENEMY_OUTLINE, 0.07),
        // 手 + 步枪
        (6.5, 2.0, 3.0, 3.0, COLOR_ENEMY_SKIN, 0.07),
        (10.0, 2.0, 8.0, 2.0, COLOR_ENEMY_GUN, 0.10),
        (10.0, 2.5, 8.0, 1.0, COLOR_PLAYER_GUN_HI, 0.105),
        (15.0, 2.0, 1.0, 1.0, COLOR_ENEMY_OUTLINE, 0.11),
        // 描边
        (-6.5, 2.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
        (6.5, 2.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
        (-5.5, -6.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
        (5.5, -6.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
    ];
    for (dx, dy, w, h, color, dz) in parts.iter().copied() {
        commands
            .spawn((
                Sprite::from_color(color, Vec2::new(w, h)),
                Transform::from_translation(Vec3::new(dx, dy, Z_ENEMY + dz)),
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
