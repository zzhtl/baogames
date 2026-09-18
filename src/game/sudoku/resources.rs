use bevy::prelude::*;

use crate::game::puzzle::grid::{GridCursor, GridLayout};

use super::generator::{SudokuSpec, conflicts, is_solved};

#[derive(Resource)]
pub struct SudokuStage {
    pub spec: SudokuSpec,
    pub layout: GridLayout,
    /// 当前盘面，0 表示空。
    pub board: Vec<u8>,
    /// 题目给定的格子，不可修改。
    pub given: Vec<bool>,
    /// 每格是否正在冲突，每次改动后重算。
    pub conflicts: Vec<bool>,
    pub initial_board: Vec<u8>,
    pub cursor: GridCursor,
    pub time_left: f32,
    pub initial_time: f32,
    /// 累计制造过多少次冲突，结算时扣分。
    pub mistakes: u32,
    pub message: String,
    pub message_clock: f32,
}

impl SudokuStage {
    pub fn refresh_conflicts(&mut self) {
        self.conflicts = conflicts(&self.board, self.spec);
    }

    pub fn has_conflict(&self) -> bool {
        self.conflicts.iter().any(|hit| *hit)
    }

    pub fn filled(&self) -> usize {
        self.board.iter().filter(|&&value| value != 0).count()
    }

    pub fn is_cleared(&self) -> bool {
        is_solved(&self.board, self.spec)
    }

    pub fn cursor_index(&self) -> usize {
        self.spec.index(self.cursor.col as usize, self.cursor.row as usize)
    }
}
