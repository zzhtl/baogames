use bevy::prelude::*;

#[derive(Component)]
pub struct SokoTileSprite;

#[derive(Component)]
pub struct SokoBox {
    pub index: usize,
}

/// 普通木箱组节点，不在目标点上时可见。
#[derive(Component)]
pub struct SokoBoxNormal;

/// 完成态（绿色）箱子组节点，压在目标点上时可见。
#[derive(Component)]
pub struct SokoBoxDone;

#[derive(Component)]
pub struct SokoPlayer;

#[derive(Component)]
pub struct SokobanHud;

#[derive(Component)]
pub struct SokobanMessage;
