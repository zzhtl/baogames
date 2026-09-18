//! 色字大挑战：Stroop 抗干扰训练。
//!
//! 玩法：中间显示一个颜色词（比如「红」），但它本身是用另一种颜色画的。顶上给规则：
//! 「看颜色」要选笔画的颜色，「看字义」要选字读出来的颜色。左右移动选色块、动作一作答。
//! 第 4 关起规则每题随机变，第 7 关起连规则提示本身都带干扰色。
//! 5-7 岁档固定「看颜色」且只用红黄蓝绿四色 —— 不识字也能玩。

mod components;
mod constants;
pub mod palette;
mod resources;
mod setup;
mod systems;

pub(super) use resources::StroopStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{stroop_check_finish, stroop_hud_update, stroop_input, stroop_render_sync};

#[cfg(test)]
mod tests;
