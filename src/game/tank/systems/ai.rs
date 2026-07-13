use bevy::prelude::*;
use rand::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{TANK_SIZE, TURN_WINDOW};
use super::super::geometry::{aabb_overlap, play_max, play_min, try_turn_at_lane};
use super::super::resources::TankStage;
use super::super::setup::{spawn_bullet, spawn_muzzle_flash};

pub fn tank_enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<TankStage>,
    mut enemies: Query<
        (
            Entity,
            &mut TankDir,
            &mut Transform,
            &mut TankFC,
            &mut EnemyTankFC,
        ),
        With<EnemyTankFC>,
    >,
    blockers: Query<
        (&Transform, &Collider),
        (
            Or<(With<BrickFC>, With<SteelFC>, With<WaterFC>, With<BaseFC>)>,
            Without<TankFC>,
        ),
    >,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    // 时钟道具：冻结期间敌人不转向、不开火
    if stage.freeze_timer > 0.0 {
        return;
    }
    let delta = time.delta_secs();
    let blocker_data: Vec<(Vec2, Vec2)> = blockers
        .iter()
        .map(|(t, c)| (t.translation.truncate(), c.size))
        .collect();
    let mut rng = thread_rng();

    for (entity, mut dir, mut tf, mut tank, mut ai) in &mut enemies {
        tank.fire_cd_left = (tank.fire_cd_left - delta).max(0.0);
        ai.turn_timer -= delta;

        // 卡墙检测：试探移动方向是否被挡
        let look_ahead = tf.translation.truncate() + dir.vec() * (tank.speed * delta + 1.0);
        let half_size = Vec2::splat(TANK_SIZE - 2.0);
        let pmin = play_min();
        let pmax = play_max();
        let half = (TANK_SIZE - 2.0) * 0.5;
        let out_of_bounds = look_ahead.x - half < pmin.x
            || look_ahead.x + half > pmax.x
            || look_ahead.y - half < pmin.y
            || look_ahead.y + half > pmax.y;
        let mut wall_blocked = out_of_bounds;
        if !wall_blocked {
            for (bp, bs) in &blocker_data {
                if aabb_overlap(look_ahead, half_size, *bp, *bs) {
                    wall_blocked = true;
                    break;
                }
            }
        }

        if wall_blocked || ai.turn_timer <= 0.0 {
            let candidates = [TankDir::Up, TankDir::Down, TankDir::Left, TankDir::Right];
            let mut shuffled = candidates;
            shuffled.shuffle(&mut rng);
            let mut new_dir = *dir;
            for c in shuffled {
                if c == *dir {
                    continue;
                }
                let test = tf.translation.truncate() + c.vec() * (tank.speed * delta + 1.0);
                let mut ok = !(test.x - half < pmin.x
                    || test.x + half > pmax.x
                    || test.y - half < pmin.y
                    || test.y + half > pmax.y);
                if ok {
                    for (bp, bs) in &blocker_data {
                        if aabb_overlap(test, half_size, *bp, *bs) {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    new_dir = c;
                    break;
                }
            }
            if new_dir != *dir
                && try_turn_at_lane(&mut tf.translation, *dir, new_dir, TURN_WINDOW)
            {
                *dir = new_dir;
                tf.rotation = Quat::from_rotation_z(new_dir.rotation());
            }
            ai.turn_timer = rng.gen_range(1.5..3.5);
        }

        // 开火
        if tank.fire_cd_left <= 0.0
            && tank.bullets_alive < tank.max_bullets
            && rng.r#gen::<f32>() < 0.04
        {
            let muzzle = tf.translation.truncate() + dir.vec() * (TANK_SIZE * 0.5 + 4.0);
            spawn_bullet(
                &mut commands,
                muzzle,
                *dir,
                tank.bullet_speed,
                TankSide::Enemy,
                1,
                entity,
            );
            spawn_muzzle_flash(&mut commands, muzzle, *dir);
            sfx.write(PlaySfx(SfxKind::Shoot));
            tank.bullets_alive += 1;
            tank.fire_cd_left = tank.fire_cd;
        }
    }
}
