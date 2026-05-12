use bevy::prelude::*;

use super::constants::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerState {
    Small,
    Big,
    Fire,
}

impl PowerState {
    pub fn size(self) -> Vec2 {
        match self {
            PowerState::Small => Vec2::new(PLAYER_W, PLAYER_H),
            PowerState::Big | PowerState::Fire => Vec2::new(PLAYER_W + 2.0, BIG_PLAYER_H),
        }
    }
}

#[derive(Component)]
pub struct MarioPlayer {
    pub vel: Vec2,
    pub on_ground: bool,
    pub jumping: bool,
    pub facing: Facing,
    pub invincible: f32,
    pub dead_timer: f32,
    pub finished: bool,
    pub state: PowerState,
    pub transform_t: f32,
    pub fire_cd: f32,
}

#[derive(Component)]
pub struct MarioVisual;

#[derive(Component)]
pub struct MarioBackground;

#[derive(Component, Clone, Copy)]
pub struct Solid {
    pub size: Vec2,
}

#[derive(Component)]
pub struct GroundTile;

#[derive(Component)]
pub struct BrickTile;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum QuestionContent {
    Coin,
    Mushroom,
    Star,
    OneUp,
}

#[derive(Component)]
pub struct QuestionBlock {
    pub used: bool,
    pub bump_t: f32,
    pub base_y: f32,
    pub content: QuestionContent,
    pub spawn_done: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpKind {
    Mushroom,
    FireFlower,
    OneUp,
    Star,
}

#[derive(Component)]
pub struct PowerUp {
    pub kind: PowerUpKind,
    pub vel: Vec2,
    pub emerge_y_target: f32,
    pub emerged: bool,
    pub on_ground: bool,
}

#[derive(Component)]
pub struct Fireball {
    pub vel: Vec2,
    pub life: f32,
}

#[derive(Component)]
pub struct BrickShard {
    pub vel: Vec2,
    pub life: f32,
    pub spin: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KoopaKind {
    Green,
    Red,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KoopaState {
    Walking,
    Shell,
    SlidingShell,
}

#[derive(Component)]
pub struct Koopa {
    pub kind: KoopaKind,
    pub vel: Vec2,
    pub on_ground: bool,
    pub state: KoopaState,
    pub state_t: f32,
    pub dead: bool,
}

#[derive(Component)]
pub struct KoopaVisual;

#[derive(Component)]
pub struct MovingPlatform {
    pub vertical: bool,
    pub center: Vec2,
    pub vel: Vec2,
    pub last_dx: f32,
    pub last_dy: f32,
}

#[derive(Component)]
pub struct LavaTile;

#[derive(Component)]
pub struct DarkBrick;

#[derive(Component)]
pub struct Bowser {
    pub vel: Vec2,
    pub on_ground: bool,
    pub hp: i32,
    pub fire_cd: f32,
    pub dir_t: f32,
    pub dead: bool,
}

#[derive(Component)]
pub struct BowserFireball {
    pub vel: Vec2,
    pub life: f32,
}

#[derive(Component)]
pub struct Axe;

#[derive(Component)]
pub struct StoneTile;

#[derive(Component)]
pub struct PipeTile;

#[derive(Component)]
pub struct Goomba {
    pub vel: Vec2,
    pub on_ground: bool,
    pub squashed: f32,
    pub dead: bool,
}

#[derive(Component)]
pub struct GoombaVisual;

#[derive(Component)]
pub struct CoinPopup {
    pub timer: f32,
    pub vy: f32,
}

#[derive(Component)]
pub struct FlagPole;

#[derive(Component)]
pub struct FlagBanner {
    pub y_target: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct MarioHud {
    pub kind: HudKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HudKind {
    Score,
    Coins,
    Time,
    World,
    Status,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_size_smaller_than_big() {
        assert!(PowerState::Small.size().y < PowerState::Big.size().y);
        assert_eq!(PowerState::Big.size(), PowerState::Fire.size());
    }
}
