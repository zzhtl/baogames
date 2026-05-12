use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::{ENEMY_H, ENEMY_W};
use super::super::geometry::{aabb_overlap, player_size};
use super::super::setup_actors::spawn_explosion;
use super::bullets::kill_player;

pub fn contra_player_vs_enemy(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut ContraPlayer, &Transform)>,
    enemy_q: Query<(Entity, &Transform), With<ContraEnemy>>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 || player.invincible > 0.0 {
        return;
    }
    let pp = ptr.translation.truncate();
    let ps = player_size(player.prone);
    for (ee, etr) in &enemy_q {
        let ep = etr.translation.truncate();
        if aabb_overlap(pp, ps, ep, Vec2::new(ENEMY_W, ENEMY_H)) {
            kill_player(&mut player, pp, &mut commands);
            spawn_explosion(&mut commands, ep, 24.0, 0.3);
            commands.entity(ee).despawn();
            if session.lives > 0 {
                session.lives -= 1;
            }
            return;
        }
    }
}
