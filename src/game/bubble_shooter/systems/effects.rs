use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::{BubbleCeiling, BubbleDeadLine};
use super::super::constants::{BUBBLE_R, DEAD_Y, ROWS_MAX};
use super::super::grid::{cell_to_pos, cols_in_row};
use super::super::resources::BubbleStage;

pub fn bubble_field_feedback(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<BubbleStage>,
    mut ceiling: Query<&mut Sprite, (With<BubbleCeiling>, Without<BubbleDeadLine>)>,
    mut dead_line: Query<&mut Sprite, (With<BubbleDeadLine>, Without<BubbleCeiling>)>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    let pulse = time.elapsed_secs() * 12.0;
    for mut sprite in &mut ceiling {
        sprite.color = if stage.flash_clock > 0.0 && pulse.sin() > 0.0 {
            Color::srgb(1.0, 0.86, 0.94)
        } else {
            Color::srgb(0.86, 0.52, 0.72)
        };
    }

    let danger = danger_level(&stage);
    let alpha = 0.58 + danger * (0.26 + pulse.sin().max(0.0) * 0.16);
    for mut sprite in &mut dead_line {
        sprite.color = Color::srgba(0.96, 0.24 + (1.0 - danger) * 0.18, 0.36, alpha);
    }
}

fn danger_level(stage: &BubbleStage) -> f32 {
    let mut lowest = f32::INFINITY;
    for row in 0..ROWS_MAX {
        for col in 0..cols_in_row(row as i32) {
            if stage.grid[row][col as usize].is_some() {
                lowest = lowest.min(cell_to_pos(col, row as i32, stage.descend).y - BUBBLE_R);
            }
        }
    }
    if lowest == f32::INFINITY {
        0.0
    } else {
        (1.0 - (lowest - DEAD_Y) / 120.0).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::bubble_shooter::constants::COLS_EVEN;

    fn stage() -> BubbleStage {
        BubbleStage {
            grid: vec![vec![None; COLS_EVEN as usize]; ROWS_MAX],
            descend: 0,
            aim: 0.0,
            current: 0,
            next: 0,
            shot_active: false,
            shots_left_for_descend: 12,
            max_shots_per_descend: 12,
            colors_count: 3,
            message: String::new(),
            message_clock: 0.0,
            flash_clock: 0.0,
            recoil_clock: 0.0,
            combo_streak: 0,
        }
    }

    #[test]
    fn danger_grows_as_ceiling_descends() {
        let mut stage = stage();
        stage.grid[8][0] = Some(0);
        let safe = danger_level(&stage);
        stage.descend = 5;
        assert!(danger_level(&stage) > safe);
    }
}
