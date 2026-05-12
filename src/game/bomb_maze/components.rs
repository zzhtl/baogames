use bevy::prelude::*;

use super::constants::{BM_INVULN_TIME, BM_PLAYER_SPEED_BASE, BM_PLAYER_SPEED_BONUS};

// ========== 方向 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Dir4 {
    pub fn vec(self) -> Vec2 {
        match self {
            Dir4::Up => Vec2::new(0.0, 1.0),
            Dir4::Down => Vec2::new(0.0, -1.0),
            Dir4::Left => Vec2::new(-1.0, 0.0),
            Dir4::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Dir4::Up => Dir4::Down,
            Dir4::Down => Dir4::Up,
            Dir4::Left => Dir4::Right,
            Dir4::Right => Dir4::Left,
        }
    }

    pub fn all() -> [Dir4; 4] {
        [Dir4::Up, Dir4::Down, Dir4::Left, Dir4::Right]
    }
}

// ========== 道具 ==========
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum PowerupKind {
    Fire,
    Bomb,
    Speed,
    BombPass,
    Detonator,
}

impl PowerupKind {
    pub fn color(self) -> Color {
        match self {
            PowerupKind::Fire => Color::srgb(1.0, 0.55, 0.18),
            PowerupKind::Bomb => Color::srgb(0.95, 0.95, 0.95),
            PowerupKind::Speed => Color::srgb(0.4, 0.85, 1.0),
            PowerupKind::BombPass => Color::srgb(0.8, 0.5, 1.0),
            PowerupKind::Detonator => Color::srgb(1.0, 0.3, 0.3),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PowerupKind::Fire => "火",
            PowerupKind::Bomb => "弹",
            PowerupKind::Speed => "速",
            PowerupKind::BombPass => "穿",
            PowerupKind::Detonator => "控",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HiddenItem {
    Nothing,
    Powerup(PowerupKind),
    Exit,
}

// ========== 敌人 ==========
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Balloom,  // 慢，纯随机
    Oneal,    // 中，偶尔追踪
    Doll,     // 快，快速换向
    Kondoria, // 慢但能穿软砖
}

impl EnemyKind {
    pub fn body_color(self) -> Color {
        match self {
            EnemyKind::Balloom => Color::srgb(0.85, 0.45, 0.85),
            EnemyKind::Oneal => Color::srgb(0.95, 0.45, 0.32),
            EnemyKind::Doll => Color::srgb(0.45, 0.85, 0.45),
            EnemyKind::Kondoria => Color::srgb(0.55, 0.45, 0.78),
        }
    }

    pub fn eye_color(self) -> Color {
        Color::srgb(1.0, 1.0, 1.0)
    }

    pub fn speed(self) -> f32 {
        match self {
            EnemyKind::Balloom => 50.0,
            EnemyKind::Oneal => 70.0,
            EnemyKind::Doll => 95.0,
            EnemyKind::Kondoria => 45.0,
        }
    }

    pub fn score(self) -> u32 {
        match self {
            EnemyKind::Balloom => 100,
            EnemyKind::Oneal => 200,
            EnemyKind::Doll => 400,
            EnemyKind::Kondoria => 1000,
        }
    }

    pub fn hunt(self) -> f32 {
        match self {
            EnemyKind::Balloom => 0.0,
            EnemyKind::Oneal => 0.35,
            EnemyKind::Doll => 0.45,
            EnemyKind::Kondoria => 0.2,
        }
    }

    pub fn wall_pass(self) -> bool {
        matches!(self, EnemyKind::Kondoria)
    }
}

// ========== 组件 ==========
#[derive(Component)]
pub struct BMPlayer {
    pub id: usize,
    pub speed_level: u8,
    pub max_bombs: u8,
    pub bombs_alive: u8,
    pub bomb_range: i32,
    pub bomb_pass: bool,
    pub remote: bool,
    pub place_cd: f32,
    pub detonate_cd: f32,
    pub walking_off: Vec<Entity>,
    pub invuln: f32,
}

impl BMPlayer {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            speed_level: 0,
            max_bombs: 1,
            bombs_alive: 0,
            bomb_range: 1,
            bomb_pass: false,
            remote: false,
            place_cd: 0.0,
            detonate_cd: 0.0,
            walking_off: Vec::new(),
            invuln: BM_INVULN_TIME,
        }
    }

