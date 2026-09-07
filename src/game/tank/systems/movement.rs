use bevy::prelude::*;

use crate::common::collide::{Solid as Solid_, depenetrate};
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
        let blocked_at = |p: Vec2| {
            blocker_data
                .iter()
                .any(|(bp, bs)| aabb_overlap(p, tank_size, *bp, *bs))
                || tank_data
                    .iter()
                    .any(|(e, op, os)| *e != self_entity && aabb_overlap(p, tank_size, *op, *os))
        };

        // 分轴回退：整体走不动就单轴试一次，贴着墙也能滑过去。
        // 原来是「全或无」——新位置一重叠就整帧不动，而坦克 collider 30 对通道 32
        // 单侧只有 1 单位余量，一旦被别的坦克或铲子刷出的钢墙压住就四个方向全阻塞，
        // 永久冻结且没有任何自愈路径。
        let here = tf.translation.truncate();
        let mut next = here;
        if !blocked_at(Vec2::new(clamped.x, here.y)) {
            next.x = clamped.x;
        }
        if !blocked_at(Vec2::new(next.x, clamped.y)) {
            next.y = clamped.y;
        }
        if next == here {
            tank.coast_left = 0.0;
            if blocked_at(here) {
                // 已经被封在墙里：沿最小穿透轴顶出去，别把玩家永久卡死
                let solids: Vec<Solid_> = blocker_data
                    .iter()
                    .map(|(bp, bs)| Solid_::fixed(*bp, *bs))
                    .collect();
                next = depenetrate(here, tank_size, &solids);
            }
        }
        let moved = here.distance_squared(next) > 0.001;
        tf.translation.x = next.x;
        tf.translation.y = next.y;
        tank.moving = moved;
        if moved {
            tank.motion_t += delta;
        }
    }
}
