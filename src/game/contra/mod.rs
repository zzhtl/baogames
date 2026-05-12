mod components;
mod constants;
mod geometry;
mod palette;
mod resources;
mod setup;
mod setup_actors;
mod setup_terrain;
mod systems;

pub(super) use resources::ContraStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    contra_boss_update, contra_bullets_update, contra_camera_follow, contra_enemy_ai,
    contra_explosion_update, contra_falcon_update, contra_hud_update, contra_offscreen_cleanup,
    contra_pickup_update, contra_player_blink, contra_player_input, contra_player_respawn,
    contra_player_vs_enemy, contra_physics, contra_spawner,
};
