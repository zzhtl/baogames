use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::common::px::WORLD_PER_PX;
use crate::common::settings::DisplayMode;
use crate::game::model::AgeTier;

use super::constants::*;
use super::palette;
use super::scene::{apply_differences, build_scene, diff_indices};
use super::setup::{motif_color, panel_layouts};

const SHAPES: usize = 6;
const COLORS: usize = 6;

#[test]
fn palette_tables_line_up() {
    assert_eq!(palette::MOTIF_SHAPES.len(), SHAPES);
    assert_eq!(palette::MOTIF_COLORS.len(), COLORS);
    assert_eq!(palette::SHADE_SCALE.len(), 2);
    for shape in palette::MOTIF_SHAPES {
        assert!(!shape.is_empty(), "图元不能是空的");
        for (dx, dy, w, h) in shape.iter().copied() {
            // 全部零件都要落在一个格子里，否则会糊到邻格上
            assert!(dx.abs() + w * 0.5 <= 0.5 + 1e-6, "图元零件超出格宽");
            assert!(dy.abs() + h * 0.5 <= 0.5 + 1e-6, "图元零件超出格高");
        }
    }
}

/// 生成的两张图必须**恰好**差 N 处，多一处少一处进度条都会对不上。
#[test]
fn exactly_the_requested_number_of_cells_differ() {
    let mut rng = StdRng::seed_from_u64(401);
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cols, rows) = grid_for(level, age);
            let want = diffs_for(level, age);
            let left = build_scene((cols * rows) as usize, SHAPES, COLORS, &mut rng);
            let (right, changed) =
                apply_differences(&left, want, subtle_for(level, age), SHAPES, COLORS, &mut rng);
            assert_eq!(changed.len(), want, "第 {level} 关 {age:?} 档改动数不对");
            assert_eq!(
                diff_indices(&left, &right),
                changed,
                "第 {level} 关 {age:?} 档实际差异与记录对不上"
            );
        }
    }
}

/// 每一处改动都必须真的看得出来 —— 改了等于没改的"不同点"是找不到的。
#[test]
fn every_change_is_actually_visible() {
    let mut rng = StdRng::seed_from_u64(409);
    for subtle in [false, true] {
        for _ in 0..50 {
            let left = build_scene(36, SHAPES, COLORS, &mut rng);
            let (right, changed) = apply_differences(&left, 8, subtle, SHAPES, COLORS, &mut rng);
            for &index in &changed {
                assert_ne!(left[index], right[index], "第 {index} 格改了个寂寞");
                assert!(
                    left[index].shape != right[index].shape
                        || motif_color(left[index]) != motif_color(right[index]),
                    "第 {index} 格形状和最终颜色都没变，玩家不可能看出来"
                );
            }
            for (index, (a, b)) in left.iter().zip(right.iter()).enumerate() {
                if !changed.contains(&index) {
                    assert_eq!(a, b, "没被挑中的格子不该变");
                }
            }
        }
    }
}

/// 细微模式只准改明暗：形状和色相一动，就不是"细微"了。
#[test]
fn subtle_mode_only_changes_brightness() {
    let mut rng = StdRng::seed_from_u64(419);
    let left = build_scene(36, SHAPES, COLORS, &mut rng);
    let (right, changed) = apply_differences(&left, 8, true, SHAPES, COLORS, &mut rng);
    for &index in &changed {
        assert_eq!(left[index].shape, right[index].shape, "细微模式不该改形状");
        assert_eq!(left[index].color, right[index].color, "细微模式不该改色相");
        assert_ne!(left[index].shade, right[index].shade);
    }
}

#[test]
fn plain_mode_never_relies_on_brightness_alone() {
    let mut rng = StdRng::seed_from_u64(421);
    let left = build_scene(36, SHAPES, COLORS, &mut rng);
    let (right, changed) = apply_differences(&left, 8, false, SHAPES, COLORS, &mut rng);
    for &index in &changed {
        assert!(
            left[index].shape != right[index].shape || left[index].color != right[index].color,
            "普通模式必须改形状或色相"
        );
    }
}

/// 两张图并排必须留在 4:3 画布里，而且都落在整像素上。
#[test]
fn both_panels_fit_the_narrow_canvas_on_whole_pixels() {
    let half_visible = DisplayMode::Classic4x3.world_width() * 0.5;
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cols, rows) = grid_for(level, age);
            let layouts = panel_layouts(cols, rows);
            for layout in layouts {
                let half_w = layout.total_size().x * 0.5;
                let center_x = layout.origin.x + (layout.cols - 1) as f32 * layout.cell_size * 0.5;
                assert!(
                    center_x.abs() + half_w <= half_visible - crate::common::px::px(4.0),
                    "第 {level} 关 {cols}×{rows} 的图出屏了"
                );
                for value in [layout.origin.x, layout.origin.y + 10.0] {
                    let in_px = value / WORLD_PER_PX;
                    assert_eq!(in_px, in_px.round(), "面板落在半像素上");
                }
            }
            // 两张图不能叠在一起
            let gap = layouts[1].origin.x - layouts[0].origin.x - layouts[0].total_size().x;
            assert!(gap > 0.0, "两张图重叠了");
        }
    }
}

#[test]
fn junior_tier_has_fewer_and_coarser_differences() {
    for level in 1..=10u8 {
        let (w, h) = grid_for(level, AgeTier::Junior);
        assert!(w <= JUNIOR_MAX_GRID.0 && h <= JUNIOR_MAX_GRID.1);
        assert!(diffs_for(level, AgeTier::Junior) <= JUNIOR_MAX_DIFFS);
        assert!(!subtle_for(level, AgeTier::Junior), "低龄档不该用只改明暗的差别");
    }
    assert!(!subtle_for(5, AgeTier::Senior));
    assert!(subtle_for(6, AgeTier::Senior));
    assert_eq!(diffs_for(10, AgeTier::Senior), 8);
}

#[test]
fn junior_tier_gets_more_time_per_difference() {
    for level in 1..=10u8 {
        let junior = time_for(level, AgeTier::Junior) / diffs_for(level, AgeTier::Junior) as f32;
        let senior = time_for(level, AgeTier::Senior) / diffs_for(level, AgeTier::Senior) as f32;
        assert!(junior > senior * 1.4, "第 {level} 关低龄档不够宽松");
    }
}

/// 不同点不能多到超过格子数，否则永远找不完。
#[test]
fn differences_never_exceed_the_cell_count() {
    for level in 1..=10u8 {
        for age in [AgeTier::Junior, AgeTier::Senior] {
            let (cols, rows) = grid_for(level, age);
            assert!(diffs_for(level, age) <= (cols * rows) as usize);
            assert!(diffs_for(level, age) >= 3);
        }
    }
}
