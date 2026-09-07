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
    pub coast_left: f32,
    pub hit_t: f32,
    pub motion_t: f32,
    pub moving: bool,
}

#[derive(Component)]
pub struct PlayerTankFC {
    pub id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyTankKind {
    Basic, // 普通
    Fast,  // 快速：移动更快
    Power, // 重炮：子弹更快
    Armor, // 装甲：3 血
}

#[derive(Component)]
pub struct EnemyTankFC {
    pub turn_timer: f32,
    // 供后续道具掉落 / 装甲受损表现使用
    #[allow(dead_code)]
    pub kind: EnemyTankKind,
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpKind {
    Star,    // 升级：多发子弹 + 更快
    Grenade, // 手雷：清掉在场敌人
    Helmet,  // 头盔：获得护盾
    Tank,    // 加命：+1 条命
    Clock,   // 时钟：冻结敌人
    Shovel,  // 铲子：基地周围筑钢墙
}

#[derive(Component)]
pub struct PowerUp {
    pub kind: PowerUpKind,
}

#[derive(Component)]
pub struct SpawnEffect {
    pub timer: Timer,
    pub spawn_pos: Vec2,
    pub side: TankSide,
    pub player_id: Option<usize>,
    pub enemy_kind: Option<EnemyTankKind>,
}

#[derive(Component)]
pub struct TankHud {
    pub kind: TankHudKind,
}

#[derive(Component)]
pub struct P2Hud;

#[derive(Component)]
pub struct ModeSelectUi;

#[derive(Component)]
pub struct TankShieldVisual {
    pub owner: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TankHudKind {
    Stage,
    Enemies,
    P1Lives,
    P2Lives,
    Base,
    Freeze,
}

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
