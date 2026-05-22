//! 记忆翻翻乐：训练专注力和短时记忆。
//!
//! 玩法：方向键移动光标，Enter / Space 翻牌；两张同样字符的牌配对成功后保留亮色。
//! 关卡棋盘逐渐变大、限时逐渐变紧。

mod components;
mod constants;
mod resources;
mod setup;
mod systems;

pub(super) use resources::MemoryStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    memory_check_finish, memory_cursor_follow, memory_hud_update, memory_input,
    memory_render_sync,
};

#[cfg(test)]
mod tests;
