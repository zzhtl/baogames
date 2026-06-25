//! 太空射击：模块入口。

mod components;
mod constants;
mod geometry;
pub mod palette;
mod resources;
mod setup;
pub mod sprites;
mod systems;
mod waves;

pub(super) use resources::SpaceState;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    space_bullets_update, space_collisions, space_despawn_offscreen, space_enemy_ai,
    space_hud_update, space_lifetimes, space_player_blink, space_player_input,
    space_powerup_drop_and_pickup, space_spawner, space_stars_scroll,
};
