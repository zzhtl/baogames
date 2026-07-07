use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::{SpacePowerUp, SpaceShipPlayer};
use super::super::geometry::aabb;
use super::super::resources::SpaceState;

pub fn space_powerup_drop_and_pickup(
    mut commands: Commands,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut state: ResMut<SpaceState>,
    mut powerups: Query<
        (Entity, &mut Transform, &Collider),
        (With<SpacePowerUp>, Without<SpaceShipPlayer>),
    >,
    players: Query<(&Transform, &Collider), With<SpaceShipPlayer>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::SpaceShooter || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    for (e, mut pt, pc) in &mut powerups {
        pt.translation.y -= 110.0 * delta;
        for (player_t, player_c) in &players {
            if aabb(
                pt.translation.truncate(),
                pc.size,
                player_t.translation.truncate(),
                player_c.size,
            ) {
                commands.entity(e).despawn();
                sfx.write(PlaySfx(SfxKind::Powerup));
                state.power = (state.power + 1).min(3);
                session.score += 200;
                state.message = match state.power {
                    2 => "火力 LV.2".to_string(),
                    3 => "火力 LV.3 — MAX".to_string(),
                    _ => "补给".to_string(),
                };
                state.message_clock = 1.4;
                break;
            }
        }
    }
}
