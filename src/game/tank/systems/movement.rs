use bevy::prelude::*;

use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{ICE_COAST_TIME, TANK_SIZE, TILE};
use super::super::geometry::{aabb_overlap, play_max, play_min};
use super::super::resources::{TankControls, TankStage};

pub fn tank_movement(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<TankStage>,
    controls: Res<TankControls>,
    mut tanks: Query<(
        Entity,
        &mut Transform,
        &TankDir,
        &mut TankFC,
        &Collider,
        Option<&PlayerTankFC>,
    )>,
    blockers: Query<
        (&Transform, &Collider),
        (
            Or<(With<BrickFC>, With<SteelFC>, With<WaterFC>, With<BaseFC>)>,
            Without<TankFC>,
        ),
    >,
    ice: Query<&Transform, (With<IceFC>, Without<TankFC>)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    let blocker_data: Vec<(Vec2, Vec2)> = blockers
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    let tank_data: Vec<(Entity, Vec2, Vec2)> = tanks
        .iter()
        .map(|(e, t, _, _, c, _)| (e, t.translation.truncate(), c.size))
        .collect();

    for (self_entity, mut tf, dir, mut tank, _collider, player) in &mut tanks {
        tank.moving = false;
        tank.hit_t = (tank.hit_t - delta).max(0.0);
        // 玩家：仅按住方向键时才推进
        let advance = if let Some(p) = player {
            let input_held = controls.movement(p.id).length_squared() > 0.05;
            let pos = tf.translation.truncate();
            let on_ice = ice.iter().any(|ice_tf| {
                aabb_overlap(
                    pos,
                    Vec2::splat(TANK_SIZE * 0.65),
                    ice_tf.translation.truncate(),
                    Vec2::splat(TILE),
                )
            });
            if input_held {
                if on_ice {
                    tank.coast_left = ICE_COAST_TIME;
                } else {
                    tank.coast_left = (tank.coast_left - delta).max(0.0);
                }
                true
            } else if tank.coast_left > 0.0 {
                tank.coast_left = (tank.coast_left - delta).max(0.0);
                true
            } else {
                false
            }
        } else {
            // 敌人：被时钟冻结时停止移动
            stage.freeze_timer <= 0.0
        };
        if !advance {
            continue;
        }
        let step = dir.vec() * tank.speed * delta;
        let new_pos = tf.translation.truncate() + step;
        let half = (TANK_SIZE - 2.0) * 0.5;

        let pmin = play_min();
        let pmax = play_max();
        let clamped = Vec2::new(
            new_pos.x.clamp(pmin.x + half + 1.0, pmax.x - half - 1.0),
            new_pos.y.clamp(pmin.y + half + 1.0, pmax.y - half - 1.0),
        );

        let tank_size = Vec2::splat(TANK_SIZE - 2.0);
        let mut blocked = false;
        for (bp, bs) in &blocker_data {
            if aabb_overlap(clamped, tank_size, *bp, *bs) {
                blocked = true;
                break;
            }
        }
        if !blocked {
            for (other_e, op, os) in &tank_data {
                if *other_e == self_entity {
                    continue;
                }
                if aabb_overlap(clamped, tank_size, *op, *os) {
                    blocked = true;
                    break;
                }
            }
        }
        if !blocked {
            let moved = tf.translation.truncate().distance_squared(clamped) > 0.001;
            tf.translation.x = clamped.x;
            tf.translation.y = clamped.y;
            tank.moving = moved;
            if moved {
                tank.motion_t += delta;
            }
        } else {
            tank.coast_left = 0.0;
        }
    }
}
