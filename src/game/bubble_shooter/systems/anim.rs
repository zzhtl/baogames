use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::{FallingBubble, PoppingBubble, SettlingBubble};
use super::super::constants::{
    FALL_GRAVITY, PLAY_BOTTOM, POP_LIFETIME, SETTLE_LIFETIME,
};

pub fn bubble_pop_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(Entity, &mut PoppingBubble, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (e, mut pop, mut tr) in &mut q {
        pop.life -= dt;
        if pop.life <= 0.0 {
            commands.entity(e).despawn();
        } else {
            let t = pop.life / POP_LIFETIME;
            let pulse = (1.0 - t) * std::f32::consts::PI;
            let s = 1.0 + pulse.sin() * 0.48;
            tr.scale = Vec3::new(s, (2.0 - s * 0.55).max(0.2), 1.0);
            tr.rotation = Quat::from_rotation_z((1.0 - t) * 0.24);
        }
    }
}

pub fn bubble_fall_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(Entity, &mut FallingBubble, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (e, mut fall, mut tr) in &mut q {
        fall.vy += FALL_GRAVITY * dt;
        tr.translation.x += fall.vx * dt;
        tr.translation.y += fall.vy * dt;
        tr.rotate_z(fall.angular_speed * dt);
        if tr.translation.y < PLAY_BOTTOM - 30.0 {
            commands.entity(e).despawn();
        }
    }
}

pub fn bubble_settle_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bubbles: Query<(Entity, &mut SettlingBubble, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (entity, mut settling, mut transform) in &mut bubbles {
        settling.life -= dt;
        if settling.life <= 0.0 {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<SettlingBubble>();
            continue;
        }
        let progress = 1.0 - settling.life / SETTLE_LIFETIME;
        let scale = 0.74 + progress * 0.34 + (progress * std::f32::consts::PI).sin() * 0.08;
        transform.scale = Vec3::new(scale, 2.0 - scale, 1.0);
    }
}
