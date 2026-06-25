use bevy::prelude::*;

#[derive(Component)]
pub struct SpaceShipPlayer {
    pub fire_cd_left: f32,
    pub invincible_left: f32,
    pub blink_phase: f32,
}

#[derive(Component)]
pub struct SpaceBullet {
    pub vel: Vec2,
    pub from_player: bool,
    pub damage: i32,
}

#[derive(Component)]
pub struct SpaceEnemy {
    pub kind: EnemyKind,
    pub hp: i32,
    pub points: u32,
    pub fire_cd_left: f32,
    pub time_alive: f32,
    pub spawn_x: f32,
    pub drops_power: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyKind {
    Scout,
    Sniper,
    Bomber,
    Carrier,
    Boss,
}

impl EnemyKind {
    pub fn size(self) -> Vec2 {
        match self {
            EnemyKind::Scout => Vec2::new(28.0, 28.0),
            EnemyKind::Sniper => Vec2::new(34.0, 24.0),
            EnemyKind::Bomber => Vec2::new(54.0, 36.0),
            EnemyKind::Carrier => Vec2::new(32.0, 32.0),
            EnemyKind::Boss => Vec2::new(170.0, 110.0),
        }
    }
    pub fn collider(self) -> Vec2 {
        let s = self.size();
        Vec2::new(s.x * 0.85, s.y * 0.85)
    }
    pub fn hp(self) -> i32 {
        match self {
            EnemyKind::Scout => 1,
            EnemyKind::Sniper => 2,
            EnemyKind::Bomber => 5,
            EnemyKind::Carrier => 3,
            EnemyKind::Boss => 80,
        }
    }
    pub fn points(self) -> u32 {
        match self {
            EnemyKind::Scout => 100,
            EnemyKind::Sniper => 200,
            EnemyKind::Bomber => 350,
            EnemyKind::Carrier => 250,
            EnemyKind::Boss => 5000,
        }
    }
    pub fn initial_cd(self) -> f32 {
        match self {
            EnemyKind::Scout => 1.4,
            EnemyKind::Sniper => 1.0,
            EnemyKind::Bomber => 1.2,
            EnemyKind::Carrier => 1.6,
            EnemyKind::Boss => 1.5,
        }
    }
    /// 给定关卡难度下的射击间隔（秒）。
    pub fn fire_period(self, level: u8) -> f32 {
        let lvl = level as f32;
        match self {
            EnemyKind::Scout => (2.4 - lvl * 0.12).max(1.4),
            EnemyKind::Sniper => (1.6 - lvl * 0.08).max(0.9),
            EnemyKind::Bomber => (1.8 - lvl * 0.08).max(1.0),
            EnemyKind::Carrier => (2.2 - lvl * 0.1).max(1.2),
            EnemyKind::Boss => (1.4 - lvl * 0.05).max(0.8),
        }
    }
}

#[derive(Component)]
pub struct SpacePowerUp;

#[derive(Component)]
pub struct SpaceStar {
    pub speed: f32,
}

#[derive(Component)]
pub struct SpaceFrame;

#[derive(Component)]
pub struct SpaceHud;

#[derive(Component)]
pub struct SpaceMessageText;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boss_has_highest_hp_and_points() {
        for k in [
            EnemyKind::Scout,
            EnemyKind::Sniper,
            EnemyKind::Bomber,
            EnemyKind::Carrier,
        ] {
            assert!(EnemyKind::Boss.hp() > k.hp());
            assert!(EnemyKind::Boss.points() > k.points());
        }
    }

    #[test]
    fn collider_smaller_than_visual_size() {
        for k in [EnemyKind::Scout, EnemyKind::Boss] {
            let s = k.size();
            let c = k.collider();
            assert!(c.x < s.x && c.y < s.y);
        }
    }

    #[test]
    fn fire_period_clamped_at_high_level() {
        // 高关卡时不会无限缩短到 0 或负数
        let lvl = 20_u8;
        assert!(EnemyKind::Scout.fire_period(lvl) >= 1.4);
        assert!(EnemyKind::Sniper.fire_period(lvl) >= 0.9);
        assert!(EnemyKind::Bomber.fire_period(lvl) >= 1.0);
        assert!(EnemyKind::Carrier.fire_period(lvl) >= 1.2);
        assert!(EnemyKind::Boss.fire_period(lvl) >= 0.8);
    }

    #[test]
    fn fire_period_decreases_with_level() {
        assert!(EnemyKind::Scout.fire_period(1) > EnemyKind::Scout.fire_period(5));
    }
}
