use bevy::math::Vec2;

use super::components::EnemyKind;
use super::constants::{PLAY_LEFT, PLAY_OFFSET_X, PLAY_RIGHT, PLAY_TOP, PLAY_W};
use super::resources::PendingSpawn;

/// 为给定波次构造一组延时刷怪事件。
pub fn build_wave(wave_idx: usize, level: u8) -> Vec<PendingSpawn> {
    let mut spawns = Vec::new();
    let lvl_bonus = level.saturating_sub(1) as usize;
    match wave_idx {
        0 => {
            // V 字侦察兵
            for i in 0..7 {
                let x = -180.0 + i as f32 * 60.0 + PLAY_OFFSET_X;
                let dy = (i as f32 - 3.0).abs() * 24.0;
                spawns.push(PendingSpawn {
                    delay: 0.4 + i as f32 * 0.18,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, PLAY_TOP + 40.0 + dy),
                    drops_power: false,
                });
            }
        }
        1 => {
            // 斜线编队
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.35,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_LEFT + 40.0 + i as f32 * 18.0,
                        PLAY_TOP + 30.0 + i as f32 * 30.0,
                    ),
                    drops_power: false,
                });
                spawns.push(PendingSpawn {
                    delay: 0.5 + i as f32 * 0.35,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_RIGHT - 40.0 - i as f32 * 18.0,
                        PLAY_TOP + 30.0 + i as f32 * 30.0,
                    ),
                    drops_power: false,
                });
            }
        }
        2 => {
            // 狙击手
            for i in 0..(3 + lvl_bonus.min(2)) {
                let x = -150.0 + i as f32 * 100.0 + PLAY_OFFSET_X;
                spawns.push(PendingSpawn {
                    delay: 0.5 + i as f32 * 0.6,
                    kind: EnemyKind::Sniper,
                    pos: Vec2::new(x, PLAY_TOP + 60.0),
                    drops_power: false,
                });
            }
            // 一颗补给
            spawns.push(PendingSpawn {
                delay: 1.6,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 80.0),
                drops_power: true,
            });
        }
        3 => {
            // 重型轰炸机
            for i in 0..2 {
                spawns.push(PendingSpawn {
                    delay: 0.6 + i as f32 * 1.4,
                    kind: EnemyKind::Bomber,
                    pos: Vec2::new(
                        if i == 0 { -90.0 } else { 90.0 } + PLAY_OFFSET_X,
                        PLAY_TOP + 80.0,
                    ),
                    drops_power: i == 1,
                });
            }
            // 两侧侦察
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 1.0 + i as f32 * 0.25,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        if i % 2 == 0 { PLAY_LEFT + 40.0 } else { PLAY_RIGHT - 40.0 },
                        PLAY_TOP + 30.0 + i as f32 * 22.0,
                    ),
                    drops_power: false,
                });
            }
        }
        4 => {
            // 弧形侦察
            for i in 0..9 {
                let t = i as f32 / 8.0;
                let x = PLAY_OFFSET_X + (t - 0.5) * 360.0;
                let y = PLAY_TOP + 30.0 + (t - 0.5).abs() * 60.0;
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.16,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, y),
                    drops_power: false,
                });
            }
        }
        5 => {
            // 狙击 + 补给
            for i in 0..4 {
                spawns.push(PendingSpawn {
                    delay: 0.4 + i as f32 * 0.5,
                    kind: EnemyKind::Sniper,
                    pos: Vec2::new(
                        PLAY_OFFSET_X + (i as f32 - 1.5) * 90.0,
                        PLAY_TOP + 60.0,
                    ),
                    drops_power: false,
                });
            }
            spawns.push(PendingSpawn {
                delay: 2.4,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X - 60.0, PLAY_TOP + 100.0),
                drops_power: true,
            });
            spawns.push(PendingSpawn {
                delay: 2.8,
                kind: EnemyKind::Carrier,
                pos: Vec2::new(PLAY_OFFSET_X + 60.0, PLAY_TOP + 100.0),
                drops_power: true,
            });
        }
        6 => {
            // 轰炸机 + 群侦察
            spawns.push(PendingSpawn {
                delay: 0.4,
                kind: EnemyKind::Bomber,
                pos: Vec2::new(PLAY_OFFSET_X, PLAY_TOP + 80.0),
                drops_power: false,
            });
            for i in 0..10 {
                let x = PLAY_LEFT + 30.0 + (i as f32 / 9.0) * (PLAY_W - 60.0);
                spawns.push(PendingSpawn {
                    delay: 1.2 + i as f32 * 0.18,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(x, PLAY_TOP + 40.0),
                    drops_power: false,
                });
            }
        }
        _ => {
            // 兜底：再来几个
            for i in 0..6 {
                spawns.push(PendingSpawn {
                    delay: 0.3 + i as f32 * 0.3,
                    kind: EnemyKind::Scout,
                    pos: Vec2::new(
                        PLAY_OFFSET_X + (i as f32 - 2.5) * 70.0,
                        PLAY_TOP + 40.0,
                    ),
                    drops_power: false,
                });
            }
        }
    }
    spawns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_named_wave_produces_spawns() {
        for idx in 0..7 {
            assert!(!build_wave(idx, 1).is_empty(), "wave {} empty", idx);
        }
    }

    #[test]
    fn fallback_wave_also_produces_spawns() {
        assert!(!build_wave(99, 1).is_empty());
    }

    #[test]
    fn wave_3_drops_a_power() {
        // 第 4 波（idx=3）的第二架轰炸机会掉道具
        let spawns = build_wave(3, 1);
        assert!(spawns.iter().any(|s| s.drops_power));
    }

    #[test]
    fn delays_are_non_negative() {
        for idx in 0..8 {
            for s in build_wave(idx, 1) {
                assert!(s.delay >= 0.0);
            }
        }
    }
}
