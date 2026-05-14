use bevy::prelude::*;

#[derive(Resource)]
pub struct TankStage {
    pub remaining_to_spawn: u8,
    pub spawn_timer: f32,
    pub spawn_idx: usize,
    #[allow(dead_code)]
    pub stage_num: u8,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub base_alive: bool,
    pub kills: u8,
    pub two_player: bool,
    pub mode_selected: bool,
}
