//! 推箱子：经典格子推箱益智小游戏。
//!
//! 玩法：方向键 / WASD 移动；走到箱子前会自动推动，箱子前是墙或另一只箱子时则推不动。
//! 把所有箱子推到目标点（带橙色圆心的格子）即可通关。R 重来本关。

mod components;
mod constants;
pub mod palette;
mod resources;
mod setup;
pub mod sprites;
mod systems;

pub(super) use resources::SokobanStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    sokoban_box_visual_sync, sokoban_check_finish, sokoban_hud_update, sokoban_input,
    sokoban_render_sync,
};

#[cfg(test)]
mod tests;
