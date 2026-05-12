use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::{aabb_overlap, level_world_max_x};

pub fn mario_physics(
    time: Res<Time>,
    session: Res<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &mut Transform), Without<Solid>>,
    solid_q: Query<(Entity, &Transform, &Solid), Without<MarioPlayer>>,
    brick_q: Query<&BrickTile>,
    mut question_q: Query<&mut QuestionBlock>,
    pipe_q: Query<&PipeTile>,
    stone_q: Query<&StoneTile>,
    ground_q: Query<&GroundTile>,
    flag_q: Query<&FlagPole>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok((mut player, mut tr)) = player_q.single_mut() else {
        return;
    };

    if player.dead_timer > 0.0 {
        player.vel.y -= GRAVITY * dt;
        player.vel.y = player.vel.y.max(-FALL_MAX);
        tr.translation.y += player.vel.y * dt;
        return;
    }

    if player.finished {
        let mut v = player.vel;
        v.x = WALK_SPEED * 0.7;
        v.y -= GRAVITY * dt;
        v.y = v.y.max(-FALL_MAX);
        let mut np = tr.translation.truncate() + v * dt;
        let p_size = player.state.size();
        let mut on_ground = false;
        for (_e, st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(np, p_size, sp, s.size) {
                let dx = np.x - sp.x;
                let dy = np.y - sp.y;
                let px = (p_size.x + s.size.x) * 0.5 - dx.abs();
                let py = (p_size.y + s.size.y) * 0.5 - dy.abs();
                if px > 0.0 && py > 0.0 {
                    if py < px {
                        if dy > 0.0 {
                            np.y += py;
                            v.y = 0.0;
                            on_ground = true;
                        } else {
                            np.y -= py;
                            v.y = 0.0;
                        }
                    } else if dx > 0.0 {
                        np.x += px;
                        v.x = 0.0;
                    } else {
                        np.x -= px;
                        v.x = 0.0;
                    }
                }
            }
        }
        tr.translation.x = np.x;
        tr.translation.y = np.y;
        player.vel = v;
        player.on_ground = on_ground;
        return;
    }

    let g = if player.jumping && player.vel.y > 0.0 {
        JUMP_HOLD_GRAVITY
    } else {
        GRAVITY
    };
    player.vel.y -= g * dt;
    player.vel.y = player.vel.y.max(-FALL_MAX);

    let mut pos = tr.translation.truncate();
    let p_size = player.state.size();

    pos.x += player.vel.x * dt;
    let left_min = PLAYER_W * 0.5;
    if pos.x < left_min {
        pos.x = left_min;
        player.vel.x = 0.0;
    }
    let right_max = level_world_max_x() - PLAYER_W * 0.5;
    if pos.x > right_max {
        pos.x = right_max;
    }
    for (_e, st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dx = pos.x - sp.x;
            let push = (p_size.x + s.size.x) * 0.5 - dx.abs();
            if push > 0.0 {
                if dx > 0.0 {
                    pos.x += push;
                } else {
                    pos.x -= push;
                }
                player.vel.x = 0.0;
            }
        }
    }

    pos.y += player.vel.y * dt;
    let mut on_ground = false;
    let mut hit_above: Option<Entity> = None;
    for (e, st, s) in &solid_q {
        let sp = st.translation.truncate();
        if aabb_overlap(pos, p_size, sp, s.size) {
            let dy = pos.y - sp.y;
            let push = (p_size.y + s.size.y) * 0.5 - dy.abs();
            if push > 0.0 {
                if dy > 0.0 {
                    pos.y += push;
                    if player.vel.y < 0.0 {
                        player.vel.y = 0.0;
                    }
                    on_ground = true;
                } else {
                    pos.y -= push;
                    if player.vel.y > 0.0 {
                        player.vel.y = 0.0;
                    }
                    hit_above = Some(e);
                }
            }
        }
    }
    player.on_ground = on_ground;

    if let Some(e) = hit_above {
        if let Ok(mut q) = question_q.get_mut(e) {
            if !q.used {
                q.used = true;
                q.bump_t = 0.18;
            } else if q.bump_t <= 0.0 {
                q.bump_t = 0.10;
            }
        } else if brick_q.get(e).is_ok() {
        } else if pipe_q.get(e).is_ok() || stone_q.get(e).is_ok() || ground_q.get(e).is_ok() {
        } else if flag_q.get(e).is_ok() {
        }
    }

    tr.translation.x = pos.x;
    tr.translation.y = pos.y;

    if pos.y < FALL_DEATH_Y && player.dead_timer <= 0.0 {
        player.dead_timer = 2.0;
        player.vel = Vec2::new(0.0, 380.0);
    }

    if player.invincible > 0.0 {
        player.invincible -= dt;
    }
}
