use super::px::px;

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

// 字号只有两级，且都必须是点阵字体原生 12px 的整数倍。
//
// 字体是 fusion-pixel-12px：字形按 12 的整数倍光栅化才有干净的点阵边缘，
// 别的尺寸会让笔画粗细不匀（渲染目标的 scale_factor 见 pixel_canvas）。
// 240 画布像素宽下 24px 一行只放得下 10 个汉字，所以第三级字号没有意义——
// 层级改用颜色和位置表达。
pub const FONT_TITLE: f32 = px(24.0); // 72 世界单位 = 24 画布像素
pub const FONT_BODY: f32 = px(12.0); //  36 世界单位 = 12 画布像素（原生）

// HUD 与覆盖层挂在相机子节点上，z 相对相机；游戏内容最大 z 为 Z_TEXT=10，
// HUD 50 起步、覆盖层 60 起步保证永远压住游戏画面。
pub const Z_HUD_LAYER: f32 = 50.0;
pub const Z_OVERLAY: f32 = 60.0;
pub const Z_OVERLAY_PANEL: f32 = 60.5;
pub const Z_OVERLAY_TEXT: f32 = 61.0;
