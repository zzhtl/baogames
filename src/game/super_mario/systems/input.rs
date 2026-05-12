use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::setup_actors::spawn_fireball;

pub fn mario_player_input(
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

    if !run && player.vel.x.abs() > WALK_SPEED && player.on_ground {
        let s = player.vel.x.signum();
        player.vel.x = (player.vel.x.abs().min(WALK_SPEED + 20.0)) * s;
    }

    if right {
        player.facing = Facing::Right;
    } else if left {
        player.facing = Facing::Left;
    }

    if jump_pressed && player.on_ground {
        let extra = (player.vel.x.abs() / RUN_SPEED) * JUMP_VEL_BONUS;
        player.vel.y = JUMP_VEL_BASE + extra;
        player.on_ground = false;
        player.jumping = true;
    }

    if !jump_held && player.vel.y > 0.0 {
        player.jumping = false;
    }

    let scale_x = if player.facing == Facing::Right { 1.0 } else { -1.0 };
    tr.scale.x = scale_x;

    if player.fire_cd > 0.0 {
        player.fire_cd = (player.fire_cd - dt).max(0.0);
    }
    if matches!(player.state, PowerState::Fire)
        && keys.just_pressed(KeyCode::KeyJ)
        && player.fire_cd <= 0.0
    {
        let dir = if player.facing == Facing::Right { 1.0 } else { -1.0 };
        let muzzle = Vec2::new(tr.translation.x + dir * 14.0, tr.translation.y + 4.0);
        spawn_fireball(&mut commands, muzzle, dir);
        player.fire_cd = FIRE_CD;
    }

    if player.transform_t > 0.0 {
        player.transform_t = (player.transform_t - dt).max(0.0);
    }
}
