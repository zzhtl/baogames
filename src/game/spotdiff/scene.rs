//! 图元场景的生成与差异注入。纯函数，差异数和可见性都在这里单测。

use rand::Rng;
use rand::seq::SliceRandom;

/// 一个图元：形状、色相、明暗档。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Motif {
    pub shape: u8,
    pub color: u8,
    pub shade: u8,
}

/// 改动的种类。第 6 关起只用 `Shade`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiffKind {
    Shape,
    Color,
    Shade,
}

/// 随机铺一张底图。
pub fn build_scene<R: Rng + ?Sized>(
    cells: usize,
    shapes: usize,
    colors: usize,
    rng: &mut R,
) -> Vec<Motif> {
    (0..cells)
        .map(|_| Motif {
            shape: rng.gen_range(0..shapes.max(1)) as u8,
            color: rng.gen_range(0..colors.max(1)) as u8,
            // 底图统一用亮档，细微差别才有地方可改。
            shade: 0,
        })
        .collect()
}

/// 复制底图并改掉**恰好** `count` 格。
///
/// 返回 `(右图, 被改格子的下标)`。`subtle` 为真时只改明暗，形状与色相保持不变。
/// 每一处都保证真的改动了（`Motif` 不相等），否则玩家会找到一个"看不出来的不同"。
pub fn apply_differences<R: Rng + ?Sized>(
    base: &[Motif],
    count: usize,
    subtle: bool,
    shapes: usize,
    colors: usize,
    rng: &mut R,
) -> (Vec<Motif>, Vec<usize>) {
    let mut right = base.to_vec();
    let mut order: Vec<usize> = (0..base.len()).collect();
    order.shuffle(rng);
    let count = count.min(base.len());
    let mut changed = Vec::with_capacity(count);
    for &index in order.iter().take(count) {
        let kinds: &[DiffKind] = if subtle {
            &[DiffKind::Shade]
        } else {
            &[DiffKind::Shape, DiffKind::Color]
        };
        let kind = kinds[rng.gen_range(0..kinds.len())];
        right[index] = mutate(base[index], kind, shapes, colors, rng);
        changed.push(index);
    }
    changed.sort_unstable();
    (right, changed)
}

fn mutate<R: Rng + ?Sized>(
    motif: Motif,
    kind: DiffKind,
    shapes: usize,
    colors: usize,
    rng: &mut R,
) -> Motif {
    let mut out = motif;
    match kind {
        DiffKind::Shape => {
            let span = shapes.max(2);
            out.shape = ((motif.shape as usize + rng.gen_range(1..span)) % span) as u8;
        }
        DiffKind::Color => {
            let span = colors.max(2);
            out.color = ((motif.color as usize + rng.gen_range(1..span)) % span) as u8;
        }
        DiffKind::Shade => out.shade = 1 - motif.shade,
    }
    out
}

/// 两张图实际有差别的格子。用来校验生成结果，也给结算文案兜底。
pub fn diff_indices(left: &[Motif], right: &[Motif]) -> Vec<usize> {
    left.iter()
        .zip(right.iter())
        .enumerate()
        .filter_map(|(index, (a, b))| (a != b).then_some(index))
        .collect()
}
