use bevy::prelude::*;

use super::super::components::CoinPopup;

pub fn mario_coin_popup(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut CoinPopup)>,
) {
    let dt = time.delta_secs();
    for (e, mut tr, mut c) in &mut q {
        c.timer -= dt;
        c.vy -= 600.0 * dt;
        tr.translation.y += c.vy * dt;
        if c.timer <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}
