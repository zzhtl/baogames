use bevy::prelude::*;

use crate::common::constants::{ARENA_H, ARENA_W};
use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::geometry::player_size;
use super::super::resources::ContraStage;

pub fn contra_player_respawn(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    stage: Res<ContraStage>,
    mut player_q: Query<(&mut ContraPlayer, &mut Transform, &mut Sprite), Without<Camera>>,
    cam_q: Query<&Transform, (With<Camera>, Without<ContraPlayer>)>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut player, mut tr, mut sprite)) = player_q.single_mut() else {
        return;
    };
    if player.dead_timer > 0.0 {
        player.dead_timer -= dt;
        if player.dead_timer <= 0.0 {
            if session.lives <= 0 {
                if apply_game_over(&mut session, &mut save) {
                    save.store();
                }
                return;
            }
            let cam_x = cam_q
                .single()
                .map(|t| t.translation.x)
                .unwrap_or(stage.player_spawn.x);
            let mut x = (cam_x - ARENA_W * 0.35).max(stage.player_spawn.x);
            if stage.boss_spawned && !stage.boss_dead {
                x = x.min(stage.boss_x - BOSS_W * 0.5 - PLAYER_W * 0.5 - 16.0);
            }
            tr.translation.x = x;
            tr.translation.y = ARENA_H * 0.5 - 40.0;
            player.vel = Vec2::ZERO;
            player.dead_timer = 0.0;
            player.invincible = INVINCIBLE_TIME;
            player.weapon = Weapon::M;
            player.fire_cd = 0.0;
            player.prone = false;
            sprite.custom_size = Some(player_size(false));
        }
    }
}

/// game over 时把分数同步到 SaveData。返回 true 表示需要持久化。
fn apply_game_over(session: &mut GameSession, save: &mut SaveData) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = false;
    session.status = "GAME OVER  Enter 重玩 / Backspace 返回菜单".to_string();
    let idx = session.kind.index();
    if session.score > save.high_scores[idx] {
        save.high_scores[idx] = session.score;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::model::GameKind;

    fn fresh_session() -> GameSession {
        GameSession {
            kind: GameKind::Contra,
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
    fn game_over_marks_session_lost() {
        let mut session = fresh_session();
        let mut save = SaveData::default();
        apply_game_over(&mut session, &mut save);
        assert!(session.finished);
        assert!(!session.won);
    }

    #[test]
    fn game_over_records_new_high_score() {
        let mut session = fresh_session();
        session.score = 2000;
        let mut save = SaveData::default();
        let dirty = apply_game_over(&mut session, &mut save);
        assert!(dirty);
        assert_eq!(save.high_scores[GameKind::Contra.index()], 2000);
    }

    #[test]
    fn game_over_keeps_better_record() {
        let mut session = fresh_session();
        session.score = 100;
        let mut save = SaveData::default();
        save.high_scores[GameKind::Contra.index()] = 9999;
        let dirty = apply_game_over(&mut session, &mut save);
        assert!(!dirty);
        assert_eq!(save.high_scores[GameKind::Contra.index()], 9999);
    }

    #[test]
    fn game_over_idempotent() {
        let mut session = fresh_session();
        session.score = 500;
        let mut save = SaveData::default();
        let first = apply_game_over(&mut session, &mut save);
        let second = apply_game_over(&mut session, &mut save);
        assert!(first);
        assert!(!second);
    }
}
