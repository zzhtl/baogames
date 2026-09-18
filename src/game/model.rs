use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::common::constants::SAVE_FILE;
use crate::common::settings::UserSettings;

const SAVE_MAGIC: [u8; 4] = *b"BAOG";
const SAVE_VERSION: u16 = 3;

/// 存档里每个进度数组的槽位数：经典 8 款占 0..8，益智 8 款占 8..16。
///
/// 顺序是**存档兼容性契约**：前 8 槽永远是原来的经典八款，新游戏只能往后追加。
pub(super) const SAVE_SLOTS: usize = 16;

/// 1.x 裸结构 `[u32;8] + [u8;8] + f32` 的精确长度。
///
/// bincode 的自由函数允许尾部多余字节，不卡死长度的话，任何 ≥44 字节的缓冲
/// （包括 216 字节的 v2 信封）都会被解成一份垃圾数据。
const LEGACY_SAVE_LEN: usize = 44;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(super) enum AppState {
    #[default]
    Menu,
    Playing,
}

/// 卡带墙的分栏。经典是原来的动作 / 射击八款，益智是面向 5-12 岁的八款。
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub(super) enum GameCategory {
    #[default]
    Classic,
    Puzzle,
}

impl GameCategory {
    pub(super) const ALL: [GameCategory; 2] = [GameCategory::Classic, GameCategory::Puzzle];

    pub(super) const fn label(self) -> &'static str {
        match self {
            GameCategory::Classic => "经典",
            GameCategory::Puzzle => "益智",
        }
    }

    pub(super) const fn games(self) -> &'static [GameKind] {
        match self {
            GameCategory::Classic => GameKind::CLASSIC,
            GameCategory::Puzzle => GameKind::PUZZLE,
        }
    }

    pub(super) const fn toggled(self) -> Self {
        match self {
            GameCategory::Classic => GameCategory::Puzzle,
            GameCategory::Puzzle => GameCategory::Classic,
        }
    }
}

/// 益智游戏的年龄档。进游戏前在选关面板里切，决定起始网格、限时和干扰强度。
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(super) enum AgeTier {
    /// 5-7 岁：网格更小、限时更宽、不开高阶干扰规则。
    #[default]
    Junior,
    /// 8-12 岁：走完整难度曲线。
    Senior,
}

impl AgeTier {
    pub(super) const fn label(self) -> &'static str {
        match self {
            AgeTier::Junior => "5-7 岁",
            AgeTier::Senior => "8-12 岁",
        }
    }

    pub(super) const fn toggled(self) -> Self {
        match self {
            AgeTier::Junior => AgeTier::Senior,
            AgeTier::Senior => AgeTier::Junior,
        }
    }

    /// 低龄档把限时放宽的倍率。
    pub(super) const fn time_scale(self) -> f32 {
        match self {
            AgeTier::Junior => 1.5,
            AgeTier::Senior => 1.0,
        }
    }

    pub(super) const fn is_junior(self) -> bool {
        matches!(self, AgeTier::Junior)
    }
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
    Schulte,
    Stroop,
    Sudoku,
    Sliding,
    MazeRun,
    LinkUp,
    Simon,
    SpotDiff,
}

