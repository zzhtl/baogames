//! 炸弹迷宫：模块入口。

mod components;
mod constants;
mod geometry;
pub mod palette;
mod resources;
mod setup;
pub mod sprites;
mod systems;

pub(super) use resources::BMStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    bm_actor_visual_update, bm_bomb_tick, bm_bomb_visual_update, bm_enemy_ai, bm_enemy_touch,
    bm_exit_and_respawn, bm_flame_burn_powerups, bm_flame_damage, bm_flame_tick, bm_hud_update,
    bm_item_visual_update, bm_player_blink, bm_player_input, bm_powerup_pickup, bm_sample_input,
};
