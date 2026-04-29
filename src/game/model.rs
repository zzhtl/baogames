use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::common::constants::SAVE_FILE;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(super) enum AppState {
    #[default]
    Menu,
    Playing,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(super) enum GameKind {
    Tank,
    BombMaze,
    BubbleShooter,
    PlaneShooter,
    RunGun,
    Platformer,
}

impl GameKind {
    pub(super) const ALL: [GameKind; 6] = [
        GameKind::Tank,
        GameKind::BombMaze,
        GameKind::BubbleShooter,
        GameKind::PlaneShooter,
        GameKind::RunGun,
        GameKind::Platformer,
    ];

    pub(super) fn index(self) -> usize {
        match self {
            GameKind::Tank => 0,
            GameKind::BombMaze => 1,
            GameKind::BubbleShooter => 2,
            GameKind::PlaneShooter => 3,
            GameKind::RunGun => 4,
            GameKind::Platformer => 5,
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            GameKind::Tank => "1 经典坦克",
            GameKind::BombMaze => "2 炸弹迷宫",
            GameKind::BubbleShooter => "3 泡泡龙风格",
            GameKind::PlaneShooter => "4 竖版飞行",
            GameKind::RunGun => "5 横版射击",
            GameKind::Platformer => "6 平台跳跃",
        }
    }

    pub(super) fn goal_text(self) -> &'static str {
        match self {
            GameKind::Tank => "保护基地，击败所有小坦克",
            GameKind::BombMaze => "炸开软砖，清理迷宫里的小机器人",
            GameKind::BubbleShooter => "发射同色泡泡，清空顶部泡泡",
            GameKind::PlaneShooter => "躲开云朵飞机，完成这一波飞行",
            GameKind::RunGun => "跳过平台，打掉挡路机器人",
            GameKind::Platformer => "收集星星，到达彩旗",
        }
    }
}

#[derive(Resource)]
pub(super) struct SelectedGame(pub(super) GameKind);

#[derive(Resource, Serialize, Deserialize)]
pub(super) struct SaveData {
    pub(super) high_scores: [u32; 6],
    pub(super) unlocked_levels: [u8; 6],
    volume: f32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            high_scores: [0; 6],
            unlocked_levels: [1; 6],
            volume: 0.7,
        }
    }
}

impl SaveData {
    pub(super) fn load() -> Self {
        match fs::read(SAVE_FILE) {
            Ok(bytes) => bincode::deserialize(&bytes).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub(super) fn store(&self) {
        if let Ok(bytes) = bincode::serialize(self) {
            let _ = fs::write(SAVE_FILE, bytes);
        }
    }
}

#[derive(Resource)]
pub(super) struct GameSession {
    pub(super) kind: GameKind,
    pub(super) level: u8,
    pub(super) score: u32,
    pub(super) lives: i32,
    pub(super) paused: bool,
    pub(super) finished: bool,
    pub(super) won: bool,
    pub(super) time_left: f32,
    pub(super) spawn_clock: f32,
    pub(super) player_cooldown: [f32; 2],
    pub(super) status: String,
}

#[derive(Component, Clone, Copy)]
pub(super) struct MenuEntity;

#[derive(Component, Clone, Copy)]
pub(super) struct GameEntity;

#[derive(Component)]
pub(super) struct HudText;

#[derive(Component)]
pub(super) struct Player {
    pub(super) id: usize,
}

#[derive(Component)]
pub(super) struct Enemy {
    pub(super) hp: i32,
    pub(super) points: u32,
}

#[derive(Component)]
pub(super) struct Projectile {
    pub(super) owner: ProjectileOwner,
    pub(super) radius: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProjectileOwner {
    Player,
}

#[derive(Component, Deref, DerefMut)]
pub(super) struct Velocity(pub(super) Vec2);

#[derive(Component)]
pub(super) struct Collider {
    pub(super) size: Vec2,
}

#[derive(Component)]
pub(super) struct SoftBlock;

#[derive(Component)]
pub(super) struct Wall;

#[derive(Component)]
pub(super) struct Goal;

#[derive(Component)]
pub(super) struct Collectible;

#[derive(Component)]
pub(super) struct Bubble {
    pub(super) color_id: usize,
}

#[derive(Component)]
pub(super) struct Bomb {
    pub(super) timer: Timer,
}

#[derive(Component)]
pub(super) struct Explosion {}

#[derive(Component)]
pub(super) struct Lifetime(pub(super) Timer);

#[derive(Component)]
pub(super) struct Grounded(pub(super) bool);
