use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::game::puzzle::grid::{GridCursor, GridLayout};

#[derive(Resource)]
pub struct LinkupStage {
    pub layout: GridLayout,
    pub cols: i32,
    pub rows: i32,
    /// 行优先，`None` 表示已经消掉。
    pub tiles: Vec<Option<u8>>,
    pub gravity: bool,
    pub cursor: GridCursor,
    /// 已选中的第一张牌。
    pub selected: Option<(i32, i32)>,
    /// 刚配对成功的连线折点，配合 `path_clock` 短暂显示。
    pub path: Vec<(i32, i32)>,
    pub path_clock: f32,
    pub pairs_left: usize,
    pub streak: u32,
    pub best_streak: u32,
    pub shuffles: u32,
    pub time_left: f32,
    pub message: String,
    pub message_clock: f32,
}

impl LinkupStage {
    pub fn tile_at(&self, col: i32, row: i32) -> Option<u8> {
        self.index(col, row).and_then(|index| self.tiles[index])
    }

    pub fn index(&self, col: i32, row: i32) -> Option<usize> {
        (col >= 0 && row >= 0 && col < self.cols && row < self.rows)
            .then(|| (row * self.cols + col) as usize)
    }

    pub fn is_cleared(&self) -> bool {
        self.tiles.iter().all(|tile| tile.is_none())
    }
}

/// 生成一张成对的盘面：每种图案都是偶数张，位置随机。
pub fn build_tiles<R: Rng + ?Sized>(cols: i32, rows: i32, patterns: usize, rng: &mut R) -> Vec<Option<u8>> {
    let cells = (cols * rows) as usize;
    let pairs = cells / 2;
    let patterns = patterns.clamp(1, pairs);
    let mut deck: Vec<Option<u8>> = (0..pairs)
        .flat_map(|pair| {
            let pattern = (pair % patterns) as u8;
            [Some(pattern), Some(pattern)]
        })
        .collect();
    // 格数是奇数时补一个空格，保证长度对得上（现有关卡都是偶数，这里只是兜底）。
    deck.resize(cells, None);
    deck.shuffle(rng);
    deck
}
