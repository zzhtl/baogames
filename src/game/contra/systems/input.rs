use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{muzzle_offset, player_size, resolve_player_aim};
use super::super::resources::ContraControls;
use super::super::setup_actors::{spawn_muzzle_flash, spawn_player_bullet};

pub fn contra_sample_input(
    actions: Res<ActionState>,
    save: Res<SaveData>,
    session: Res<GameSession>,
    mut controls: ResMut<ContraControls>,
) {
    if session.paused || session.finished {
        controls.clear();
        return;
    }
    controls.movement = actions.movement(PlayerSlot::One);
    controls.fire_held = actions.pressed(PlayerSlot::One, InputAction::Primary);
    if actions.just_pressed(PlayerSlot::One, InputAction::Secondary) {
        controls.latch_jump(save.settings.gameplay_profile);
    }
}

pub fn contra_player_input(
    mut controls: ResMut<ContraControls>,
    save: Res<SaveData>,
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
        controls.jump_buffer_ticks = 0;
        return;
    }

    let movement = controls.movement;
    let left = movement.x < 0.0;
    let right = movement.x > 0.0;
    let up = movement.y > 0.0;
    let down = movement.y < 0.0;

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

    player.aim = resolve_player_aim(movement, player.on_ground, player.facing);

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

    if player.on_ground {
        player.coyote_ticks = save.settings.gameplay_profile.coyote_ticks();
    } else {
        player.coyote_ticks = player.coyote_ticks.saturating_sub(1);
    }
    let jump_ready = (player.on_ground || player.coyote_ticks > 0) && !player.prone;
    if controls.consume_jump(jump_ready) {
        player.vel.y = JUMP_VEL;
        player.on_ground = false;
        player.coyote_ticks = 0;
        sfx.write(PlaySfx(SfxKind::Jump));
    }

    if player.fire_cd > 0.0 {
        player.fire_cd = (player.fire_cd - dt).max(0.0);
    }
    if controls.fire_held && player.fire_cd <= 0.0 {
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
        spawn_muzzle_flash(&mut commands, origin, dir);
        sfx.write(PlaySfx(SfxKind::Shoot));
        player.fire_cd = weapon.fire_cd();
    }
}
