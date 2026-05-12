use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TankDir {
    Up,
    Down,
    Left,
    Right,
}

impl TankDir {
    pub fn vec(self) -> Vec2 {
        match self {
            TankDir::Up => Vec2::new(0.0, 1.0),
            TankDir::Down => Vec2::new(0.0, -1.0),
            TankDir::Left => Vec2::new(-1.0, 0.0),
            TankDir::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn rotation(self) -> f32 {
        use std::f32::consts::*;
        match self {
            TankDir::Up => 0.0,
            TankDir::Left => FRAC_PI_2,
            TankDir::Down => PI,
            TankDir::Right => -FRAC_PI_2,
        }
    }

    pub fn from_input(v: Vec2) -> Option<Self> {
        if v.length_squared() < 0.05 {
            return None;
        }
        if v.x.abs() >= v.y.abs() {
            Some(if v.x > 0.0 {
                TankDir::Right
            } else {
                TankDir::Left
            })
        } else {
            Some(if v.y > 0.0 {
                TankDir::Up
            } else {
                TankDir::Down
            })
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum TankSide {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct TankFC {
    pub side: TankSide,
    pub speed: f32,
    pub fire_cd: f32,
    pub fire_cd_left: f32,
    pub bullet_speed: f32,
    pub max_bullets: u8,
    pub bullets_alive: u8,
    pub hp: u8,
    pub shield_left: f32,
}

#[derive(Component)]
pub struct PlayerTankFC {
    pub id: usize,
}

#[derive(Component)]
pub struct EnemyTankFC {
    pub turn_timer: f32,
}

#[derive(Component)]
pub struct BrickFC;

#[derive(Component)]
pub struct SteelFC;

#[derive(Component)]
pub struct WaterFC;

#[derive(Component)]
pub struct BushFC;

#[derive(Component)]
pub struct IceFC;

#[derive(Component)]
pub struct BaseFC;

#[derive(Component)]
pub struct BulletFC {
    pub side: TankSide,
    #[allow(dead_code)]
    pub dir: TankDir,
    #[allow(dead_code)]
    pub power: u8,
    pub owner: Option<Entity>,
}

#[derive(Component)]
pub struct SpawnEffect {
    pub timer: Timer,
    pub spawn_pos: Vec2,
    pub side: TankSide,
    pub player_id: Option<usize>,
}

#[derive(Component)]
pub struct TankHud;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input_picks_dominant_axis() {
        assert_eq!(TankDir::from_input(Vec2::new(1.0, 0.2)), Some(TankDir::Right));
        assert_eq!(TankDir::from_input(Vec2::new(-1.0, 0.2)), Some(TankDir::Left));
        assert_eq!(TankDir::from_input(Vec2::new(0.1, 1.0)), Some(TankDir::Up));
        assert_eq!(TankDir::from_input(Vec2::new(0.1, -1.0)), Some(TankDir::Down));
    }

    #[test]
    fn from_input_dead_zone_returns_none() {
        assert_eq!(TankDir::from_input(Vec2::ZERO), None);
        assert_eq!(TankDir::from_input(Vec2::new(0.1, 0.1)), None);
    }

    #[test]
    fn vec_is_unit_axis() {
        assert_eq!(TankDir::Up.vec(), Vec2::new(0.0, 1.0));
        assert_eq!(TankDir::Down.vec(), Vec2::new(0.0, -1.0));
        assert_eq!(TankDir::Left.vec(), Vec2::new(-1.0, 0.0));
        assert_eq!(TankDir::Right.vec(), Vec2::new(1.0, 0.0));
    }
}
