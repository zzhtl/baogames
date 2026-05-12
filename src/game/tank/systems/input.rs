use bevy::prelude::*;

use crate::common::input::input_for;
use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::TANK_SIZE;
use super::super::geometry::{play_max, play_min, snap_perpendicular};
use super::super::resources::TankStage;
use super::super::setup::{spawn_bullet, spawn_muzzle_flash};

pub fn tank_player_input(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    mut tanks: Query<
        (
            Entity,
            &PlayerTankFC,
            &mut Transform,
            &mut TankDir,
            &mut TankFC,
        ),
        Without<EnemyTankFC>,
    >,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let delta = time.delta_secs();
    if stage.p1_respawn > 0.0 {
        stage.p1_respawn = (stage.p1_respawn - delta).max(0.0);
    }
    if stage.p2_respawn > 0.0 {
        stage.p2_respawn = (stage.p2_respawn - delta).max(0.0);
    }

    for (entity, player, mut tf, mut dir, mut tank) in &mut tanks {
        let input = input_for(&keys, player.id);
        tank.fire_cd_left = (tank.fire_cd_left - delta).max(0.0);
        if tank.shield_left > 0.0 {
            tank.shield_left = (tank.shield_left - delta).max(0.0);
        }

        if let Some(want) = TankDir::from_input(input.move_dir) {
            if want != *dir {
                *dir = want;
                tf.rotation = Quat::from_rotation_z(want.rotation());
                snap_perpendicular(&mut tf.translation, want);
            }
        }

        if input.fire && tank.fire_cd_left <= 0.0 && tank.bullets_alive < tank.max_bullets {
            let muzzle = tf.translation.truncate() + dir.vec() * (TANK_SIZE * 0.5 + 2.0);
            // 只有炮口仍在场地内才允许射击，避免朝边界外开火立即销毁导致体感"打不出子弹"
            let pmin = play_min();
            let pmax = play_max();
            let in_field =
                muzzle.x > pmin.x && muzzle.x < pmax.x && muzzle.y > pmin.y && muzzle.y < pmax.y;
            if in_field {
                spawn_bullet(
                    &mut commands,
                    muzzle,
                    *dir,
                    tank.bullet_speed,
                    TankSide::Player,
                    1,
                    entity,
                );
                spawn_muzzle_flash(&mut commands, muzzle, *dir);
                tank.bullets_alive += 1;
                tank.fire_cd_left = tank.fire_cd;
            }
        }
    }
}
