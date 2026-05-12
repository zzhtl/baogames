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
    stage.spawn_idx += 1;
    let pos = tile_center(col, 0);
    spawn_spawn_effect(&mut commands, pos, TankSide::Enemy, None);
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
                TankSide::Enemy => spawn_enemy_tank(&mut commands, pos),
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
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(0));
        stage.p1_respawn = RESPAWN_TIME;
    }
    if !p2_alive && stage.p2_lives > 0 && stage.p2_respawn <= 0.0 && stage.p2_respawn != -1.0 {
        let pos = tile_center(PLAYER2_SPAWN.0, PLAYER2_SPAWN.1);
        spawn_spawn_effect(&mut commands, pos, TankSide::Player, Some(1));
        stage.p2_respawn = RESPAWN_TIME;
    }
}
