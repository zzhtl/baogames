use bevy::prelude::*;

// 关卡棋盘：每关 (cols, rows)，cols*rows 必须为偶数。
pub const LEVEL_GRID: [(u32, u32); 10] = [
    (4, 3),
    (4, 4),
    (5, 4),
    (6, 4),
    (6, 5),
    (6, 6),
    (7, 6),
    (8, 6),
    (8, 6),
    (8, 6),
];

// 关卡限时（秒）。后期同样棋盘下时间逐渐变紧。
pub const LEVEL_TIME: [f32; 10] = [
    90.0, 90.0, 100.0, 110.0, 130.0, 150.0, 170.0, 200.0, 170.0, 140.0,
];

// 棋盘区可用空间（中心居中）。
pub const BOARD_W: f32 = 880.0;
pub const BOARD_H: f32 = 380.0;
pub const BOARD_CENTER_Y: f32 = -20.0;

// 卡片之间的间距占 cell 的比例。
pub const CARD_GAP_RATIO: f32 = 0.16;

// 翻错牌后停留时间。
pub const PEEK_TIME: f32 = 0.8;

// 字符库（最多支持 24 对）。
pub const PAIR_CHARS: [&str; 24] = [
    "花", "果", "星", "月", "日", "云", "雨", "雪", "风", "火", "水", "山",
    "海", "鱼", "鸟", "虫", "车", "船", "伞", "糖", "笑", "家", "灯", "书",
];

// 每个 pair_id 的字符颜色（鲜艳且适合儿童）。
pub fn pair_color(idx: u32) -> Color {
    const PALETTE: [Color; 12] = [
        Color::srgb(0.95, 0.32, 0.42),
        Color::srgb(0.98, 0.55, 0.20),
        Color::srgb(0.96, 0.72, 0.20),
        Color::srgb(0.56, 0.78, 0.28),
        Color::srgb(0.20, 0.78, 0.42),
        Color::srgb(0.20, 0.74, 0.78),
        Color::srgb(0.24, 0.58, 0.96),
        Color::srgb(0.40, 0.40, 0.92),
        Color::srgb(0.66, 0.36, 0.94),
        Color::srgb(0.92, 0.40, 0.78),
        Color::srgb(0.92, 0.30, 0.58),
        Color::srgb(0.66, 0.46, 0.28),
    ];
    PALETTE[(idx as usize) % PALETTE.len()]
}

pub const COLOR_BACK_BORDER: Color = Color::srgb(0.55, 0.38, 0.86);
pub const COLOR_BACK_INNER: Color = Color::srgb(0.32, 0.22, 0.58);
pub const COLOR_BACK_MARK: Color = Color::srgb(1.0, 0.94, 0.62);

pub const COLOR_FACE_BORDER: Color = Color::srgb(0.94, 0.86, 0.62);
pub const COLOR_FACE_INNER: Color = Color::srgb(1.0, 0.98, 0.92);
pub const COLOR_FACE_MATCHED_BORDER: Color = Color::srgb(0.96, 0.66, 0.20);
pub const COLOR_FACE_MATCHED_INNER: Color = Color::srgb(1.0, 0.92, 0.46);

pub const COLOR_CURSOR: Color = Color::srgb(1.0, 0.94, 0.32);
