use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::*;
use super::super::constants::{
    ENEMY_SPAWN_COLS, MAX_ALIVE_ENEMIES, PLAYER1_SPAWN, PLAYER2_SPAWN, RESPAWN_TIME, SPAWN_INTERVAL,
};
use super::super::geometry::tile_center;
use super::super::resources::TankStage;
use super::super::setup::{spawn_enemy_tank, spawn_player_tank, spawn_spawn_effect};

pub fn tank_enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    enemies: Query<&Transform, With<EnemyTankFC>>,
    pending_effects: Query<&SpawnEffect>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let alive: u8 = enemies.iter().count() as u8;
    let pending_enemies: u8 = pending_effects
        .iter()
        .filter(|e| e.side == TankSide::Enemy)
        .count() as u8;
    if alive + pending_enemies >= MAX_ALIVE_ENEMIES || stage.remaining_to_spawn == 0 {
        return;
    }
    stage.spawn_timer -= time.delta_secs();
    if stage.spawn_timer > 0.0 {
        return;
    }
    let col = ENEMY_SPAWN_COLS[stage.spawn_idx % ENEMY_SPAWN_COLS.len()];
    let kind = pick_enemy_kind(stage.spawn_idx, stage.stage_num);
    stage.spawn_idx += 1;
    let pos = tile_center(col, 0);
    spawn_spawn_effect(&mut commands, pos, TankSide::Enemy, None, Some(kind));
    stage.remaining_to_spawn -= 1;
    stage.spawn_timer = SPAWN_INTERVAL;
}

pub fn tank_spawn_effect(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut effects: Query<(Entity, &mut SpawnEffect, &mut Sprite)>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    for (entity, mut effect, mut sprite) in &mut effects {
        effect.timer.tick(time.delta());
        let t = effect.timer.fraction();
        // 闪烁：高低亮交替
        let pulse = ((t * 18.0).sin() * 0.5 + 0.5) as f32;
        sprite.color = Color::srgb(0.8 + 0.2 * pulse, 0.9, 1.0);
        if effect.timer.just_finished() {
            let pos = effect.spawn_pos;
            let side = effect.side;
            let player_id = effect.player_id;
            commands.entity(entity).despawn();
            match side {
                TankSide::Enemy => spawn_enemy_tank(
                    &mut commands,
                    pos,
                    effect.enemy_kind.unwrap_or(EnemyTankKind::Basic),
                ),
                TankSide::Player => {
                    if let Some(id) = player_id {
                        spawn_player_tank(&mut commands, id, pos);
                    }
                }
            }
        }
    }
}

pub fn tank_player_respawn(
    mut commands: Commands,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    players: Query<&PlayerTankFC>,
    spawning: Query<&SpawnEffect>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished {
        return;
    }
    let _ = time;
    let p1_alive =
        players.iter().any(|p| p.id == 0) || spawning.iter().any(|e| e.player_id == Some(0));
    let p2_alive =
        players.iter().any(|p| p.id == 1) || spawning.iter().any(|e| e.player_id == Some(1));

    if !p1_alive && stage.p1_lives > 0 && stage.p1_respawn <= 0.0 && stage.p1_respawn != -1.0 {
        let pos = tile_center(PLAYER1_SPAWN.0, PLAYER1_SPAWN.1);
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(0), None);
        stage.p1_respawn = RESPAWN_TIME;
    }
    if !p2_alive && stage.p2_lives > 0 && stage.p2_respawn <= 0.0 && stage.p2_respawn != -1.0 {
        let pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(1), None);
        stage.p2_respawn = RESPAWN_TIME;
    }
}

/// 按生成序号与关卡决定敌坦克兵种：多数普通，穿插快速/重炮，越靠后(关卡越高)越多装甲。
fn pick_enemy_kind(idx: usize, stage_num: u8) -> EnemyTankKind {
    let armor_from = (17usize.saturating_sub(stage_num as usize)).max(8);
    if idx >= armor_from {
        EnemyTankKind::Armor
    } else if idx % 5 == 2 {
        EnemyTankKind::Power
    } else if idx % 3 == 1 {
        EnemyTankKind::Fast
    } else {
        EnemyTankKind::Basic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enemy_kinds_cover_all_four_over_a_stage() {
        let kinds: Vec<EnemyTankKind> = (0..20).map(|i| pick_enemy_kind(i, 5)).collect();
        for k in [
            EnemyTankKind::Basic,
            EnemyTankKind::Fast,
            EnemyTankKind::Power,
            EnemyTankKind::Armor,
        ] {
            assert!(kinds.contains(&k), "一关 20 只里应出现 {k:?}");
        }
    }

    #[test]
    fn higher_stage_has_more_armor() {
        let armor = |stage| {
            (0..20)
                .filter(|&i| pick_enemy_kind(i, stage) == EnemyTankKind::Armor)
                .count()
        };
        assert!(armor(7) > armor(1), "高关卡装甲坦克应更多");
    }
}
