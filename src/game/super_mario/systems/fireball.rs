use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;

pub fn mario_fireball_update(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut q: Query<(Entity, &mut Fireball, &mut Transform), Without<Goomba>>,
    solid_q: Query<(&Transform, &Solid), (Without<Fireball>, Without<Goomba>)>,
    mut goomba_q: Query<(&mut Goomba, &Transform), Without<Fireball>>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let size = Vec2::splat(FIREBALL_SIZE);
    for (e, mut fb, mut tr) in &mut q {
        fb.life -= dt;
        if fb.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        fb.vel.y -= GRAVITY * 0.6 * dt;
        fb.vel.y = fb.vel.y.max(-FALL_MAX);

        let mut pos = tr.translation.truncate();
        pos.x += fb.vel.x * dt;
        let mut hit_wall = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                hit_wall = true;
                break;
            }
        }
        if hit_wall {
            commands.entity(e).despawn();
            continue;
        }
        pos.y += fb.vel.y * dt;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        fb.vel.y = FIREBALL_BOUNCE;
                    } else {
                        commands.entity(e).despawn();
                        break;
                    }
                }
            }
        }
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;

        if pos.y < FALL_DEATH_Y {
            commands.entity(e).despawn();
            continue;
        }

        for (mut g, gtr) in &mut goomba_q {
            if g.dead || g.squashed > 0.0 {
                continue;
            }
            if aabb_overlap(pos, size, gtr.translation.truncate(), Vec2::new(GOOMBA_W, GOOMBA_H))
            {
                g.squashed = GOOMBA_SQUASH_TIME;
                g.vel.x = 0.0;
                session.score = session.score.saturating_add(200);
                commands.entity(e).despawn();
                break;
            }
        }
    }
}
