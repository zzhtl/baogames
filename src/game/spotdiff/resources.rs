use bevy::prelude::*;

use crate::game::puzzle::grid::{GridCursor, GridLayout};

#[derive(Resource)]
pub struct SpotDiffStage {
    /// 左右两张图各自的网格布局。
    pub layouts: [GridLayout; 2],
    pub cols: i32,
    pub rows: i32,
    /// 真正不一样的格子下标，升序。
    pub diffs: Vec<usize>,
    /// 每个不同点是否已经被找出来（与 `diffs` 同序）。
    pub found: Vec<bool>,
    pub cursor: GridCursor,
    pub mistakes: u32,
    pub time_left: f32,
    pub message: String,
    pub message_clock: f32,
}

impl SpotDiffStage {
    pub fn cursor_index(&self) -> usize {
        (self.cursor.row * self.cols + self.cursor.col) as usize
    }

    /// 这一格是不是不同点；是的话给出它在 `diffs` 里的位置。
    pub fn diff_slot(&self, index: usize) -> Option<usize> {
        self.diffs.iter().position(|&i| i == index)
    }

    pub fn is_found(&self, index: usize) -> bool {
        self.diff_slot(index).is_some_and(|slot| self.found[slot])
    }

    pub fn found_count(&self) -> usize {
        self.found.iter().filter(|hit| **hit).count()
    }

    pub fn is_cleared(&self) -> bool {
        !self.found.is_empty() && self.found.iter().all(|hit| *hit)
    }
}
