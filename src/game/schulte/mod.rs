//! 舒尔特方格：经典注意力广度训练。
//!
//! 玩法：方格里乱序填着 1..N²，用方向键移动光标、动作一按顺序点出来。点错扣时间。
//! 第 5 关起改成倒序（从大到小），第 8 关起改成红蓝双色交替（红 1、蓝 1、红 2…）。
//! 5-7 岁档固定为 4×4 以内的顺序模式，限时放宽一半。

mod components;
mod constants;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::SchulteStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{schulte_check_finish, schulte_cursor_follow, schulte_hud_update, schulte_input, schulte_render_sync};

#[cfg(test)]
mod tests;
