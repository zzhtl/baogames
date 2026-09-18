use bevy::prelude::*;

/// 「已找到」的标记圈，两张图各一个，按格子下标一起显隐。
#[derive(Component, Clone, Copy)]
pub struct FoundMark {
    pub index: usize,
}

/// 光标。两张图各一个，同步移动 —— 对比的时候视线要能来回跳。
#[derive(Component, Clone, Copy)]
pub struct SpotCursor {
    pub side: usize,
}

/// 顶栏右侧：关卡 / 时间 / 进度。
#[derive(Component)]
pub struct SpotDiffHud;

/// 底栏左侧：分数 / 纪录 / 标错次数。
#[derive(Component)]
pub struct SpotDiffScoreHud;

/// 底栏右侧：瞬时提示。
#[derive(Component)]
pub struct SpotDiffMessage;
