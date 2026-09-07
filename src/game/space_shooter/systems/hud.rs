use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::super::components::*;
use super::super::constants::TOTAL_WAVES_BEFORE_BOSS;
use super::super::resources::SpaceState;

pub fn space_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    state: Res<SpaceState>,
    enemies: Query<&SpaceEnemy>,
    mut hud: Query<&mut Text2d, (With<SpaceHud>, Without<SpaceMessageText>)>,
    mut message: Query<&mut Text2d, (With<SpaceMessageText>, Without<SpaceHud>)>,
) {
    if session.kind != GameKind::SpaceShooter {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let high = save.high_scores[GameKind::SpaceShooter.index()].max(session.score);
        let boss_line = if state.boss_spawned && !state.boss_defeated {
            let boss_hp = enemies
                .iter()
                .find(|e| e.kind == EnemyKind::Boss)
                .map(|e| e.hp.max(0))
                .unwrap_or(0);
            format!("BOSS {}", gauge(boss_hp, state.boss_hp_max, 6))
        } else {
            String::new()
        };
        set_text(
            &mut t,
            &format!(
                "分数 {}\n纪录 {}\n生命 {}\n火力 LV.{}\n回避 x{}\n波次 {}/{}\n{}",
                session.score,
                high,
                session.lives.max(0),
                state.power,
                state.rolls,
                (state.wave_idx + 1).min(TOTAL_WAVES_BEFORE_BOSS),
                TOTAL_WAVES_BEFORE_BOSS,
                boss_line
            ),
        );
    }
    // 暂停/结束由统一覆盖层显示，这里只放玩法瞬时消息
    if let Ok(mut t) = message.single_mut() {
        let value = if state.message_clock > 0.0 && !session.finished {
            state.message.as_str()
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}

fn gauge(value: i32, max: i32, width: usize) -> String {
    let max = max.max(1);
    let value = value.clamp(0, max);
    let filled = ((value as f32 / max as f32) * width as f32).ceil() as usize;
    format!("{}{}", "#".repeat(filled), "-".repeat(width - filled))
}

#[cfg(test)]
mod tests {
    use super::gauge;

    #[test]
    fn boss_gauge_clamps_and_preserves_width() {
        assert_eq!(gauge(80, 80, 8), "########");
        assert_eq!(gauge(0, 80, 8), "--------");
        assert_eq!(gauge(1, 80, 8), "#-------");
        assert_eq!(gauge(100, 80, 8), "########");
    }
}
