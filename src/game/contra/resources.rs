use bevy::prelude::*;

use super::components::{EnemyKind, Weapon};

pub struct EnemySpawnMark {
    pub trigger_x: f32,
    pub pos: Vec2,
    pub kind: EnemyKind,
    pub facing: f32,
}

pub struct FalconMark {
    pub trigger_x: f32,
    pub start: Vec2,
    pub vx: f32,
    pub weapon: Weapon,
}

#[derive(Resource)]
pub struct ContraStage {
    pub level: u8,
    pub world_w: f32,
    pub player_spawn: Vec2,
    pub spawn_marks: Vec<EnemySpawnMark>,
    pub spawn_idx: usize,
    pub falcon_marks: Vec<FalconMark>,
    pub falcon_idx: usize,
    pub boss_x: f32,
    pub boss_spawned: bool,
    pub boss_dead: bool,
    pub top_score: u32,
}
