//! 游戏渲染与离线预览工具共享的精灵描述。
//!
//! 一个精灵 = 一组带 z 偏移的彩色矩形（中心锚点、相对父原点的局部坐标）。
//! 这份数据既被 [`crate::common::render::spawn_sprite_def`] 渲染进游戏，
//! 也被 `cargo run --bin preview` 离线光栅化成 PNG —— 两边读同一份 `const`，
//! 从根上杜绝"预览满意但抄进游戏走样"。

use bevy::prelude::{Color, Vec2};

/// 一块子矩形：相对父原点的局部坐标(中心锚点) + 颜色 + z 偏移。
///
/// 字段顺序刻意对齐历史上的 `(dx, dy, w, h, color, dz)` 元组，迁移零摩擦。
#[derive(Clone, Copy)]
pub struct Part {
    pub dx: f32,
    pub dy: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    /// z 偏移：叠加顺序，越大越靠上（画家算法）。
    pub dz: f32,
}

/// 一个完整精灵：包络尺寸(父实体 custom_size，决定碰撞/实机尺寸) + 子矩形数组。
pub struct SpriteDef {
    pub size: Vec2,
    pub parts: &'static [Part],
}

impl SpriteDef {
    /// 自检：`parts` 非空、每块宽高为正、且都落在 `size` 包络内（含少量外延容差，
    /// 给枪管/飘带这类合理外伸留余地）。供各游戏单元测试统一调用。
    pub fn check(&self, name: &str) {
        assert!(!self.parts.is_empty(), "{name}: parts 不能为空");
        let (hw, hh) = (self.size.x * 0.5, self.size.y * 0.5);
        // 外延容差：允许子矩形中心在包络外最多 size 的一半（武器/飘带常超出躯干包络）
        let (mx, my) = (self.size.x, self.size.y);
        for (i, p) in self.parts.iter().enumerate() {
            assert!(p.w > 0.0 && p.h > 0.0, "{name}: part #{i} 宽高必须为正");
            assert!(
                p.dx.abs() <= hw + mx && p.dy.abs() <= hh + my,
                "{name}: part #{i} 偏离包络过远 (dx={}, dy={})",
                p.dx,
                p.dy
            );
        }
    }
}

/// 把 `(dx, dy, w, h, color, dz)` 元组数组写成 `&'static [Part]`。
///
/// ```ignore
/// pub const BILL: SpriteDef = SpriteDef {
///     size: Vec2::new(18.0, 32.0),
///     parts: parts![ (0.0, 13.0, 14.0, 3.0, COLOR_HELMET, 0.06), /* ... */ ],
/// };
/// ```
#[macro_export]
macro_rules! parts {
    ($(($dx:expr, $dy:expr, $w:expr, $h:expr, $c:expr, $dz:expr)),* $(,)?) => {
        &[$($crate::common::sprite_def::Part {
            dx: $dx, dy: $dy, w: $w, h: $h, color: $c, dz: $dz,
        }),*]
    };
}
