//! 儿童数独：从 4×4 起步的数独。
//!
//! 玩法：方向键移动光标，动作一把格子里的数 +1、动作二 -1（都循环，经过 0 就是清空），
//! 重置键重来本关。行、列、宫里出现重复会立刻标红。
//! 关卡 1-3 是 4×4，4-6 是 6×6，7-10 是 9×9；5-7 岁档封顶 6×6，靠多挖空加难度。
//!
//! 题目是每局现生成的：回溯法先解出一张完整解，再逐格挖空并校验**唯一解**。

mod components;
mod constants;
pub mod generator;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::SudokuStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{sudoku_check_finish, sudoku_cursor_follow, sudoku_hud_update, sudoku_input, sudoku_render_sync};

#[cfg(test)]
mod tests;
