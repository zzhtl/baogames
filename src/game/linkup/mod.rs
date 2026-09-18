//! 连连看：两张相同图案、连线拐弯不超过两次就能消掉。
//!
//! 玩法：方向键移动光标，动作一选中 / 配对，动作二取消选中，重置键重新洗牌。
//! 消完全部图案过关。第 6 关起消除后剩下的图案会往下掉。
//! 5-7 岁档封顶 6×4、图案更少，也不开下落。
//!
//! 连线可以绕到棋盘外面一圈 —— 这是连连看的标准规则，边角的图案才配得上对。
//! 走进死局（一对都连不上）时会自动洗牌，不会把小朋友卡在那儿。

mod components;
mod constants;
pub mod link;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::LinkupStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{linkup_check_finish, linkup_cursor_follow, linkup_hud_update, linkup_input, linkup_render_sync};

#[cfg(test)]
mod tests;