impl GameKind {
    /// 经典栏。顺序冻结：它同时是存档 `high_scores` 等数组的前 8 个下标。
    pub(super) const CLASSIC: &'static [GameKind] = &[
        GameKind::Tank,
        GameKind::BombMaze,
        GameKind::SpaceShooter,
        GameKind::SuperMario,
        GameKind::Contra,
        GameKind::BubbleBobble,
        GameKind::MemoryMatch,
        GameKind::Sokoban,
    ];

    /// 益智栏，占存档槽位 8..16。
    pub(super) const PUZZLE: &'static [GameKind] = &[
        GameKind::Schulte,
        GameKind::Stroop,
        GameKind::Sudoku,
        GameKind::Sliding,
        GameKind::MazeRun,
        GameKind::LinkUp,
        GameKind::Simon,
        GameKind::SpotDiff,
    ];

    /// 全部游戏，顺序必须是 `CLASSIC` 后接 `PUZZLE`（`index()` 与之对齐）。
    pub(super) const ALL: &'static [GameKind] = &[
        GameKind::Tank,
        GameKind::BombMaze,
        GameKind::SpaceShooter,
        GameKind::SuperMario,
        GameKind::Contra,
        GameKind::BubbleBobble,
        GameKind::MemoryMatch,
        GameKind::Sokoban,
        GameKind::Schulte,
        GameKind::Stroop,
        GameKind::Sudoku,
        GameKind::Sliding,
        GameKind::MazeRun,
        GameKind::LinkUp,
        GameKind::Simon,
        GameKind::SpotDiff,
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
            GameKind::Schulte => 8,
            GameKind::Stroop => 9,
            GameKind::Sudoku => 10,
            GameKind::Sliding => 11,
            GameKind::MazeRun => 12,
            GameKind::LinkUp => 13,
            GameKind::Simon => 14,
            GameKind::SpotDiff => 15,
        }
    }

    pub(super) fn category(self) -> GameCategory {
        if GameCategory::Puzzle.games().contains(&self) {
            GameCategory::Puzzle
        } else {
            GameCategory::Classic
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
            GameKind::Schulte => "9 舒尔特方格",
            GameKind::Stroop => "10 色字大挑战",
            GameKind::Sudoku => "11 儿童数独",
            GameKind::Sliding => "12 数字华容道",
            GameKind::MazeRun => "13 迷宫探险",
            GameKind::LinkUp => "14 连连看",
            GameKind::Simon => "15 记忆序列",
            GameKind::SpotDiff => "16 找不同",
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
            GameKind::Schulte => "按数字顺序点满整张方格，越快越好！",
            GameKind::Stroop => "看颜色还是看字义？别被字骗了！",
            GameKind::Sudoku => "每行每列每宫，数字都不能重复！",
            GameKind::Sliding => "滑动数字块，把它们排回顺序！",
            GameKind::MazeRun => "找到钥匙，走出迷宫的出口！",
            GameKind::LinkUp => "连起一样的图案，拐弯别超过两次！",
            GameKind::Simon => "看清灯光顺序，照着按回去！",
            GameKind::SpotDiff => "两张图哪里不一样？全找出来！",
        }
    }

    pub(super) const fn max_level(self) -> u8 {
        match self {
            GameKind::SuperMario => 4,
            _ => 10,
        }
    }

    /// 每个游戏的主色，全局唯一真源。
    ///
    /// 卡带墙的卡片描边、关卡背景边框、结算覆盖层描边共用它 —— 之前这三处
    /// 各写了一套互不相同的取色，同一个游戏在不同界面是不同颜色。
    pub(super) const fn accent(self) -> Color {
        match self {
            GameKind::Tank => Color::srgb(0.35, 0.78, 0.42),
            GameKind::BombMaze => Color::srgb(0.95, 0.58, 0.24),
            GameKind::SpaceShooter => Color::srgb(0.36, 0.72, 1.0),
            GameKind::SuperMario => Color::srgb(0.92, 0.42, 0.30),
            GameKind::Contra => Color::srgb(0.95, 0.36, 0.22),
            GameKind::BubbleBobble => Color::srgb(0.90, 0.40, 0.80),
            GameKind::MemoryMatch => Color::srgb(0.36, 0.86, 0.86),
            GameKind::Sokoban => Color::srgb(0.96, 0.78, 0.32),
            GameKind::Schulte => Color::srgb(0.36, 0.86, 0.70),
            GameKind::Stroop => Color::srgb(0.90, 0.48, 0.82),
            GameKind::Sudoku => Color::srgb(0.46, 0.55, 0.95),
            GameKind::Sliding => Color::srgb(0.82, 0.86, 0.34),
            GameKind::MazeRun => Color::srgb(0.45, 0.82, 0.38),
            GameKind::LinkUp => Color::srgb(0.36, 0.76, 0.96),
            GameKind::Simon => Color::srgb(0.98, 0.42, 0.66),
            GameKind::SpotDiff => Color::srgb(0.99, 0.56, 0.38),
        }
    }

    /// 卡带墙上的短名（去掉 `title()` 前面的编号）。
    pub(super) fn short_title(self) -> &'static str {
        let t = self.title();
        t.split_once(' ').map(|(_, name)| name).unwrap_or(t)
    }
}

#[derive(Resource)]
pub(super) struct SelectedGame(pub(super) GameKind);

