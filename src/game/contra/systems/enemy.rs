use bevy::prelude::*;

use crate::common::constants::ARENA_W;
use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::aabb_overlap;
use super::super::setup_actors::spawn_enemy_bullet;

pub fn contra_enemy_ai(
    time: Res<Time>,
    session: Res<GameSession>,
    mut commands: Commands,
    mut enemy_q: Query<(&mut ContraEnemy, &mut Transform), Without<ContraSolid>>,
    solid_q: Query<(&Transform, &ContraSolid), Without<ContraEnemy>>,
    player_q: Query<(&ContraPlayer, &Transform), Without<ContraEnemy>>,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraEnemy>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs().min(0.033);
    let Ok(cam) = cam_q.single() else { return };
    let cam_x = cam.translation.x;
    let player_pos = player_q
        .single()
        .map(|(p, t)| {
            if p.dead_timer > 0.0 {
                None
            } else {
                Some(t.translation.truncate())
            }
        })
        .unwrap_or(None);

    for (mut enemy, mut tr) in &mut enemy_q {
        let mut pos = tr.translation.truncate();
        enemy.ai_t += dt;
        match enemy.kind {
            EnemyKind::Soldier => {
                if let Some(pp) = player_pos {
                    let dx = pp.x - pos.x;
                    if dx.abs() > 24.0 {
                        enemy.facing = dx.signum();
                    }
                    enemy.vel.x = enemy.facing * ENEMY_SPEED;
                } else {
                    enemy.vel.x = enemy.facing * ENEMY_SPEED;
                }
            }
            EnemyKind::Sniper => {
                enemy.vel.x = 0.0;
                if let Some(pp) = player_pos {
                    enemy.facing = (pp.x - pos.x).signum().max(-1.0).min(1.0);
                    if enemy.facing == 0.0 {
                        enemy.facing = -1.0;
                    }
                }
            }
            EnemyKind::Jumper => {
                if enemy.on_ground {
                    if enemy.ai_t > 1.0 {
                        enemy.vel.y = JUMPER_JUMP_VEL;
                        enemy.on_ground = false;
                        enemy.ai_t = 0.0;
                    }
                    if let Some(pp) = player_pos {
                        let dx = pp.x - pos.x;
                        if dx.abs() > 24.0 {
                            enemy.facing = dx.signum();
                        }
                    }
                    enemy.vel.x = enemy.facing * ENEMY_SPEED * 0.8;
                } else {
                    enemy.vel.x = enemy.facing * ENEMY_SPEED * 0.6;
                }
            }
        }

        enemy.vel.y -= GRAVITY * dt;
        enemy.vel.y = enemy.vel.y.max(-FALL_MAX);

        let size = Vec2::new(ENEMY_W, ENEMY_H);
        pos.x += enemy.vel.x * dt;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dx = pos.x - sp.x;
                let push = (size.x + s.size.x) * 0.5 - dx.abs();
                if push > 0.0 {
                    if dx > 0.0 {
                        pos.x += push;
                    } else {
                        pos.x -= push;
                    }
                    enemy.vel.x = 0.0;
                    enemy.facing = -enemy.facing;
                }
            }
        }
        pos.y += enemy.vel.y * dt;
        let mut grounded = false;
        for (st, s) in &solid_q {
            let sp = st.translation.truncate();
            if aabb_overlap(pos, size, sp, s.size) {
                let dy = pos.y - sp.y;
                let push = (size.y + s.size.y) * 0.5 - dy.abs();
                if push > 0.0 {
                    if dy > 0.0 {
                        pos.y += push;
                        if enemy.vel.y < 0.0 {
                            grounded = true;
                            enemy.vel.y = 0.0;
                        }
                    } else {
                        pos.y -= push;
                        if enemy.vel.y > 0.0 {
                            enemy.vel.y = 0.0;
                        }
                    }
                }
            }
        }
        enemy.on_ground = grounded;
        tr.translation.x = pos.x;
        tr.translation.y = pos.y;
        let s = if enemy.facing >= 0.0 { 1.0 } else { -1.0 };
        tr.scale.x = s;

        enemy.fire_cd = (enemy.fire_cd - dt).max(0.0);
        if enemy.fire_cd <= 0.0 {
            if let Some(pp) = player_pos {
                let mut dir = pp - pos;
                if dir.x.abs() < 4.0 {
                    dir.x = enemy.facing * 1.0;
                }
                if (pos.x - cam_x).abs() < ARENA_W * 0.55 {
                    spawn_enemy_bullet(&mut commands, pos, dir);
                    enemy.fire_cd = match enemy.kind {
                        EnemyKind::Sniper => SNIPER_FIRE_CD,
                        _ => ENEMY_FIRE_CD,
                    };
                } else {
                    enemy.fire_cd = ENEMY_FIRE_CD * 0.6;
                }
            }
        }
    }
}
