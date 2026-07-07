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

/// 卡背组节点（紫色面板 + 问号），FaceDown 时可见。
#[derive(Component)]
pub struct CardBack;

/// 卡面组节点（米白面板 + 字符），FaceUp 时可见。
#[derive(Component)]
pub struct CardFace;

/// 已配对卡面组节点（金色面板 + 字符），Matched 时可见。
#[derive(Component)]
pub struct CardFaceMatched;

/// 4 条边组成的选中框，offset 是相对于当前光标所在卡中心的偏移。
#[derive(Component)]
pub struct CardCursor {
    pub offset: Vec2,
}

#[derive(Component)]
pub struct MemoryHud;

#[derive(Component)]
pub struct MemoryMessage;