#[derive(Resource, Clone, Serialize, Deserialize)]
pub(super) struct SaveData {
    pub(super) high_scores: [u32; SAVE_SLOTS],
    pub(super) unlocked_levels: [u8; SAVE_SLOTS],
    pub(super) selected_levels: [u8; SAVE_SLOTS],
    pub(super) settings: UserSettings,
    /// 益智游戏的年龄档。放在结构尾部，后续版本继续往尾部追加。
    pub(super) age_tier: AgeTier,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            high_scores: [0; SAVE_SLOTS],
            unlocked_levels: [1; SAVE_SLOTS],
            selected_levels: [1; SAVE_SLOTS],
            settings: UserSettings::default(),
            age_tier: AgeTier::default(),
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

/// v2（8 槽）存档的冻结快照，字段顺序与类型不可修改。
#[derive(Serialize, Deserialize)]
struct SaveDataV2 {
    high_scores: [u32; 8],
    unlocked_levels: [u8; 8],
    selected_levels: [u8; 8],
    settings: UserSettings,
}

#[derive(Serialize, Deserialize)]
struct SaveEnvelopeV2 {
    magic: [u8; 4],
    version: u16,
    data: SaveDataV2,
}

#[derive(Serialize, Deserialize)]
struct SaveEnvelope {
    magic: [u8; 4],
    version: u16,
    data: SaveData,
}

/// 只含定长头的视图。
///
/// bincode 允许尾部多余字节，所以任何 ≥6 字节的缓冲都能解出它 —— 这正是分流
/// 的关键：整条信封一旦因 `data` 段长度对不上而 `Err`，就再也读不到 `version`。
#[derive(Deserialize)]
struct SaveHeader {
    magic: [u8; 4],
    version: u16,
}

/// 存档文件的绝对路径。
///
/// macOS 下放到 `~/Library/Application Support/BaoGames/`：从 Finder 启动 `.app` 时
/// 工作目录是 `/`，用相对路径写入会静默失败、存档丢失。其它平台保持运行目录下的
/// 相对路径，开发期 `cargo run` 行为不变。
fn save_path() -> std::path::PathBuf {
    // 无头截图跑起来会读写存档，而 baogames.save 是入库文件——必须能重定向，
    // 否则每截一轮图工作区就脏一次。
    #[cfg(feature = "devtools")]
    if let Some(path) = std::env::var_os("BAOGAMES_SAVE_PATH") {
        return std::path::PathBuf::from(path);
    }
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

    /// 解码顺序：先读 6 字节头按 version 分流；**带 magic 的一律不回落到 1.x 分支**。
    ///
    /// 回落是会真出事的：v2 信封 216 字节，拿去解 44 字节的 `LegacySaveData` 会因
    /// 允许尾部字节而**成功**，解出的 `volume` 落在 v2 的 `unlocked_levels` 上，
    /// clamp 后约等于 0 —— 玩家的音乐音效会莫名其妙全部静音。
    fn decode(bytes: &[u8]) -> Option<Self> {
        if let Ok(header) = bincode::deserialize::<SaveHeader>(bytes)
            && header.magic == SAVE_MAGIC
        {
            let mut data = match header.version {
                SAVE_VERSION => bincode::deserialize::<SaveEnvelope>(bytes).ok()?.data,
                2 => Self::migrate_v2(bincode::deserialize::<SaveEnvelopeV2>(bytes).ok()?.data),
                // 未知版本（含从更新的构建降级回来）明确拒绝，宁可回默认档也不猜。
                _ => return None,
            };
            data.sanitize();
            return Some(data);
        }
        if bytes.len() == LEGACY_SAVE_LEN
            && let Ok(legacy) = bincode::deserialize::<LegacySaveData>(bytes)
        {
            let mut data = Self::migrate_legacy(legacy);
            data.sanitize();
            return Some(data);
        }
        None
    }

    fn migrate_v2(old: SaveDataV2) -> Self {
        // 从 default() 起步，益智那 8 个新槽才会拿到「解锁第 1 关」而不是 0。
        let mut data = Self::default();
        data.high_scores[..8].copy_from_slice(&old.high_scores);
        data.unlocked_levels[..8].copy_from_slice(&old.unlocked_levels);
        data.selected_levels[..8].copy_from_slice(&old.selected_levels);
        data.settings = old.settings;
        data
    }

    fn migrate_legacy(legacy: LegacySaveData) -> Self {
        let volume = legacy.volume.clamp(0.0, 1.0);
        let mut data = Self::default();
        data.high_scores[..8].copy_from_slice(&legacy.high_scores);
        data.unlocked_levels[..8].copy_from_slice(&legacy.unlocked_levels);
        data.selected_levels[..8].copy_from_slice(&legacy.unlocked_levels);
        data.settings.music_volume = volume;
        data.settings.sfx_volume = volume;
        data
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

    fn v2_bytes(data: SaveDataV2) -> Vec<u8> {
        bincode::serialize(&SaveEnvelopeV2 {
            magic: SAVE_MAGIC,
            version: 2,
            data,
        })
        .expect("v2 test data should serialize")
    }

    fn v2_sample() -> SaveDataV2 {
        SaveDataV2 {
            high_scores: [11, 22, 33, 44, 55, 66, 77, 88],
            unlocked_levels: [2; 8],
            selected_levels: [2; 8],
            settings: UserSettings {
                music_volume: 0.7,
                sfx_volume: 0.5,
                ..default()
            },
        }
    }

    #[test]
    fn legacy_save_migrates_progress_and_volume() {
        let legacy = LegacySaveData {
            high_scores: [42; 8],
            unlocked_levels: [3; 8],
            volume: 0.4,
        };
        let bytes = bincode::serialize(&legacy).expect("legacy test data should serialize");
        let migrated = SaveData::decode(&bytes).expect("legacy save should migrate");
        assert_eq!(&migrated.high_scores[..8], &[42u32; 8]);
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
        data.age_tier = AgeTier::Senior;
        let envelope = SaveEnvelope {
            magic: SAVE_MAGIC,
            version: SAVE_VERSION,
            data,
        };
        let bytes = bincode::serialize(&envelope).expect("save envelope should serialize");
        let decoded = SaveData::decode(&bytes).expect("current save should decode");
        assert_eq!(decoded.settings.display_mode, DisplayMode::Widescreen16x9);
        assert_eq!(decoded.settings.gameplay_profile, GameplayProfile::Assist);
        assert_eq!(decoded.age_tier, AgeTier::Senior);
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

    #[test]
    fn v2_save_migrates_into_the_first_eight_slots() {
        let migrated = SaveData::decode(&v2_bytes(v2_sample())).expect("v2 save should migrate");
        assert_eq!(&migrated.high_scores[..8], &[11, 22, 33, 44, 55, 66, 77, 88]);
        assert_eq!(&migrated.high_scores[8..], &[0u32; SAVE_SLOTS - 8]);
        // 新槽必须是「已解锁第 1 关」，不是 0 —— 0 会让卡带墙显示 0/10 且进不去。
        assert_eq!(&migrated.unlocked_levels[8..], &[1u8; SAVE_SLOTS - 8]);
        assert_eq!(&migrated.selected_levels[8..], &[1u8; SAVE_SLOTS - 8]);
        assert_eq!(migrated.age_tier, AgeTier::default());
    }

    /// 回归测试：v2 信封有 200+ 字节，如果被 44 字节的 1.x 结构解走，`volume`
    /// 会落在 `unlocked_levels` 的字节上、clamp 后约等于 0，玩家音量凭空归零。
    #[test]
    fn v2_save_is_never_read_as_a_1x_save() {
        let migrated = SaveData::decode(&v2_bytes(v2_sample())).expect("v2 save should migrate");
        assert_eq!(migrated.settings.music_volume, 0.7);
        assert_eq!(migrated.settings.sfx_volume, 0.5);
    }

    /// 未来版本的存档不许被当前构建猜着读。
    #[test]
    fn unknown_version_is_rejected() {
        let mut bytes = v2_bytes(v2_sample());
        bytes[4] = 99;
        assert!(SaveData::decode(&bytes).is_none());
    }

    /// `LegacySaveData` 分支靠精确长度兜底，任何更长的缓冲都不该走进去。
    #[test]
    fn oversized_headerless_buffer_is_rejected() {
        let mut bytes = vec![0u8; LEGACY_SAVE_LEN + 16];
        bytes[0] = 1;
        assert!(SaveData::decode(&bytes).is_none());
    }

    /// `SaveDataV2` 里内嵌的是**活的** `UserSettings`。它一旦改结构，v2 老档就
    /// 解不出来了。这条断言会先红，逼着做一次自觉的决定（冻一份 V2 快照）。
    #[test]
    fn user_settings_wire_size_is_pinned() {
        let size = bincode::serialized_size(&UserSettings::default())
            .expect("settings should serialize");
        assert_eq!(size, 162, "UserSettings 结构变了，v2 存档迁移会失效");
    }

    /// 前 8 个槽位是存档兼容性契约，新游戏只能往后追加。
    #[test]
    fn classic_slots_are_frozen() {
        assert_eq!(
            GameKind::CLASSIC,
            &[
                GameKind::Tank,
                GameKind::BombMaze,
                GameKind::SpaceShooter,
                GameKind::SuperMario,
                GameKind::Contra,
                GameKind::BubbleBobble,
                GameKind::MemoryMatch,
                GameKind::Sokoban,
            ]
        );
        assert_eq!(&GameKind::ALL[..8], GameKind::CLASSIC);
    }

    /// `ALL` 必须是 `CLASSIC` 后接 `PUZZLE`，且 `index()` 与之一一对应。
    #[test]
    fn all_is_classic_then_puzzle_and_matches_index() {
        let joined: Vec<GameKind> = GameKind::CLASSIC
            .iter()
            .chain(GameKind::PUZZLE.iter())
            .copied()
            .collect();
        assert_eq!(GameKind::ALL, joined.as_slice());
        assert!(GameKind::ALL.len() <= SAVE_SLOTS);
        for (slot, kind) in GameKind::ALL.iter().copied().enumerate() {
            assert_eq!(kind.index(), slot, "{} 的槽位对不上", kind.title());
        }
    }

    #[test]
    fn category_routes_every_game_to_exactly_one_tab() {
        for kind in GameKind::ALL.iter().copied() {
            assert!(kind.category().games().contains(&kind));
            assert!(!kind.category().toggled().games().contains(&kind));
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
