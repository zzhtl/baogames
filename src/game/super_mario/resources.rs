use bevy::prelude::*;

#[derive(Resource)]
pub struct MarioStage {
    pub time_left: f32,
    pub coins: u32,
    pub level: u8,
    pub finish_timer: f32,
    pub player_spawn: Vec2,
}
