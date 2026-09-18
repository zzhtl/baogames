//! 找不同：左右两张图，找出被改过的格子。
//!
//! 玩法：方向键移动光标（左右两张图同步移动），动作一标记你认为不一样的那一格。
//! 标错会扣时间。把所有不同点都找出来就通关。
//! 第 6 关起改动只发生在**明暗**上（形状和色相都不变），要盯得更细。
//! 5-7 岁档不同点更少，也永远只改形状或颜色这种大差别。
//!
//! 两张图都是现生成的：先随机铺一张底图，再复制一份改掉恰好 N 格。

mod components;
mod constants;
pub mod palette;
pub mod scene;
mod resources;
mod setup;
mod systems;

pub(super) use resources::SpotDiffStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{spotdiff_check_finish, spotdiff_cursor_follow, spotdiff_hud_update, spotdiff_input, spotdiff_render_sync};

#[cfg(test)]
mod tests;
