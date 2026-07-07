use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::super::components::*;
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
            format!("BOSS: {} / {}", boss_hp, state.boss_hp_max.max(1))
        } else {
            String::new()
        };
        set_text(
            &mut t,
            &format!(
                "分数 {}\n纪录 {}\n生命 {}\n火力 LV.{}\n第 {} 关\n{}",
                session.score,
                high,
                session.lives.max(0),
                state.power,
                session.level,
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
