//! 迷宫探险：程序化生成的完美迷宫。
//!
//! 玩法：方向键走（可以按住连走），从左上角走到右下角的出口。
//! 第 4 关起出口上了锁，要先捡到钥匙；第 7 关起只看得见身边一小圈（雾视野）。
//! 5-7 岁档迷宫封顶 15×11，并且永远不开雾。
//!
//! 迷宫用随机深度优先挖通道生成，保证是一棵树：任意两点之间有且只有一条路，
//! 不会出现"看着能过去其实是死路"的环。

mod components;
mod constants;
pub mod generator;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::MazeStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{maze_check_finish, maze_fog_sync, maze_hud_update, maze_input, maze_render_sync};

#[cfg(test)]
mod tests;
