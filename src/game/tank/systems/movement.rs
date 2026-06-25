use bevy::prelude::*;

use crate::common::input::input_for;
use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::TANK_SIZE;
use super::super::geometry::{aabb_overlap, play_max, play_min};
use super::super::resources::TankStage;

pub fn tank_movement(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<TankStage>,
    keys: Res<ButtonInput<KeyCode>>,
    mut tanks: Query<(
        Entity,
        &mut Transform,
        &TankDir,
        &TankFC,
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

    for (self_entity, mut tf, dir, tank, _collider, player) in &mut tanks {
        // 玩家：仅按住方向键时才推进
        let advance = if let Some(p) = player {
            input_for(&keys, p.id).move_dir.length_squared() > 0.05
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
            tf.translation.x = clamped.x;
            tf.translation.y = clamped.y;
        }
    }
}
