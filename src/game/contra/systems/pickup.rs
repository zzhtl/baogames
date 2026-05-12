use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{aabb_overlap, player_size};
use super::super::palette::COLOR_FALCON;

pub fn contra_pickup_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut commands: Commands,
    mut pickup_q: Query<
        (Entity, &mut ContraPickup, &mut Transform, &mut Sprite),
        Without<ContraPlayer>,
    >,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraPickup>>,
    mut player_q: Query<(&mut ContraPlayer, &Transform), Without<ContraPickup>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    for (e, mut p, mut tr, mut sprite) in &mut pickup_q {
        if !p.on_ground {
            p.vel_y = (p.vel_y - GRAVITY * 0.6 * dt).max(-PICKUP_FALL_SPEED * 1.5);
            tr.translation.y += p.vel_y * dt;
            let pos = tr.translation.truncate();
            for (st, s) in &solid_q {
                let sp = st.translation.truncate();
                if aabb_overlap(pos, Vec2::splat(PICKUP_SIZE), sp, s.size) {
                    let dy = pos.y - sp.y;
                    if dy > 0.0 && p.vel_y < 0.0 {
                        let push = (PICKUP_SIZE + s.size.y) * 0.5 - dy.abs();
                        if push > 0.0 {
                            tr.translation.y += push;
                            p.vel_y = 0.0;
                            p.on_ground = true;
                        }
                    }
                }
            }
        }
        p.pulse += dt;
        let blink = (p.pulse * 6.0).sin() * 0.5 + 0.5;
        let mut col = p.weapon.pickup_color().to_srgba();
        col.red = (col.red * (0.6 + 0.4 * blink)).min(1.0);
        col.green = (col.green * (0.6 + 0.4 * blink)).min(1.0);
        col.blue = (col.blue * (0.6 + 0.4 * blink)).min(1.0);
        sprite.color = Color::Srgba(col);

        if let Ok((mut player, ptr)) = player_q.single_mut() {
            if player.dead_timer <= 0.0 {
                let pp = ptr.translation.truncate();
                let ps = player_size(player.prone);
                if aabb_overlap(pp, ps, tr.translation.truncate(), Vec2::splat(PICKUP_SIZE)) {
                    player.weapon = p.weapon;
                    commands.entity(e).despawn();
                    session.score += 300;
                }
            }
        }
    }
}

pub fn contra_falcon_update(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut ContraFalcon, &mut Transform, &mut Sprite)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    for (e, f, mut tr, mut sprite) in &mut q {
        tr.translation.x += f.vel.x * dt;
        tr.translation.y += (f.vel.x.signum() * 6.0 * (time.elapsed_secs() * 4.0).sin()) * dt;
        let mut col = COLOR_FALCON.to_srgba();
        let phase = (time.elapsed_secs() * 8.0).sin() * 0.5 + 0.5;
        col.red = 0.7 + 0.3 * phase;
        col.green = 0.7 + 0.3 * phase;
        col.blue = 0.85;
        sprite.color = Color::Srgba(col);
        if tr.translation.x > WORLD_W + 80.0 || tr.translation.x < -80.0 {
            commands.entity(e).despawn();
        }
    }
}
