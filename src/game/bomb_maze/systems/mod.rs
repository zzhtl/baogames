mod ai;
mod bomb;
mod combat;
mod exit;
mod hud;
mod input;

pub use ai::bm_enemy_ai;
pub use bomb::{bm_bomb_tick, bm_flame_tick};
pub use combat::{bm_enemy_touch, bm_flame_damage, bm_powerup_pickup};
pub use exit::{bm_exit_and_respawn, bm_player_blink};
pub use hud::bm_hud_update;
pub use input::bm_player_input;
