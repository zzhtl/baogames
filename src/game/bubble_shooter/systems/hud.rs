use bevy::prelude::*;

use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::super::components::{BubbleCell, BubbleHud, BubbleMessage, GridBubble};
use super::super::constants::ROWS_MAX;
use super::super::grid::cols_in_row;
use super::super::resources::BubbleStage;

pub fn bubble_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<BubbleStage>,
    grid_q: Query<&BubbleCell, With<GridBubble>>,
    mut hud: Query<&mut Text2d, (With<BubbleHud>, Without<BubbleMessage>)>,
    mut msg: Query<&mut Text2d, (With<BubbleMessage>, Without<BubbleHud>)>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let high = save.high_scores[GameKind::BubbleBobble.index()].max(session.score);
        // 数泡泡：用 grid 数据更准确
        let mut left = 0u32;
        for r in 0..ROWS_MAX {
            for c in 0..cols_in_row(r as i32) {
                if stage.grid[r][c as usize].is_some() {
                    left += 1;
                }
            }
        }
        let _ = grid_q;
        set_text(
            &mut t,
            &format!(
                "分数 {}\n纪录 {}\n第 {} 关\n剩余 {}\n连消 x{}\n下移 {} 步",
                session.score,
                high,
                session.level,
                left,
                stage.combo_streak,
                stage.shots_left_for_descend.max(0),
            ),
        );
    }
    // 暂停/结束由统一覆盖层显示，这里只放玩法瞬时消息
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
