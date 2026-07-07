use bevy::prelude::*;

use crate::game::model::{GameSession, SaveData};

use super::super::components::*;
use super::super::constants::*;
use super::super::resources::MarioStage;

pub fn mario_flag_check(
    mut session: ResMut<GameSession>,
    mut player_q: Query<(&mut MarioPlayer, &Transform)>,
    flag_q: Query<&Transform, (With<FlagPole>, Without<MarioPlayer>)>,
    mut banner_q: Query<&mut FlagBanner>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok((mut player, ptr)) = player_q.single_mut() else {
        return;
    };
    if player.finished || player.dead_timer > 0.0 {
        return;
    }
    let Ok(ftr) = flag_q.single() else {
        return;
    };
    let p_pos = ptr.translation.truncate();
    if (p_pos.x - ftr.translation.x).abs() < 16.0 && p_pos.y > FLOOR_Y {
        player.finished = true;
        let h_ratio = ((p_pos.y - FLOOR_Y) / (10.0 * TILE)).clamp(0.0, 1.0);
        let pts = (h_ratio * 5000.0) as u32 + 100;
        session.score = session.score.saturating_add(pts);
        for mut b in &mut banner_q {
            b.y_target = FLOOR_Y + TILE + 8.0;
            b.speed = 90.0;
        }
    }
}

pub fn mario_flag_anim(
    time: Res<Time>,
    session: Res<GameSession>,
    mut q: Query<(&mut Transform, &FlagBanner)>,
) {
    // 只挡暂停：通关(finished)后旗帜仍需落到底
    if session.paused {
        return;
    }
    let dt = time.delta_secs();
    for (mut tr, b) in &mut q {
        if tr.translation.y > b.y_target {
            tr.translation.y = (tr.translation.y - b.speed * dt).max(b.y_target);
        }
    }
}

pub fn mario_finish_seq(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MarioStage>,
    player_q: Query<&MarioPlayer>,
    mut save: ResMut<SaveData>,
) {
    if session.paused || session.finished {
        return;
    }
    let Ok(player) = player_q.single() else {
        return;
    };
    if !player.finished {
        return;
    }
    stage.finish_timer += time.delta_secs();
    if stage.time_left > 0.0 {
        let take = (200.0 * time.delta_secs()).min(stage.time_left);
        stage.time_left -= take;
        session.score = session.score.saturating_add(take.ceil() as u32 * 50);
    }
    if stage.finish_timer > 3.5 && apply_finish(&mut session, &mut save) {
        save.store();
    }
}

/// 通关时同步分数和解锁关卡到 SaveData，返回 true 表示需要持久化。
fn apply_finish(session: &mut GameSession, save: &mut SaveData) -> bool {
    if session.finished {
        return false;
    }
    session.finished = true;
    session.won = true;
    session.status = format!("冲上旗杆 · 得分 {}", session.score);
    let idx = session.kind.index();
    let mut dirty = false;
    if session.score > save.high_scores[idx] {
        save.high_scores[idx] = session.score;
        dirty = true;
    }
    let next = (session.level + 1).min(4);
    if next > save.unlocked_levels[idx] {
        save.unlocked_levels[idx] = next;
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
            lives: 3,
            paused: false,
            finished: false,
            won: false,
            status: String::new(),
        }
    }

    #[test]
    fn finish_unlocks_next_level_and_updates_score() {
        let mut session = fresh_session();
        session.score = 5000;
        let mut save = SaveData::default();
        let dirty = apply_finish(&mut session, &mut save);
        assert!(dirty);
        assert!(session.finished);
        assert!(session.won);
        assert_eq!(save.high_scores[GameKind::SuperMario.index()], 5000);
        assert_eq!(save.unlocked_levels[GameKind::SuperMario.index()], 2);
    }

    #[test]
    fn finish_caps_unlocked_level_at_four() {
        // 玛丽只有 4 关
        let mut session = fresh_session();
        session.level = 4;
        let mut save = SaveData::default();
        apply_finish(&mut session, &mut save);
        assert_eq!(save.unlocked_levels[GameKind::SuperMario.index()], 4);
    }

    #[test]
    fn finish_keeps_existing_higher_high_score() {
        let mut session = fresh_session();
        session.score = 100;
        let mut save = SaveData::default();
        save.high_scores[GameKind::SuperMario.index()] = 9999;
        let dirty = apply_finish(&mut session, &mut save);
        // 关卡解锁推进时仍 dirty，但高分不被回退
        assert!(dirty);
        assert_eq!(save.high_scores[GameKind::SuperMario.index()], 9999);
    }

    #[test]
    fn finish_idempotent() {
        let mut session = fresh_session();
        session.score = 1000;
        let mut save = SaveData::default();
        let first = apply_finish(&mut session, &mut save);
        let second = apply_finish(&mut session, &mut save);
        assert!(first);
        assert!(!second);
    }

    #[test]
    fn finish_no_progress_no_dirty() {
        let mut session = fresh_session();
        session.level = 4;
        let mut save = SaveData::default();
        save.high_scores[GameKind::SuperMario.index()] = 9999;
        save.unlocked_levels[GameKind::SuperMario.index()] = 4;
        let dirty = apply_finish(&mut session, &mut save);
        // 已经在最高关、且高分没破，本次通关不写盘
        assert!(!dirty);
        assert!(session.finished);
        assert!(session.won);
    }
}
