use bevy::prelude::*;

#[derive(Resource)]
pub struct BubbleStage {
    pub grid: Vec<Vec<Option<u8>>>,
    pub descend: usize,
    pub aim: f32,
    pub current: u8,
    pub next: u8,
    pub shot_active: bool,
    pub shots_left_for_descend: i32,
    pub max_shots_per_descend: i32,
    #[allow(dead_code)]
    pub colors_count: u8,
    pub message: String,
    pub message_clock: f32,
    pub flash_clock: f32,
}
