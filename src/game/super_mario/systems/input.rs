use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::MarioControls;
use super::super::setup_actors::spawn_fireball;

pub fn mario_sample_input(
    actions: Res<ActionState>,
    save: Res<SaveData>,
    session: Res<GameSession>,
    mut controls: ResMut<MarioControls>,
) {
    if session.paused || session.finished {
        controls.clear_for_inactive_session();
        return;
    }

    let player = PlayerSlot::One;
    controls.horizontal = actions.movement(player).x;
    controls.run_held = actions.pressed(player, InputAction::Primary);
    controls.jump_held = actions.pressed(player, InputAction::Secondary);
    if actions.just_pressed(player, InputAction::Secondary) {
        controls.latch_jump(save.settings.gameplay_profile);
    }
    if actions.just_pressed(player, InputAction::Primary) {
        controls.fire_buffer_ticks = controls.fire_buffer_ticks.max(2);
    }
}

pub fn mario_player_input(
    mut controls: ResMut<MarioControls>,
    save: Res<SaveData>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(&mut MarioPlayer, &mut Transform)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut tr)) = q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.finished {
        controls.jump_buffer_ticks = 0;
        controls.fire_buffer_ticks = 0;
        return;
    }

    let horizontal = controls.horizontal;
    let left = horizontal < 0.0;
    let right = horizontal > 0.0;
    let run = controls.run_held;
    let jump_held = controls.jump_held;

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

    if !run && player.vel.x.abs() > WALK_SPEED && player.on_ground {
        let s = player.vel.x.signum();
        player.vel.x = (player.vel.x.abs().min(WALK_SPEED + 20.0)) * s;
    }

    if right {
        player.facing = Facing::Right;
    } else if left {
        player.facing = Facing::Left;
    }

    if player.on_ground {
        player.coyote_ticks = save.settings.gameplay_profile.coyote_ticks();
    } else {
        player.coyote_ticks = player.coyote_ticks.saturating_sub(1);
    }
    let jump_ready = player.on_ground || player.coyote_ticks > 0;
    if controls.jump_buffer_ticks > 0 && jump_ready {
        let extra = (player.vel.x.abs() / RUN_SPEED) * JUMP_VEL_BONUS;
        player.vel.y = JUMP_VEL_BASE + extra;
        player.on_ground = false;
        player.coyote_ticks = 0;
        player.jumping = true;
        controls.jump_buffer_ticks = 0;
        sfx.write(PlaySfx(SfxKind::Jump));
    } else {
        controls.jump_buffer_ticks = controls.jump_buffer_ticks.saturating_sub(1);
    }

    if !jump_held && player.vel.y > 0.0 {
        player.jumping = false;
    }

    let scale_x = if player.facing == Facing::Right {
        ACTOR_SCALE
    } else {
        -ACTOR_SCALE
    };
    tr.scale.x = scale_x;

    if player.fire_cd > 0.0 {
        player.fire_cd = (player.fire_cd - dt).max(0.0);
    }
    if matches!(player.state, PowerState::Fire)
        && controls.fire_buffer_ticks > 0
        && player.fire_cd <= 0.0
    {
        let dir = if player.facing == Facing::Right { 1.0 } else { -1.0 };
        let muzzle = Vec2::new(tr.translation.x + dir * 14.0, tr.translation.y + 4.0);
        spawn_fireball(&mut commands, muzzle, dir);
        player.fire_cd = FIRE_CD;
        controls.fire_buffer_ticks = 0;
    } else {
        controls.fire_buffer_ticks = controls.fire_buffer_ticks.saturating_sub(1);
    }

    if player.transform_t > 0.0 {
        player.transform_t = (player.transform_t - dt).max(0.0);
    }
}
