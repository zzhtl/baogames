use bevy::prelude::*;

use crate::common::constants::ARENA_W;

use super::super::components::*;
use super::super::constants::ENEMY_DESPAWN_BACK;

pub fn contra_offscreen_cleanup(
    mut commands: Commands,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraEnemy>)>,
    enemy_q: Query<(Entity, &Transform), With<ContraEnemy>>,
    bullet_q: Query<(Entity, &Transform), With<ContraBullet>>,
    pickup_q: Query<(Entity, &Transform), With<ContraPickup>>,
) {
    let Ok(cam) = cam_q.single() else { return };
    let cam_x = cam.translation.x;
    let cull = cam_x - ARENA_W * 0.5 - ENEMY_DESPAWN_BACK;
    for (e, t) in &enemy_q {
        if t.translation.x < cull {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &bullet_q {
        if t.translation.x < cull || t.translation.x > cam_x + ARENA_W * 0.5 + 200.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, t) in &pickup_q {
        if t.translation.x < cull {
            commands.entity(e).despawn();
        }
    }
}
