use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::game::model::{Collider, GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{PLAYER_BULLET_SPEED, POWERUP_SIZE, STAGE_TOTAL_ENEMIES, TANK_SIZE, TILE};
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
    // 所有坦克的位置，供铲子刷钢墙时做占位检测
    let tank_positions: Vec<Vec2> = players
        .iter()
        .map(|(t, _, _, _)| t.translation.truncate())
        .collect();
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
        let bonus = apply_powerup(
            powerup.kind,
            id,
            &mut stage,
            &mut commands,
            &mut players,
            &enemies,
            &bases,
            &tank_positions,
        );
        session.score += 500 + bonus;
        sfx.write(PlaySfx(SfxKind::Powerup));
        commands.entity(pe).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_powerup(
    kind: PowerUpKind,
    player_id: usize,
    stage: &mut TankStage,
    commands: &mut Commands,
    players: &mut PlayerQuery,
    enemies: &Query<Entity, With<EnemyTankFC>>,
    bases: &Query<&Transform, With<BaseFC>>,
    tank_positions: &[Vec2],
) -> u32 {
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
            let destroyed = enemies.iter().count() as u8;
            for e in enemies.iter() {
                commands.entity(e).despawn();
            }
            stage.kills = grenade_kill_total(stage.kills, destroyed);
            return destroyed as u32 * 100;
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
                    let cell = bp + off;
                    // 占位检测：这几格原本是砖墙，玩家把它打掉后可以站进去。
                    // 无条件刷 32×32 钢墙会把人直接封死在里面，且没有脱困路径。
                    if tank_positions
                        .iter()
                        .any(|p| aabb_overlap(*p, Vec2::splat(TANK_SIZE - 2.0), cell, Vec2::splat(TILE)))
                    {
                        continue;
                    }
                    spawn_steel_at(commands, cell);
                }
            }
        }
    }
    0
}

fn grenade_kill_total(current: u8, destroyed: u8) -> u8 {
    current.saturating_add(destroyed).min(STAGE_TOTAL_ENEMIES)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grenade_counts_destroyed_enemies_toward_stage_clear() {
        assert_eq!(grenade_kill_total(7, 4), 11);
        assert_eq!(grenade_kill_total(19, 4), STAGE_TOTAL_ENEMIES);
    }
}
