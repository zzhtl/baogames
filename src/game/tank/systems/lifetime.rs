use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession, Lifetime};

pub fn tank_lifetime_tick(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut life: Query<(Entity, &mut Lifetime)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    for (e, mut l) in &mut life {
        l.0.tick(time.delta());
        if l.0.just_finished() {
            commands.entity(e).despawn();
        }
    }
}
