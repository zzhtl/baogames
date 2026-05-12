use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::{FallingBubble, PoppingBubble};
use super::super::constants::{FALL_GRAVITY, PLAY_BOTTOM, POP_LIFETIME};

pub fn bubble_pop_anim(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(Entity, &mut PoppingBubble, &mut Sprite, &mut Transform)>,
) {
    if session.kind != GameKind::BubbleBobble || session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (e, mut pop, mut sp, mut tr) in &mut q {
        pop.life -= dt;
        if pop.life <= 0.0 {
            commands.entity(e).despawn();
        } else {
            let t = pop.life / POP_LIFETIME;
            let s = (1.0 - t) * 1.4 + t;
            tr.scale = Vec3::splat(s.max(0.1));
            let mut c = sp.color.to_srgba();
            c.alpha = (t * 0.9 + 0.05).clamp(0.0, 1.0);
            sp.color = Color::srgba(c.red, c.green, c.blue, c.alpha);
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
        tr.translation.y += fall.vy * dt;
        if tr.translation.y < PLAY_BOTTOM - 30.0 {
            commands.entity(e).despawn();
        }
    }
}
