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
    SpaceShooter,
    SuperMario,
    Contra,
    BubbleBobble,
}

impl GameKind {
    pub(super) const ALL: [GameKind; 6] = [
        GameKind::Tank,
        GameKind::BombMaze,
        GameKind::SpaceShooter,
        GameKind::SuperMario,
        GameKind::Contra,
        GameKind::BubbleBobble,
    ];

    pub(super) fn index(self) -> usize {
        match self {
            GameKind::Tank => 0,
            GameKind::BombMaze => 1,
            GameKind::SpaceShooter => 2,
            GameKind::SuperMario => 3,
            GameKind::Contra => 4,
            GameKind::BubbleBobble => 5,
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            GameKind::Tank => "1 坦克大战",
            GameKind::BombMaze => "2 炸弹迷宫",
            GameKind::SpaceShooter => "3 太空射击",
            GameKind::SuperMario => "4 超级玛丽",
            GameKind::Contra => "5 魂斗罗",
            GameKind::BubbleBobble => "6 泡泡龙",
        }
    }

    pub(super) fn goal_text(self) -> &'static str {
        match self {
            GameKind::Tank => "保护基地，击败所有小坦克",
            GameKind::BombMaze => "炸开软砖，清理迷宫里的小机器人",
            GameKind::SpaceShooter => "驾驶战机击落敌机，挑战关底 BOSS",
            GameKind::SuperMario => "踩 Goomba、吃蘑菇、冲到旗杆下！",
            GameKind::Contra => "8 方向射击，吃道具，击破要塞 BOSS",
            GameKind::BubbleBobble => "瞄准三连同色，清空所有泡泡！",
        }
    }

    pub(super) fn implemented(self) -> bool {
        matches!(
            self,
            GameKind::Tank
                | GameKind::BombMaze
                | GameKind::SpaceShooter
                | GameKind::SuperMario
                | GameKind::Contra
                | GameKind::BubbleBobble
        )
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
    pub(super) status: String,
}

#[derive(Component, Clone, Copy)]
pub(super) struct MenuEntity;

#[derive(Component, Clone, Copy)]
pub(super) struct GameEntity;

#[derive(Component, Deref, DerefMut)]
pub(super) struct Velocity(pub(super) Vec2);

#[derive(Component)]
pub(super) struct Collider {
    pub(super) size: Vec2,
}

#[derive(Component)]
pub(super) struct Lifetime(pub(super) Timer);
