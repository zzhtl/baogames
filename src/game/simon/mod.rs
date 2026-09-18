//! 记忆序列：四个音板依次亮起，照着按回去。
//!
//! 玩法：上下左右四个键就是四个音板，零学习成本。每答对一轮，序列加长一个。
//! 第 5 关起播放变快，第 8 关起要**倒着**按回去。5-7 岁档播放更慢、也不考倒序。
//!
//! 这一款用命数而不是倒计时：Simon 的压力来自序列长度，加个总倒计时只会打断节奏。
//! 命数沿用 `setup_game` 统一给的 3 条，每一步仍有输入限时，避免干等着不按。

mod components;
mod constants;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::SimonStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{simon_check_finish, simon_hud_update, simon_render_sync, simon_update};

#[cfg(test)]
mod tests;
