use bevy::prelude::*;

use super::constants::*;
use super::palette::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Weapon {
    M,
    S,
    F,
    R,
}

impl Weapon {
    pub fn fire_cd(self) -> f32 {
        match self {
            Weapon::M => FIRE_CD_M,
            Weapon::S => FIRE_CD_S,
            Weapon::F => FIRE_CD_F,
            Weapon::R => FIRE_CD_R,
        }
    }
    pub fn pickup_color(self) -> Color {
        match self {
            Weapon::M => COLOR_PICKUP_M,
            Weapon::S => COLOR_PICKUP_S,
            Weapon::F => COLOR_PICKUP_F,
            Weapon::R => COLOR_PICKUP_R,
        }
    }
    pub fn letter(self) -> &'static str {
        match self {
            Weapon::M => "M",
            Weapon::S => "S",
            Weapon::F => "F",
            Weapon::R => "R",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyKind {
    Soldier, // 跑步射击
    Sniper,  // 站立射击
    Jumper,  // 高处跳下后跑动
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AimDir {
    Up,
    UpLeft,
    UpRight,
    Left,
    Right,
    DownLeft,
    DownRight,
    Down,
}

impl AimDir {
    pub fn vec(self) -> Vec2 {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        match self {
            AimDir::Up => Vec2::new(0.0, 1.0),
            AimDir::Down => Vec2::new(0.0, -1.0),
            AimDir::Left => Vec2::new(-1.0, 0.0),
            AimDir::Right => Vec2::new(1.0, 0.0),
            AimDir::UpLeft => Vec2::new(-s, s),
            AimDir::UpRight => Vec2::new(s, s),
            AimDir::DownLeft => Vec2::new(-s, -s),
            AimDir::DownRight => Vec2::new(s, -s),
        }
    }
}

#[derive(Component)]
pub struct ContraPlayer {
    pub vel: Vec2,
    pub on_ground: bool,
    pub prone: bool,
    pub facing: f32,
    pub aim: AimDir,
    pub weapon: Weapon,
    pub fire_cd: f32,
    pub dead_timer: f32,
    pub invincible: f32,
    pub finish: bool,
}

#[derive(Component)]
pub struct ContraBackground;

#[derive(Component, Clone, Copy)]
pub struct ContraSolid {
    pub size: Vec2,
}

#[derive(Component)]
pub struct ContraEnemy {
    pub kind: EnemyKind,
    pub vel: Vec2,
    pub on_ground: bool,
    pub facing: f32,
    pub fire_cd: f32,
    pub ai_t: f32,
    pub hp: i32,
}

#[derive(Component)]
pub struct ContraBullet {
    pub vel: Vec2,
    pub from_player: bool,
    pub weapon: Weapon,
    pub life: f32,
}

#[derive(Component)]
pub struct ContraFalcon {
    pub vel: Vec2,
    pub weapon: Weapon,
}

#[derive(Component)]
pub struct ContraPickup {
    pub weapon: Weapon,
    pub vel_y: f32,
    pub on_ground: bool,
    pub pulse: f32,
}

#[derive(Component)]
pub struct ContraExplosion {
    pub t: f32,
    pub max_t: f32,
    pub size: f32,
}

#[derive(Component)]
pub struct ContraTurret {
    pub fire_cd: f32,
    pub hp: i32,
}

#[derive(Component)]
pub struct ContraBoss {
    pub hp: i32,
    pub die_t: f32,
    pub flash_t: f32,
    pub spawn_t: f32,
}

#[derive(Component, Clone, Copy)]
pub struct ContraHud {
    pub kind: ContraHudKind,
    pub offset: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContraHudKind {
    Score,
    TopScore,
    Lives,
    Weapon,
    WeaponLetter,
    World,
    Status,
    BossHp,
}

// 命数图标（顶部 HUD 上显示的小 Bill 头）
#[derive(Component)]
pub struct ContraHudLifeIcon {
    pub idx: i32,
}

#[derive(Component)]
pub struct ContraHudPanel {
    pub offset: Vec2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aim_dir_vec_has_unit_length() {
        for d in [
            AimDir::Up,
            AimDir::Down,
            AimDir::Left,
            AimDir::Right,
            AimDir::UpLeft,
            AimDir::UpRight,
            AimDir::DownLeft,
            AimDir::DownRight,
        ] {
            let v = d.vec();
            assert!((v.length() - 1.0).abs() < 1e-5, "{:?} not unit", d);
        }
    }

    #[test]
    fn weapon_fire_cd_matches_table() {
        assert_eq!(Weapon::M.fire_cd(), FIRE_CD_M);
        assert_eq!(Weapon::S.fire_cd(), FIRE_CD_S);
        assert_eq!(Weapon::F.fire_cd(), FIRE_CD_F);
        assert_eq!(Weapon::R.fire_cd(), FIRE_CD_R);
    }

    #[test]
    fn weapon_letters_distinct() {
        let letters: Vec<&str> = [Weapon::M, Weapon::S, Weapon::F, Weapon::R]
            .iter()
            .map(|w| w.letter())
            .collect();
        // 4 个，全部不同
        let mut sorted = letters.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 4);
    }
}
