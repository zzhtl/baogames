use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    FaceDown,
    FaceUp,
    Matched,
}

#[derive(Component)]
pub struct MemoryCard {
    pub col: i32,
    pub row: i32,
    pub pair_id: u32,
    pub state: CardState,
}

/// 卡背（紫色面板 + 问号）相关子实体。
#[derive(Component)]
pub struct CardBack;

/// 卡面（奶油色或金色面板 + 字符）相关子实体。
#[derive(Component)]
pub struct CardFace;

/// 卡面边框：颜色随 state 切换。
#[derive(Component)]
pub struct CardFaceBorder;

/// 卡面内层填充：颜色随 state 切换。
#[derive(Component)]
pub struct CardFaceInner;

/// 4 条边组成的选中框，offset 是相对于当前光标所在卡中心的偏移。
#[derive(Component)]
pub struct CardCursor {
    pub offset: Vec2,
}

#[derive(Component)]
pub struct MemoryHud;

#[derive(Component)]
pub struct MemoryMessage;
