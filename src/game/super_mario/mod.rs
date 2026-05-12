mod components;
mod constants;
mod geometry;
mod levels;
mod palette;
mod resources;
mod setup;
mod setup_actors;
mod setup_terrain;
mod systems;

pub(super) use resources::MarioStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    mario_axe_check, mario_block_anim, mario_bowser_ai, mario_bowser_cleanup,
    mario_bowser_fireball_update, mario_brick_break, mario_camera_follow, mario_coin_popup,
    mario_finish_seq, mario_fireball_update, mario_flag_anim, mario_flag_check, mario_goomba_ai,
    mario_hud_update, mario_koopa_ai, mario_lava_check, mario_physics, mario_platform_update,
    mario_player_blink, mario_player_fire_vs_bowser, mario_player_input, mario_player_vs_goomba,
    mario_player_vs_koopa, mario_player_vs_powerup, mario_powerup_update, mario_respawn,
    mario_shard_update, mario_shell_kills, mario_time_check,
};
