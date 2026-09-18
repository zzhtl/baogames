use bevy::prelude::*;

use crate::game::puzzle::grid::GridLayout;

use super::logic::{is_solved, manhattan_lower_bound};

#[derive(Resource)]
pub struct SlidingStage {
    pub size: usize,
    pub layout: GridLayout,
    /// 行优先盘面，0 是空格。
    pub board: Vec<u8>,
    pub initial_board: Vec<u8>,
    pub moves: u32,
    /// 打乱后盘面的曼哈顿下界，用来算效率分。
    pub lower_bound: u32,
    /// 撤销栈存的是"上一步块的移动方向"，回放时反着滑一次即可。
    pub history: Vec<(i32, i32)>,
    pub time_left: f32,
    pub message: String,
    pub message_clock: f32,
}

impl SlidingStage {
    pub fn is_cleared(&self) -> bool {
        is_solved(&self.board)
    }

    /// 编号为 `value` 的块当前在哪一格。
    pub fn cell_of(&self, value: u8) -> Option<(i32, i32)> {
        let index = self.board.iter().position(|&v| v == value)?;
        Some(((index % self.size) as i32, (index / self.size) as i32))
    }

    /// 已经回到自己位置的块数。
    pub fn settled(&self) -> usize {
        self.board
            .iter()
            .enumerate()
            .filter(|&(index, &value)| value != 0 && value as usize == index + 1)
            .count()
    }

    pub fn tiles_total(&self) -> usize {
        self.size * self.size - 1
    }

    pub fn recompute_lower_bound(&mut self) {
        self.lower_bound = manhattan_lower_bound(&self.board, self.size);
    }
}
