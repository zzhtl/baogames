use bevy::prelude::*;

/// 挂在格子底板与数字上，`index` 是行优先下标。
#[derive(Component, Clone, Copy)]
pub struct SudokuCell {
    pub index: usize,
}

#[derive(Component)]
pub struct SudokuCellFill;

#[derive(Component)]
pub struct SudokuCellText;

#[derive(Component)]
pub struct SudokuCursor;

/// 顶栏右侧：关卡 / 剩余时间 / 已填格数。
#[derive(Component)]
pub struct SudokuHud;

/// 底栏左侧：分数 / 纪录。
#[derive(Component)]
pub struct SudokuScoreHud;

/// 底栏右侧：瞬时提示。
#[derive(Component)]
pub struct SudokuMessage;
