use bevy::prelude::*;

use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette::*;

pub fn spawn_player(commands: &mut Commands, pos: Vec2) {
    let state = PowerState::Small;
    let size = state.size();
    let player = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), size),
            Transform::from_translation(pos.extend(Z_PLAYER)).with_scale(Vec3::splat(ACTOR_SCALE)),
            MarioPlayer {
                vel: Vec2::ZERO,
                on_ground: false,
                coyote_ticks: 0,
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

pub fn build_player_visual(commands: &mut Commands, player: Entity, state: PowerState) {
    let parts: Vec<(f32, f32, f32, f32, Color)> = match state {
        PowerState::Small => small_mario_parts().to_vec(),
        PowerState::Big => big_mario_parts(false),
        PowerState::Fire => big_mario_parts(true),
    };
    for (dx, dy, w, h, c) in parts {
        let limb = mario_limb(state, dx, dy);
        let base = Vec3::new(dx, dy, 0.05);
        commands
            .spawn((
                Sprite::from_color(c, Vec2::new(w, h)),
                Transform::from_translation(base),
                MarioVisual { base, limb },
                GameEntity,
            ))
            .insert(ChildOf(player));
    }
}

fn mario_limb(state: PowerState, x: f32, y: f32) -> MarioLimb {
    let foot_y = if state == PowerState::Small { -7.0 } else { -8.0 };
    if y <= foot_y {
        if x < 0.0 {
            MarioLimb::LeftFoot
        } else {
            MarioLimb::RightFoot
        }
    } else {
        let hand = match state {
            PowerState::Small => x.abs() >= 8.0 && (-2.0..=3.0).contains(&y),
            PowerState::Big | PowerState::Fire => x.abs() >= 9.0 && (-2.0..=7.0).contains(&y),
        };
        if hand && x < 0.0 {
            MarioLimb::LeftHand
        } else if hand {
            MarioLimb::RightHand
        } else {
            MarioLimb::Body
        }
    }
}

pub fn small_mario_parts() -> [(f32, f32, f32, f32, Color); 18] {
    [
        (-6.0, 11.0, 14.0, 4.0, COLOR_M_RED),
        (0.0, 13.0, 12.0, 2.0, COLOR_M_RED),
        (-8.0, 7.0, 4.0, 4.0, COLOR_M_BROWN),
        (8.0, 7.0, 2.0, 4.0, COLOR_M_BROWN),
        (-2.0, 7.0, 8.0, 4.0, COLOR_M_SKIN),
        (-2.0, 4.0, 10.0, 2.0, COLOR_M_SKIN),
        (3.0, 7.0, 2.0, 2.0, COLOR_M_BLACK),
        (-2.0, 5.0, 8.0, 1.0, COLOR_M_BROWN),
        (-4.0, 0.0, 14.0, 4.0, COLOR_M_RED),
        (0.0, 2.0, 6.0, 2.0, COLOR_M_RED),
        (-3.0, 0.0, 2.0, 4.0, COLOR_M_BLUE),
        (3.0, 0.0, 2.0, 4.0, COLOR_M_BLUE),
        (-4.0, -4.0, 14.0, 4.0, COLOR_M_BLUE),
        (-4.0, -8.0, 6.0, 4.0, COLOR_M_BLUE),
        (4.0, -8.0, 6.0, 4.0, COLOR_M_BLUE),
        (-5.0, -12.0, 6.0, 2.0, COLOR_M_BROWN),
        (5.0, -12.0, 6.0, 2.0, COLOR_M_BROWN),
        (-9.0, 1.0, 3.0, 3.0, COLOR_M_SKIN),
    ]
}

pub fn big_mario_parts(fire: bool) -> Vec<(f32, f32, f32, f32, Color)> {
    let cap = if fire { COLOR_M_WHITE } else { COLOR_M_FIRE_RED };
    let shirt = if fire { COLOR_M_FIRE_RED } else { COLOR_M_RED };
    let overall = if fire { COLOR_M_FIRE_RED } else { COLOR_M_BLUE };
    vec![
        (-7.0, 21.0, 16.0, 4.0, cap),
        (1.0, 23.0, 12.0, 2.0, cap),
        (-9.0, 17.0, 6.0, 2.0, cap),
        (-9.0, 13.0, 4.0, 4.0, COLOR_M_BROWN),
        (9.0, 13.0, 2.0, 4.0, COLOR_M_BROWN),
        (-2.0, 13.0, 10.0, 6.0, COLOR_M_SKIN),
        (-1.0, 9.0, 12.0, 2.0, COLOR_M_SKIN),
        (4.0, 13.0, 2.0, 3.0, COLOR_M_BLACK),
        (-1.0, 10.0, 10.0, 1.5, COLOR_M_BROWN),
        (0.0, 7.0, 8.0, 2.0, COLOR_M_SKIN),
        (-4.0, 3.0, 16.0, 6.0, shirt),
        (0.0, 5.0, 6.0, 2.0, shirt),
        (-3.0, 1.0, 2.0, 6.0, overall),
        (3.0, 1.0, 2.0, 6.0, overall),
        (0.0, 4.0, 2.0, 2.0, COLOR_QUESTION),
        (-10.0, 4.0, 4.0, 5.0, shirt),
        (10.0, 4.0, 4.0, 5.0, shirt),
        (-11.0, 1.0, 4.0, 4.0, COLOR_M_SKIN),
        (11.0, 1.0, 4.0, 4.0, COLOR_M_SKIN),
        (-4.0, -4.0, 16.0, 6.0, overall),
        (-5.0, -10.0, 6.0, 6.0, overall),
        (5.0, -10.0, 6.0, 6.0, overall),
        (0.0, -6.0, 1.5, 8.0, COLOR_M_BLACK),
        (-6.0, -16.0, 8.0, 4.0, COLOR_M_BROWN),
        (6.0, -16.0, 8.0, 4.0, COLOR_M_BROWN),
        (-6.0, -19.0, 9.0, 2.0, COLOR_M_BLACK),
        (6.0, -19.0, 9.0, 2.0, COLOR_M_BLACK),
    ]
}

pub fn spawn_goomba(commands: &mut Commands, pos: Vec2) {
    let goomba = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(GOOMBA_W, GOOMBA_H)),
            Transform::from_translation(pos.extend(Z_ENEMY)).with_scale(Vec3::splat(ACTOR_SCALE)),
            Goomba {
                vel: Vec2::new(-GOOMBA_SPEED, 0.0),
                on_ground: false,
                squashed: 0.0,
                dead: false,
            },
            GameEntity,
        ))
        .id();
    let parts: [(f32, f32, f32, f32, Color); 9] = [
        (0.0, 4.0, 22.0, 14.0, COLOR_G_BROWN),
        (0.0, 12.0, 16.0, 4.0, COLOR_G_DARK),
        (-5.0, 6.0, 4.0, 5.0, COLOR_G_LIGHT),
        (5.0, 6.0, 4.0, 5.0, COLOR_G_LIGHT),
        (-5.0, 6.0, 2.0, 3.0, COLOR_M_BLACK),
        (5.0, 6.0, 2.0, 3.0, COLOR_M_BLACK),
        (-2.0, 0.0, 2.0, 2.0, COLOR_CLOUD),
        (2.0, 0.0, 2.0, 2.0, COLOR_CLOUD),
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

pub fn spawn_koopa(commands: &mut Commands, pos: Vec2, kind: KoopaKind) {
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
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(KOOPA_W, KOOPA_H)),
            Transform::from_translation(pos.extend(Z_ENEMY)).with_scale(Vec3::splat(ACTOR_SCALE)),
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
        (0.0, 2.0, 22.0, 18.0, body),
        (0.0, 10.0, 18.0, 4.0, dark),
        (-8.0, 4.0, 4.0, 10.0, COLOR_SHELL_RIM),
        (8.0, 4.0, 4.0, 10.0, COLOR_SHELL_RIM),
        (0.0, -6.0, 22.0, 4.0, COLOR_SHELL_RIM),
        (0.0, 14.0, 12.0, 8.0, COLOR_KOOPA_BELLY),
        (-3.0, 16.0, 2.0, 2.0, COLOR_M_BLACK),
        (3.0, 16.0, 2.0, 2.0, COLOR_M_BLACK),
        (3.0, 12.0, 4.0, 2.0, COLOR_KOOPA_RED_DARK),
        (-6.0, -16.0, 6.0, 4.0, COLOR_KOOPA_BELLY),
        (6.0, -16.0, 6.0, 4.0, COLOR_KOOPA_BELLY),
        (0.0, -10.0, 14.0, 6.0, COLOR_KOOPA_BELLY),
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

pub fn spawn_bowser(commands: &mut Commands, pos: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::new(BOWSER_W, BOWSER_H)),
            Transform::from_translation(pos.extend(Z_ENEMY)).with_scale(Vec3::splat(ACTOR_SCALE)),
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
        (0.0, -4.0, 50.0, 32.0, COLOR_BOWSER_BODY),
        (0.0, 4.0, 50.0, 4.0, COLOR_BOWSER_DARK),
        (0.0, -12.0, 50.0, 4.0, COLOR_BOWSER_DARK),
        (-18.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (-6.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (6.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (18.0, 14.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (0.0, -8.0, 28.0, 20.0, COLOR_BOWSER_BELLY),
        (0.0, -8.0, 24.0, 2.0, COLOR_BOWSER_DARK),
        (-2.0, 18.0, 24.0, 14.0, COLOR_BOWSER_BODY),
        (-9.0, 26.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (9.0, 26.0, 4.0, 6.0, COLOR_BOWSER_HORN),
        (0.0, 24.0, 14.0, 4.0, COLOR_BOWSER_HAIR),
        (-5.0, 19.0, 3.0, 4.0, COLOR_M_WHITE),
        (5.0, 19.0, 3.0, 4.0, COLOR_M_WHITE),
        (-5.0, 19.0, 2.0, 3.0, COLOR_M_BLACK),
        (5.0, 19.0, 2.0, 3.0, COLOR_M_BLACK),
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

pub fn spawn_axe(commands: &mut Commands, center: Vec2) {
    let entity = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(AXE_SIZE)),
            Transform::from_translation(center.extend(Z_DECOR)).with_scale(Vec3::splat(ACTOR_SCALE)),
            Axe,
            GameEntity,
        ))
        .id();
    commands
        .spawn((
            Sprite::from_color(COLOR_AXE_HANDLE, Vec2::new(3.0, 18.0)),
            Transform::from_translation(Vec3::new(0.0, -3.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
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

pub fn spawn_powerup(commands: &mut Commands, block_pos: Vec2, kind: PowerUpKind) {
    let size = POWERUP_SIZE;
    let entity = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(size)),
            Transform::from_translation(Vec3::new(block_pos.x, block_pos.y, Z_POWERUP))
                .with_scale(Vec3::splat(ACTOR_SCALE)),
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
        (0.0, 9.0, 12.0, 4.0, cap),
        (0.0, 5.0, 22.0, 6.0, cap),
        (0.0, 0.0, 24.0, 4.0, cap),
        (-7.0, 5.0, 6.0, 6.0, COLOR_MUSHROOM_SPOT),
        (7.0, 5.0, 6.0, 6.0, COLOR_MUSHROOM_SPOT),
        (0.0, -3.0, 22.0, 2.0, COLOR_M_BLACK),
        (0.0, -8.0, 20.0, 8.0, COLOR_MUSHROOM_STEM),
        (-4.0, -7.0, 3.0, 5.0, COLOR_M_BLACK),
        (4.0, -7.0, 3.0, 5.0, COLOR_M_BLACK),
    ]
}

fn flower_parts() -> Vec<(f32, f32, f32, f32, Color)> {
    vec![
        (0.0, -8.0, 4.0, 12.0, COLOR_FLOWER_GREEN),
        (-7.0, -6.0, 8.0, 4.0, COLOR_FLOWER_GREEN),
        (7.0, -6.0, 8.0, 4.0, COLOR_FLOWER_GREEN),
        (0.0, 6.0, 18.0, 8.0, COLOR_FLOWER_RED),
        (0.0, 6.0, 8.0, 18.0, COLOR_FLOWER_RED),
        (0.0, 6.0, 12.0, 6.0, COLOR_FLOWER_YELLOW),
        (0.0, 6.0, 6.0, 12.0, COLOR_FLOWER_YELLOW),
        (0.0, 6.0, 4.0, 4.0, COLOR_M_BLACK),
    ]
}

fn star_parts() -> Vec<(f32, f32, f32, f32, Color)> {
    vec![
        (0.0, 0.0, 14.0, 14.0, COLOR_FLOWER_YELLOW),
        (0.0, 9.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (0.0, -9.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (-9.0, 0.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (9.0, 0.0, 6.0, 6.0, COLOR_FLOWER_YELLOW),
        (-3.0, 1.0, 2.0, 3.0, COLOR_M_BLACK),
        (3.0, 1.0, 2.0, 3.0, COLOR_M_BLACK),
    ]
}

pub fn spawn_fireball(commands: &mut Commands, pos: Vec2, dir: f32) {
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
    commands
        .spawn((
            Sprite::from_color(COLOR_FIREBALL_CORE, Vec2::splat(size * 0.5)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)),
            GameEntity,
        ))
        .insert(ChildOf(entity));
}

pub fn spawn_brick_shards(commands: &mut Commands, center: Vec2) {
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
