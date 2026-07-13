use bevy::prelude::*;

use crate::common::settings::GameplayProfile;
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::LEVEL_TIME;
use super::super::resources::MarioStage;
use super::super::setup_actors::build_player_visual;

pub fn mario_checkpoint_update(
    save: Res<SaveData>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<(&MarioPlayer, &Transform)>,
) {
    if save.settings.gameplay_profile != GameplayProfile::Assist {
        return;
    }
    let Ok((player, transform)) = player_q.single() else {
        return;
    };
    let x = transform.translation.x;
    if player.dead_timer <= 0.0
        && !player.finished
        && player.on_ground
        && x >= stage.next_checkpoint_x
    {
        stage.player_spawn = transform.translation.truncate();
        stage.next_checkpoint_x += 40.0 * super::super::constants::TILE;
    }
}

pub fn mario_respawn(
    time: Res<Time>,
    mut commands: Commands,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    mut player_q: Query<(Entity, &mut MarioPlayer, &mut Transform)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((player_e, mut player, mut tr)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= time.delta_secs();
        if player.dead_timer <= 0.0 {
            if session.lives <= 0 {
                if apply_game_over(&mut session, &mut save) {
                    save.store();
                }
                return;
            }
            tr.translation.x = stage.player_spawn.x;
            tr.translation.y = stage.player_spawn.y;
            player.vel = Vec2::ZERO;
            player.on_ground = false;
            player.coyote_ticks = 0;
            player.invincible = 1.6;
            player.dead_timer = 0.0;
            player.state = PowerState::Small;
            player.transform_t = 0.0;
            player.fire_cd = 0.0;
            commands.entity(player_e).despawn_related::<Children>();
            build_player_visual(&mut commands, player_e, PowerState::Small);
            stage.time_left = LEVEL_TIME;
        }
    }
}

/// game over 时更新 SaveData 中的最高分；返回 true 表示需要持久化。
/// 抽成纯函数便于单测，IO 由调用方决定。
fn apply_game_over(session: &mut GameSession, save: &mut SaveData) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = false;
    session.status = format!("生命耗尽 · 得分 {}", session.score);
    let idx = session.kind.index();
    let mut dirty = false;
    if session.score > save.high_scores[idx] {
        save.high_scores[idx] = session.score;
        dirty = true;
    }
    dirty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::GameKind;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::SuperMario,
            level: 1,
            score: 0,
            lives: 0,
            paused: false,
            finished: false,
            won: false,
            status: String::new(),
        }
    }

    #[test]
    fn game_over_updates_high_score_when_score_is_higher() {
        let mut session = fresh_session();
        session.score = 1234;
        let mut save = SaveData::default();
        let dirty = apply_game_over(&mut session, &mut save);
        assert!(dirty);
        assert!(session.finished);
        assert!(!session.won);
        assert_eq!(save.high_scores[GameKind::SuperMario.index()], 1234);
    }

    #[test]
    fn game_over_keeps_better_existing_record() {
        let mut session = fresh_session();
        session.score = 100;
        let mut save = SaveData::default();
        save.high_scores[GameKind::SuperMario.index()] = 9999;
        let dirty = apply_game_over(&mut session, &mut save);
        // 没破纪录 → 不需要持久化
        assert!(!dirty);
        assert_eq!(save.high_scores[GameKind::SuperMario.index()], 9999);
    }

    #[test]
    fn game_over_is_idempotent() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        let first = apply_game_over(&mut session, &mut save);
        let second = apply_game_over(&mut session, &mut save);
        // 第一次未破纪录 (score=0)，所以也不 dirty
        assert!(!first);
        assert!(!second);
        assert!(session.finished);
    }
}
