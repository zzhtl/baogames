use bevy::prelude::*;

/// 挂在格子的底板与数字上，`index` 是行优先的格子下标。
#[derive(Component, Clone, Copy)]
pub struct SchulteCell {
    pub index: usize,
}

/// 格子底板（需要随状态换色）。
#[derive(Component)]
pub struct SchulteCellFill;

/// 格子描边。
#[derive(Component)]
pub struct SchulteCellBorder;

/// 格子上的数字（需要随状态换色）。
#[derive(Component)]
pub struct SchulteCellText;

/// 跟随光标的选中框。
#[derive(Component)]
pub struct SchulteCursor;

/// 顶栏右侧：关卡 / 剩余时间 / 进度。
#[derive(Component)]
pub struct SchulteHud;

/// 底栏左侧：分数 / 纪录。
#[derive(Component)]
pub struct SchulteScoreHud;

/// 底栏右侧：下一个要点的目标，颜色随双色模式变化。
#[derive(Component)]
pub struct SchulteTarget;
