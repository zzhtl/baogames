use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::game::model::AgeTier;

use super::constants::*;
use super::generator::{cell_to_tile, distances, generate, pick_key_tile, tile_dims};

fn floors(walls: &[bool]) -> usize {
    walls.iter().filter(|wall| !**wall).count()
}

/// 完美迷宫 = 一棵树：房间 c×r，可通行瓦片恰好 c·r 个房间 + (c·r−1) 条通道。
/// 数目对得上就说明既没有环、也没有断开的区域。
#[test]
fn generated_mazes_are_perfect() {
    let mut rng = StdRng::seed_from_u64(101);
    for (cw, ch) in [(2, 2), (4, 3), (7, 5), (10, 7)] {
        for _ in 0..5 {
            let walls = generate(cw, ch, &mut rng);
            let (w, h) = tile_dims(cw, ch);
            assert_eq!(walls.len(), w * h);
            assert_eq!(
                floors(&walls),
                2 * cw * ch - 1,
                "{cw}×{ch} 的可通行瓦片数不对，说明有环或有断开的区域"
            );
        }
    }
}

#[test]
fn every_room_is_reachable_from_the_entrance() {
    let mut rng = StdRng::seed_from_u64(103);
    for (cw, ch) in [(4, 3), (8, 6), (10, 7)] {
        let walls = generate(cw, ch, &mut rng);
        let (w, h) = tile_dims(cw, ch);
        let dist = distances(&walls, w, h, cell_to_tile(0, 0));
        let reached = dist.iter().filter(|d| d.is_some()).count();
        assert_eq!(reached, floors(&walls), "{cw}×{ch} 有走不到的地面");
        let exit = cell_to_tile(cw - 1, ch - 1);
        assert!(dist[exit.1 * w + exit.0].is_some(), "{cw}×{ch} 走不到出口");
    }
}

#[test]
fn the_outer_border_is_always_solid() {
    let mut rng = StdRng::seed_from_u64(107);
    let (cw, ch) = (6, 4);
    let walls = generate(cw, ch, &mut rng);
    let (w, h) = tile_dims(cw, ch);
    for x in 0..w {
        assert!(walls[x], "上边界破了");
        assert!(walls[(h - 1) * w + x], "下边界破了");
    }
    for y in 0..h {
        assert!(walls[y * w], "左边界破了");
        assert!(walls[y * w + w - 1], "右边界破了");
    }
}

/// 钥匙不能顺路就捡到：它到起点、到出口的距离都要有一定量。
#[test]
fn the_key_is_placed_away_from_both_ends() {
    let mut rng = StdRng::seed_from_u64(109);
    for (cw, ch) in [(6, 5), (8, 6), (10, 7)] {
        let walls = generate(cw, ch, &mut rng);
        let (w, h) = tile_dims(cw, ch);
        let start = cell_to_tile(0, 0);
        let exit = cell_to_tile(cw - 1, ch - 1);
        let key = pick_key_tile(&walls, w, h, start, exit).expect("应该能放下钥匙");
        assert_ne!(key, start);
        assert_ne!(key, exit);
        assert!(!walls[key.1 * w + key.0], "钥匙掉进墙里了");
        let from_start = distances(&walls, w, h, start)[key.1 * w + key.0].unwrap();
        let from_exit = distances(&walls, w, h, exit)[key.1 * w + key.0].unwrap();
        assert!(from_start >= 4 && from_exit >= 4, "钥匙离两端太近：{from_start}/{from_exit}");
    }
}

#[test]
fn junior_tier_caps_the_maze_and_never_uses_fog() {
    for level in 1..=10u8 {
        let (w, h) = cells_for(level, AgeTier::Junior);
        assert!(w <= JUNIOR_MAX_CELLS.0 && h <= JUNIOR_MAX_CELLS.1);
        assert_eq!(fog_radius(level, AgeTier::Junior), None, "低龄档不该开雾");
    }
    assert_eq!(fog_radius(6, AgeTier::Senior), None);
    assert_eq!(fog_radius(7, AgeTier::Senior), Some(FOG_RADIUS));
    assert!(!needs_key(3));
    assert!(needs_key(4));
}

/// 限时必须够走完：用 BFS 的最短路当下界，留出三倍余量给找路的过程。
#[test]
fn every_level_can_be_finished_within_its_time_budget() {
    let mut rng = StdRng::seed_from_u64(113);
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cw, ch) = cells_for(level, age);
            let walls = generate(cw, ch, &mut rng);
            let (w, h) = tile_dims(cw, ch);
            let start = cell_to_tile(0, 0);
            let exit = cell_to_tile(cw - 1, ch - 1);
            let shortest = distances(&walls, w, h, start)[exit.1 * w + exit.0].unwrap();
            // 按长按连走的节奏折算走完最短路要多久
            let walk = shortest as f32 * MOVE_COOLDOWN;
            assert!(
                time_for(level, age) > walk * 3.0,
                "第 {level} 关 {age:?} 档时间不够：限时 {:.0}s，最短路要 {walk:.0}s",
                time_for(level, age)
            );
        }
    }
}

#[test]
fn junior_tier_gets_more_time_per_room() {
    for level in 1..=10u8 {
        let (jw, jh) = cells_for(level, AgeTier::Junior);
        let (sw, sh) = cells_for(level, AgeTier::Senior);
        let junior = time_for(level, AgeTier::Junior) / (jw * jh) as f32;
        let senior = time_for(level, AgeTier::Senior) / (sw * sh) as f32;
        assert!(junior > senior * 1.4, "第 {level} 关低龄档不够宽松");
    }
}
