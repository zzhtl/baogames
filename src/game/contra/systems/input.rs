use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{muzzle_offset, player_size};
use super::super::setup_actors::spawn_player_bullet;

pub fn contra_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(&mut ContraPlayer, &mut Sprite, &mut Transform)>,
    mut sfx: MessageWriter<PlaySfx>,
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

    if right && !left {
        player.facing = 1.0;
    } else if left && !right {
        player.facing = -1.0;
    }

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

    if jump_pressed && player.on_ground && !player.prone {
        player.vel.y = JUMP_VEL;
        player.on_ground = false;
    }

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
                for delta in [-0.35_f32, -0.18, 0.0, 0.18, 0.35] {
                    let (sin, cos) = delta.sin_cos();
                    let v = Vec2::new(dir.x * cos - dir.y * sin, dir.x * sin + dir.y * cos);
                    spawn_player_bullet(&mut commands, origin, v, Weapon::S);
                }
            }
            _ => spawn_player_bullet(&mut commands, origin, dir, weapon),
        }
        sfx.write(PlaySfx(SfxKind::Shoot));
        player.fire_cd = weapon.fire_cd();
    }

    let s = if player.facing >= 0.0 { 1.0 } else { -1.0 };
    tr.scale.x = s;
}
