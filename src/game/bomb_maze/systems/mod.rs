mod ai;
mod bomb;
mod combat;
mod effects;
mod exit;
mod hud;
mod input;

pub use ai::bm_enemy_ai;
pub use bomb::{bm_bomb_tick, bm_flame_tick};
pub use combat::{
    bm_enemy_touch, bm_flame_burn_powerups, bm_flame_damage, bm_powerup_pickup,
};
pub use effects::{bm_actor_visual_update, bm_bomb_visual_update, bm_item_visual_update};
pub use exit::{bm_exit_and_respawn, bm_player_blink};
pub use hud::bm_hud_update;
pub use input::{bm_player_input, bm_sample_input};
