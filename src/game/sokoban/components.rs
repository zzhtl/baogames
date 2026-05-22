use bevy::prelude::*;

#[derive(Component)]
pub struct SokoTileSprite;

#[derive(Component)]
pub struct SokoBox {
    pub index: usize,
}

#[derive(Component)]
pub struct SokoBoxBorder;

#[derive(Component)]
pub struct SokoBoxInner;

#[derive(Component)]
pub struct SokoPlayer;

#[derive(Component)]
pub struct SokobanHud;

#[derive(Component)]
pub struct SokobanMessage;
