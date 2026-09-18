use bevy::prelude::*;

/// 一张图案牌，`index` 是它在盘面里的行优先下标（位置固定，内容会变）。
#[derive(Component, Clone, Copy)]
pub struct LinkTile {
    pub index: usize,
}

#[derive(Component)]
pub struct LinkTileEdge;

#[derive(Component)]
pub struct LinkTileFace;

#[derive(Component)]
pub struct LinkTileText;

#[derive(Component)]
pub struct LinkCursor;

/// 配对成功后短暂显示的连线段，到点整批销毁。
#[derive(Component)]
pub struct LinkPathSegment;

/// 顶栏右侧：关卡 / 时间 / 剩余对数。
#[derive(Component)]
pub struct LinkupHud;

/// 底栏左侧：分数 / 纪录 / 连对。
#[derive(Component)]
pub struct LinkupScoreHud;

/// 底栏右侧：瞬时提示。
#[derive(Component)]
pub struct LinkupMessage;
