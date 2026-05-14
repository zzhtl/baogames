//! 泡泡龙：模块入口。

mod components;
mod constants;
mod grid;
mod resources;
mod setup;
mod systems;

pub(super) use resources::{BubbleAssets, BubbleStage};
pub(super) use setup::setup_stage;
pub(super) use systems::{
    bubble_aim_dots_update, bubble_fall_anim, bubble_hud_update, bubble_next_preview_update,
    bubble_player_input, bubble_pop_anim, bubble_shot_update,
};
