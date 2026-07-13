pub mod block;
pub mod bowser;
pub mod camera;
pub mod coin;
pub mod effects;
pub mod fireball;
pub mod flag;
pub mod goomba;
pub mod hud;
pub mod input;
pub mod koopa;
pub mod physics;
pub mod platform;
pub mod powerup;
pub mod respawn;

pub use block::mario_block_anim;
pub use bowser::{
    mario_axe_check, mario_bowser_ai, mario_bowser_cleanup, mario_bowser_fireball_update,
    mario_player_fire_vs_bowser,
};
pub use camera::mario_camera_follow;
pub use coin::mario_coin_popup;
pub use effects::{mario_player_animation, mario_player_blink, mario_shard_update};
pub use fireball::mario_fireball_update;
pub use flag::{mario_finish_seq, mario_flag_anim, mario_flag_check};
pub use goomba::{mario_goomba_ai, mario_player_vs_goomba};
pub use hud::{mario_hud_update, mario_time_check};
pub use input::{mario_player_input, mario_sample_input};
pub use koopa::{mario_koopa_ai, mario_player_vs_koopa, mario_shell_kills};
pub use physics::mario_physics;
pub use platform::{mario_lava_check, mario_platform_update};
pub use powerup::{mario_player_vs_powerup, mario_powerup_update};
pub use respawn::{mario_checkpoint_update, mario_respawn};
