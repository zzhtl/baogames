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

// 统一字号层级：覆盖层/菜单大标题、HUD 主行、正文、次要提示
pub const FONT_TITLE: f32 = 34.0;
pub const FONT_HEADING: f32 = 20.0;
pub const FONT_BODY: f32 = 16.0;
pub const FONT_SMALL: f32 = 13.0;

// HUD 与覆盖层挂在相机子节点上，z 相对相机；游戏内容最大 z 为 Z_TEXT=10，
// HUD 50 起步、覆盖层 60 起步保证永远压住游戏画面。
pub const Z_HUD_LAYER: f32 = 50.0;
pub const Z_OVERLAY: f32 = 60.0;
pub const Z_OVERLAY_PANEL: f32 = 60.5;
pub const Z_OVERLAY_TEXT: f32 = 61.0;
