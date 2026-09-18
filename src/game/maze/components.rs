use bevy::prelude::*;

/// 一块瓦片（墙或地面），带瓦片坐标，雾视野按它做显隐。
#[derive(Component, Clone, Copy)]
pub struct MazeTile {
    pub col: i32,
    pub row: i32,
}

/// 地面瓦片，走过之后换成带脚印的颜色。
#[derive(Component)]
pub struct MazeFloor;

#[derive(Component)]
pub struct MazePlayer;

#[derive(Component)]
pub struct MazeKey;

#[derive(Component)]
pub struct MazeExit;

/// 顶栏右侧：关卡 / 时间 / 钥匙状态。
#[derive(Component)]
pub struct MazeHud;

/// 底栏左侧：分数 / 纪录 / 步数。
#[derive(Component)]
pub struct MazeScoreHud;

/// 底栏右侧：瞬时提示。
#[derive(Component)]
pub struct MazeMessage;
