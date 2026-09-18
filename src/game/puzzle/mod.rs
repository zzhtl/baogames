//! 益智游戏共用的网格 / 游标 / 输入基元。
//!
//! 新增的八款益智小游戏全是「网格 + 游标 + 确认键」，这里放它们共用的部分：
//! 棋盘自适应布局、格子游标的长按节奏、语义输入的固定步缓冲。
//!
//! 记忆翻翻乐与推箱子早于本模块，各自留了一份等价实现。它们能跑，不动。

pub mod controls;
pub mod grid;

pub(super) use controls::{PuzzleControls, puzzle_sample_input};

#[cfg(test)]
mod tests;
