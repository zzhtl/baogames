use bevy::prelude::*;

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
        **t = format!(
            "分数 {}\n纪录 {}\n生命 {}\n火力 LV.{}\n第 {} 关\n{}",
            session.score,
            high,
            session.lives.max(0),
            state.power,
            session.level,
            boss_line
        );
    }
    if let Ok(mut t) = message.single_mut() {
        **t = if session.finished {
            if session.won {
                "通关！Enter 重玩，Esc 返回".to_string()
            } else {
                "再试一次！Enter 重玩，Esc 返回".to_string()
            }
        } else if session.paused {
            "已暂停：Esc 继续，Backspace 返回".to_string()
        } else if state.message_clock > 0.0 {
            state.message.clone()
        } else {
            String::new()
        };
    }
}
