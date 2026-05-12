use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession, Lifetime};

use super::super::components::SpaceBullet;
use super::super::constants::{PLAY_BOTTOM, PLAY_LEFT, PLAY_RIGHT, PLAY_TOP};

pub fn space_bullets_update(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut bullets: Query<(Entity, &mut Transform, &SpaceBullet)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (e, mut t, b) in &mut bullets {
        t.translation.x += b.vel.x * delta;
        t.translation.y += b.vel.y * delta;
        if t.translation.y > PLAY_TOP + 40.0
            || t.translation.y < PLAY_BOTTOM - 40.0
            || t.translation.x < PLAY_LEFT - 40.0
            || t.translation.x > PLAY_RIGHT + 40.0
        {
            commands.entity(e).despawn();
        }
    }
}

pub fn space_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut life: Query<(Entity, &mut Lifetime)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    for (e, mut l) in &mut life {
        l.0.tick(time.delta());
        if l.0.just_finished() {
            commands.entity(e).despawn();
        }
    }
}
