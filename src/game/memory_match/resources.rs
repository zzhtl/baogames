use bevy::prelude::*;

#[derive(Resource)]
pub struct MemoryStage {
    pub cols: u32,
    pub rows: u32,
    pub cell_size: f32,
    /// 棋盘左上角第一张卡（col=0, row=0）的中心坐标。
    pub origin: Vec2,
    pub pairs_total: u32,
    pub pairs_done: u32,
    pub flips: u32,
    pub time_left: f32,
    pub cursor_col: i32,
    pub cursor_row: i32,
    pub first_pick: Option<Entity>,
    pub second_pick: Option<Entity>,
    /// 两张牌翻开后停留对比的剩余时间。
    pub resolve_timer: f32,
    pub message: String,
    pub message_clock: f32,
}

impl MemoryStage {
    pub fn cell_center(&self, col: i32, row: i32) -> Vec2 {
        Vec2::new(
            self.origin.x + col as f32 * self.cell_size,
            self.origin.y - row as f32 * self.cell_size,
        )
    }
}