    pub fn speed(&self) -> f32 {
        BM_PLAYER_SPEED_BASE + self.speed_level as f32 * BM_PLAYER_SPEED_BONUS
    }

    /// 应用道具效果。
    pub fn apply_powerup(&mut self, kind: PowerupKind) {
        match kind {
            PowerupKind::Fire => self.bomb_range = (self.bomb_range + 1).min(8),
            PowerupKind::Bomb => self.max_bombs = (self.max_bombs + 1).min(8),
            PowerupKind::Speed => self.speed_level = (self.speed_level + 1).min(4),
            PowerupKind::BombPass => self.bomb_pass = true,
            PowerupKind::Detonator => self.remote = true,
        }
    }
}

#[derive(Component)]
pub struct BMBomb {
    pub fuse: Timer,
    pub range: i32,
    pub owner: Option<Entity>,
    pub remote: bool,
    pub triggered: bool,
}

#[derive(Component)]
pub struct BMFlame {
    pub timer: Timer,
}

#[derive(Component)]
pub struct BMHardWall;

#[derive(Component)]
pub struct BMSoftWall {
    pub hides: HiddenItem,
}

#[derive(Component)]
pub struct BMPowerup {
    pub kind: PowerupKind,
}

#[derive(Component)]
pub struct BMExit;

#[derive(Component)]
pub struct BMEnemy {
    pub kind: EnemyKind,
    pub dir: Dir4,
    pub change_timer: f32,
}

#[derive(Component)]
pub struct BMTilePos {
    pub col: i32,
    pub row: i32,
}

#[derive(Component, Clone, Copy)]
pub enum BMHud {
    Stage,
    Time,
    Score,
    P1Lives,
    P2Lives,
    Enemies,
    Status,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir4_opposite_is_involution() {
        for d in Dir4::all() {
            assert_eq!(d.opposite().opposite(), d);
        }
    }

    #[test]
    fn dir4_vec_is_unit_axis() {
        assert_eq!(Dir4::Up.vec(), Vec2::new(0.0, 1.0));
        assert_eq!(Dir4::Down.vec(), Vec2::new(0.0, -1.0));
        assert_eq!(Dir4::Left.vec(), Vec2::new(-1.0, 0.0));
        assert_eq!(Dir4::Right.vec(), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn apply_powerup_increments_with_clamp() {
        let mut p = BMPlayer::new(0);
        assert_eq!(p.bomb_range, 1);
        for _ in 0..20 {
            p.apply_powerup(PowerupKind::Fire);
        }
        assert_eq!(p.bomb_range, 8);

        for _ in 0..20 {
            p.apply_powerup(PowerupKind::Bomb);
        }
        assert_eq!(p.max_bombs, 8);

        for _ in 0..20 {
            p.apply_powerup(PowerupKind::Speed);
        }
        assert_eq!(p.speed_level, 4);
    }

    #[test]
    fn apply_powerup_flags() {
        let mut p = BMPlayer::new(0);
        assert!(!p.bomb_pass);
        assert!(!p.remote);
        p.apply_powerup(PowerupKind::BombPass);
        p.apply_powerup(PowerupKind::Detonator);
        assert!(p.bomb_pass);
        assert!(p.remote);
    }

    #[test]
    fn speed_scales_with_level() {
        let mut p = BMPlayer::new(0);
        let base = p.speed();
        p.apply_powerup(PowerupKind::Speed);
        assert!(p.speed() > base);
    }
}
