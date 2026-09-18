use bevy::prelude::*;

/// 一个音板。`pad` 是 0..4，对应上 / 下 / 左 / 右。
#[derive(Component, Clone, Copy)]
pub struct SimonPad {
    pub pad: usize,
}

/// 中央显示轮次的方块。
#[derive(Component)]
pub struct SimonCore;

/// 顶栏右侧：关卡 / 轮次 / 命数。
#[derive(Component)]
pub struct SimonHud;

/// 底栏左侧：分数 / 纪录 / 序列长度。
#[derive(Component)]
pub struct SimonScoreHud;

/// 底栏右侧：当前该做什么（看、按、倒着按）。
#[derive(Component)]
pub struct SimonMessage;
