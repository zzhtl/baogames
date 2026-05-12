use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::SpaceStar;
use super::super::constants::{PLAY_BOTTOM, PLAY_TOP};

pub fn space_stars_scroll(
    time: Res<Time>,
    session: Res<GameSession>,
    mut stars: Query<(&mut Transform, &SpaceStar)>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (mut t, s) in &mut stars {
        t.translation.y -= s.speed * delta;
        if t.translation.y < PLAY_BOTTOM - 4.0 {
            t.translation.y = PLAY_TOP + 4.0;
        }
    }
}
