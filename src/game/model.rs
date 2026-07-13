use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::common::constants::SAVE_FILE;
use crate::common::settings::UserSettings;

const SAVE_MAGIC: [u8; 4] = *b"BAOG";
const SAVE_VERSION: u16 = 2;

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
    MemoryMatch,
    Sokoban,
}

impl GameKind {
    pub(super) const ALL: [GameKind; 8] = [
        GameKind::Tank,
        GameKind::BombMaze,
        GameKind::SpaceShooter,
        GameKind::SuperMario,
        GameKind::Contra,
        GameKind::BubbleBobble,
        GameKind::MemoryMatch,
        GameKind::Sokoban,
    ];

    pub(super) fn index(self) -> usize {
        match self {
            GameKind::Tank => 0,
            GameKind::BombMaze => 1,
            GameKind::SpaceShooter => 2,
            GameKind::SuperMario => 3,
            GameKind::Contra => 4,
            GameKind::BubbleBobble => 5,
            GameKind::MemoryMatch => 6,
            GameKind::Sokoban => 7,
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
            GameKind::MemoryMatch => "7 记忆翻翻乐",
            GameKind::Sokoban => "8 推箱子",
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
            GameKind::MemoryMatch => "翻开同样字符的两张牌，限时配对全部！",
            GameKind::Sokoban => "把所有箱子推到目标点，从易到难十连关！",
        }
    }

    pub(super) const fn max_level(self) -> u8 {
        match self {
            GameKind::SuperMario => 4,
            _ => 10,
        }
    }

}

#[derive(Resource)]
pub(super) struct SelectedGame(pub(super) GameKind);

#[derive(Resource, Clone, Serialize, Deserialize)]
pub(super) struct SaveData {
    pub(super) high_scores: [u32; 8],
    pub(super) unlocked_levels: [u8; 8],
    pub(super) selected_levels: [u8; 8],
    pub(super) settings: UserSettings,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            high_scores: [0; 8],
            unlocked_levels: [1; 8],
            selected_levels: [1; 8],
            settings: UserSettings::default(),
        }
    }
}

/// 1.x 版本直接序列化的存档形状，字段顺序不可修改。
#[derive(Serialize, Deserialize)]
struct LegacySaveData {
    high_scores: [u32; 8],
    unlocked_levels: [u8; 8],
    volume: f32,
}

#[derive(Serialize, Deserialize)]
struct SaveEnvelope {
    magic: [u8; 4],
    version: u16,
    data: SaveData,
}

/// 存档文件的绝对路径。
///
/// macOS 下放到 `~/Library/Application Support/BaoGames/`：从 Finder 启动 `.app` 时
/// 工作目录是 `/`，用相对路径写入会静默失败、存档丢失。其它平台保持运行目录下的
/// 相对路径，开发期 `cargo run` 行为不变。
fn save_path() -> std::path::PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        let dir = std::path::Path::new(&home).join("Library/Application Support/BaoGames");
        let _ = fs::create_dir_all(&dir);
        return dir.join(SAVE_FILE);
    }
    std::path::PathBuf::from(SAVE_FILE)
}

impl SaveData {
    pub(super) fn load() -> Self {
        let Ok(bytes) = fs::read(save_path()) else {
            return Self::default();
        };
        Self::decode(&bytes).unwrap_or_default()
    }

    fn decode(bytes: &[u8]) -> Option<Self> {
        if let Ok(envelope) = bincode::deserialize::<SaveEnvelope>(bytes)
            && envelope.magic == SAVE_MAGIC
            && envelope.version == SAVE_VERSION
        {
            let mut data = envelope.data;
            data.sanitize();
            return Some(data);
        }
        if let Ok(legacy) = bincode::deserialize::<LegacySaveData>(bytes) {
            let volume = legacy.volume.clamp(0.0, 1.0);
            let mut data = Self {
                high_scores: legacy.high_scores,
                unlocked_levels: legacy.unlocked_levels,
                selected_levels: legacy.unlocked_levels,
                settings: UserSettings {
                    music_volume: volume,
                    sfx_volume: volume,
                    ..default()
                },
            };
            data.sanitize();
            return Some(data);
        }
        None
    }

    pub(super) fn store(&self) {
        let envelope = SaveEnvelope {
            magic: SAVE_MAGIC,
            version: SAVE_VERSION,
            data: self.clone(),
        };
        if let Ok(bytes) = bincode::serialize(&envelope) {
            let path = save_path();
            let temporary = path.with_extension("tmp");
            if fs::write(&temporary, &bytes).is_ok() && fs::rename(&temporary, &path).is_ok() {
                return;
            }
            let _ = fs::write(path, bytes);
            let _ = fs::remove_file(temporary);
        }
    }

    fn sanitize(&mut self) {
        self.settings.sanitize();
        for (index, kind) in GameKind::ALL.iter().copied().enumerate() {
            let max = kind.max_level();
            self.unlocked_levels[index] = self.unlocked_levels[index].clamp(1, max);
            self.selected_levels[index] = self.selected_levels[index]
                .clamp(1, self.unlocked_levels[index]);
        }
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;
    use crate::common::settings::{DisplayMode, GameplayProfile};

    #[test]
    fn legacy_save_migrates_progress_and_volume() {
        let legacy = LegacySaveData {
            high_scores: [42; 8],
            unlocked_levels: [3; 8],
            volume: 0.4,
        };
        let bytes = bincode::serialize(&legacy).expect("legacy test data should serialize");
        let migrated = SaveData::decode(&bytes).expect("legacy save should migrate");
        assert_eq!(migrated.high_scores, [42; 8]);
        assert_eq!(migrated.unlocked_levels[GameKind::SuperMario.index()], 3);
        assert_eq!(migrated.selected_levels[0], 3);
        assert_eq!(migrated.settings.music_volume, 0.4);
        assert_eq!(migrated.settings.sfx_volume, 0.4);
    }

    #[test]
    fn current_envelope_round_trips_settings() {
        let mut data = SaveData::default();
        data.settings.display_mode = DisplayMode::Widescreen16x9;
        data.settings.gameplay_profile = GameplayProfile::Assist;
        let envelope = SaveEnvelope {
            magic: SAVE_MAGIC,
            version: SAVE_VERSION,
            data,
        };
        let bytes = bincode::serialize(&envelope).expect("save envelope should serialize");
        let decoded = SaveData::decode(&bytes).expect("current save should decode");
        assert_eq!(decoded.settings.display_mode, DisplayMode::Widescreen16x9);
        assert_eq!(decoded.settings.gameplay_profile, GameplayProfile::Assist);
    }

    #[test]
    fn migration_clamps_mario_to_four_levels() {
        let legacy = LegacySaveData {
            high_scores: [0; 8],
            unlocked_levels: [10; 8],
            volume: 1.5,
        };
        let bytes = bincode::serialize(&legacy).expect("legacy test data should serialize");
        let migrated = SaveData::decode(&bytes).expect("legacy save should migrate");
        assert_eq!(migrated.unlocked_levels[GameKind::SuperMario.index()], 4);
        assert_eq!(migrated.settings.music_volume, 1.0);
    }

    #[test]
    fn corrupt_save_is_rejected() {
        assert!(SaveData::decode(b"not a baogames save").is_none());
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
