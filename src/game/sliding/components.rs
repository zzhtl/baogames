use bevy::prelude::*;

/// 一块滑块。`value` 是它的编号（1 起），不随移动改变。
#[derive(Component, Clone, Copy)]
pub struct SlideTile {
    pub value: u8,
}

/// 块的描边与面，需要随「是否归位」换色。
#[derive(Component)]
pub struct SlideTileEdge;

#[derive(Component)]
pub struct SlideTileFace;

/// 顶栏右侧：关卡 / 时间 / 归位进度。
#[derive(Component)]
pub struct SlidingHud;

/// 底栏左侧：分数 / 纪录 / 步数。
#[derive(Component)]
pub struct SlidingScoreHud;

/// 底栏右侧：瞬时提示。
#[derive(Component)]
pub struct SlidingMessage;
