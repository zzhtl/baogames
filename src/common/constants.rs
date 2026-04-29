// 逻辑画面区域：所有游戏元素仍按这个坐标系摆放，不需要改动
pub const ARENA_W: f32 = 960.0;
pub const ARENA_H: f32 = 540.0;
// 实际窗口分辨率：通过相机正交投影自动等比放大
pub const WINDOW_W: u32 = 1280;
pub const WINDOW_H: u32 = 720;
pub const SAVE_FILE: &str = "baogames.save";
pub const Z_BACKGROUND: f32 = -10.0;
pub const Z_SPRITE: f32 = 0.0;
pub const Z_TEXT: f32 = 10.0;
