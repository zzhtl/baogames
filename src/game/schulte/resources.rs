use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::puzzle::grid::{GridCursor, GridLayout};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SchulteMode {
    /// 1 → N²
    Ascending,
    /// N² → 1
    Descending,
    /// 红 1、蓝 1、红 2、蓝 2…
    DualColor,
}

impl SchulteMode {
    pub const fn hint(self) -> &'static str {
        match self {
            SchulteMode::Ascending => "从小到大点",
            SchulteMode::Descending => "倒序，从大往小",
            SchulteMode::DualColor => "红蓝交替点",
        }
    }
}

#[derive(Resource)]
pub struct SchulteStage {
    pub layout: GridLayout,
    pub mode: SchulteMode,
    /// 行优先的题面：每格是 (数值, 颜色编号)。单色模式颜色编号恒为 0。
    pub cells: Vec<(u32, u8)>,
    /// 目标出现的先后顺序，`progress` 是它的下标。
    pub order: Vec<(u32, u8)>,
    pub found: Vec<bool>,
    pub progress: usize,
    pub cursor: GridCursor,
    pub time_left: f32,
    pub initial_time: f32,
    pub streak: u32,
    pub best_streak: u32,
    pub mistakes: u32,
    /// 点错时高亮的格子与剩余闪烁时间。
    pub wrong_cell: Option<usize>,
    pub wrong_clock: f32,
    pub message: String,
    pub message_clock: f32,
}

impl SchulteStage {
    pub fn target(&self) -> Option<(u32, u8)> {
        self.order.get(self.progress).copied()
    }

    pub fn is_cleared(&self) -> bool {
        self.progress >= self.order.len()
    }

    pub fn cell_at(&self, col: i32, row: i32) -> Option<(usize, (u32, u8))> {
        let index = self.layout.index(col, row)?;
        self.cells.get(index).map(|&cell| (index, cell))
    }
}

/// 生成题面与目标顺序。
///
/// 返回 `(cells, order)`：`cells` 是打乱后的格子内容，`order` 是必须依次点中的
/// 目标序列。抽成纯函数便于单测（每个数恰好出现一次、顺序与模式一致）。
pub fn build_board(size: i32, mode: SchulteMode) -> (Vec<(u32, u8)>, Vec<(u32, u8)>) {
    let total = (size * size) as u32;
    let mut cells: Vec<(u32, u8)> = match mode {
        SchulteMode::Ascending | SchulteMode::Descending => {
            (1..=total).map(|value| (value, 0u8)).collect()
        }
        SchulteMode::DualColor => {
            // 奇数格时红色多拿一个，交替序列才能以红色收尾。
            let warm = total.div_ceil(2);
            let cool = total / 2;
            (1..=warm)
                .map(|value| (value, 0u8))
                .chain((1..=cool).map(|value| (value, 1u8)))
                .collect()
        }
    };
    let order: Vec<(u32, u8)> = match mode {
        SchulteMode::Ascending => cells.clone(),
        SchulteMode::Descending => {
            let mut reversed = cells.clone();
            reversed.reverse();
            reversed
        }
        SchulteMode::DualColor => {
            let warm = total.div_ceil(2);
            let cool = total / 2;
            (1..=warm)
                .flat_map(|value| {
                    let second = (value <= cool).then_some((value, 1u8));
                    std::iter::once((value, 0u8)).chain(second)
                })
                .collect()
        }
    };
    cells.shuffle(&mut thread_rng());
    (cells, order)
}
