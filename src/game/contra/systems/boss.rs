use bevy::prelude::*;

use crate::game::model::GameSession;

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::ContraStage;
use super::super::setup_actors::{spawn_enemy_bullet, spawn_explosion};

pub fn contra_boss_update(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut commands: Commands,
    mut stage: ResMut<ContraStage>,
    mut boss_q: Query<(Entity, &mut ContraBoss, &Transform)>,
    mut turret_q: Query<(&mut ContraTurret, &Transform), Without<ContraBoss>>,
    player_q: Query<&Transform, (With<ContraPlayer>, Without<ContraBoss>, Without<ContraTurret>)>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let player_pos = player_q.single().ok().map(|t| t.translation.truncate());
    for (be, mut boss, btr) in &mut boss_q {
        boss.spawn_t += dt;
        if boss.flash_t > 0.0 {
            boss.flash_t = (boss.flash_t - dt).max(0.0);
        }
        if boss.die_t > 0.0 {
            boss.die_t -= dt;
            if (boss.die_t * 8.0).fract() < dt * 8.0 {
                let dx = ((boss.die_t * 13.0).sin()) * BOSS_W * 0.4;
                let dy = ((boss.die_t * 17.0).cos()) * BOSS_H * 0.4;
                spawn_explosion(
                    &mut commands,
                    btr.translation.truncate() + Vec2::new(dx, dy),
                    36.0,
                    0.4,
                );
            }
            if boss.die_t <= 0.0 {
                spawn_explosion(&mut commands, btr.translation.truncate(), 80.0, 0.7);
                commands.entity(be).despawn();
                stage.boss_dead = true;
                session.finished = true;
                session.won = true;
                session.status = "STAGE CLEAR！Enter 重玩 / Backspace 返回菜单".to_string();
                session.score += 5000;
            }
            continue;
        }
        for (mut turret, ttr) in &mut turret_q {
            turret.fire_cd = (turret.fire_cd - dt).max(0.0);
            if turret.fire_cd <= 0.0 {
                if let Some(pp) = player_pos {
                    let origin = ttr.translation.truncate() + Vec2::new(-22.0, 0.0);
                    let mut dir = pp - origin;
                    if dir.length_squared() < 4.0 {
                        dir = Vec2::new(-1.0, 0.0);
                    }
                    spawn_enemy_bullet(&mut commands, origin, dir);
                    turret.fire_cd = TURRET_FIRE_CD;
                }
            }
        }
    }
}
