use bevy::prelude::*;

#[derive(Resource)]
pub struct BMStage {
    pub level: u8,
    pub time_left: f32,
    pub p1_lives: i32,
    pub p2_lives: i32,
    pub p1_respawn: f32,
    pub p2_respawn: f32,
    pub all_enemies_dead_msg_shown: bool,
    pub status: String,
}
