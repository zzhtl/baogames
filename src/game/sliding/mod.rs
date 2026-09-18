//! 数字华容道：经典滑块复原。
//!
//! 玩法：方向键 = 想让哪个方向的块滑过来。按「右」时，空格左边那块往右滑（空格自己往左走）。
//! 动作二撤销一步，重置键重排本关。把 1..N²−1 按顺序排好、空格回到右下角即通关。
//! 关卡 1-10 从 3×3 递进到 5×5；5-7 岁档封顶 4×4，打乱步数也减半。
//!
//! 打乱一定是从**已完成状态**做合法随机走，所以每一局都保证有解 ——
//! 直接随机排列有一半的概率是死局。

mod components;
mod constants;
pub mod logic;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::SlidingStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{sliding_check_finish, sliding_hud_update, sliding_input, sliding_render_sync};

#[cfg(test)]
mod tests;
