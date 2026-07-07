use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{PLAYER_BULLET_SPEED, POWERUP_SIZE};
use super::super::geometry::aabb_overlap;
use super::super::resources::TankStage;
use super::super::setup::spawn_steel_at;

type PlayerQuery<'w, 's> =
    Query<'w, 's, (&'static Transform, &'static mut TankFC, &'static PlayerTankFC, &'static Collider)>;

pub fn tank_powerup_pickup(
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<TankStage>,
    powerups: Query<(Entity, &Transform, &PowerUp)>,
    mut players: PlayerQuery,
    enemies: Query<Entity, With<EnemyTankFC>>,
    bases: Query<&Transform, With<BaseFC>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    for (pe, ptr, powerup) in &powerups {
        let pp = ptr.translation.truncate();
        // 找到吃到道具的玩家 id（先只读判定，避免与后续可变借用冲突）
        let mut hit_id: Option<usize> = None;
        for (player_tr, _, player, col) in &players {
            if aabb_overlap(
                pp,
                Vec2::splat(POWERUP_SIZE * 0.8),
                player_tr.translation.truncate(),
                col.size,
            ) {
                hit_id = Some(player.id);
                break;
            }
        }
        let Some(id) = hit_id else { continue };
        apply_powerup(
            powerup.kind,
            id,
            &mut stage,
            &mut commands,
            &mut players,
            &enemies,
            &bases,
        );
        session.score += 500;
        sfx.write(PlaySfx(SfxKind::Powerup));
        commands.entity(pe).despawn();
    }
}

fn apply_powerup(
    kind: PowerUpKind,
    player_id: usize,
    stage: &mut TankStage,
    commands: &mut Commands,
    players: &mut PlayerQuery,
    enemies: &Query<Entity, With<EnemyTankFC>>,
    bases: &Query<&Transform, With<BaseFC>>,
) {
    match kind {
        PowerUpKind::Star => {
            for (_, mut tank, player, _) in players.iter_mut() {
                if player.id == player_id {
                    tank.max_bullets = (tank.max_bullets + 1).min(2);
                    tank.bullet_speed = (tank.bullet_speed * 1.15).min(PLAYER_BULLET_SPEED * 1.6);
                }
            }
        }
        PowerUpKind::Helmet => {
            for (_, mut tank, player, _) in players.iter_mut() {
                if player.id == player_id {
                    tank.shield_left = 8.0;
                }
            }
        }
        PowerUpKind::Grenade => {
            for e in enemies.iter() {
                commands.entity(e).despawn();
            }
        }
        PowerUpKind::Tank => {
            if player_id == 0 {
                stage.p1_lives += 1;
            } else {
                stage.p2_lives += 1;
            }
        }
        PowerUpKind::Clock => {
            stage.freeze_timer = 6.0;
        }
        PowerUpKind::Shovel => {
            if let Ok(base_tr) = bases.single() {
                let bp = base_tr.translation.truncate();
                for off in [
                    Vec2::new(0.0, 32.0),
                    Vec2::new(-32.0, 0.0),
                    Vec2::new(32.0, 0.0),
                    Vec2::new(-32.0, 32.0),
                    Vec2::new(32.0, 32.0),
                ] {
                    spawn_steel_at(commands, bp + off);
                }
            }
        }
    }
}

/// 冻结计时（时钟道具）：每帧递减，归零后敌人解冻。
pub fn tank_freeze_tick(time: Res<Time>, session: Res<GameSession>, mut stage: ResMut<TankStage>) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    if stage.freeze_timer > 0.0 {
        stage.freeze_timer = (stage.freeze_timer - time.delta_secs()).max(0.0);
    }
}
