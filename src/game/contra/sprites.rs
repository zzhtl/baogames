//! 魂斗罗精灵的共享定义（游戏渲染 + 离线预览同源）。
//!
//! 这些 `SpriteDef` 既被 `setup_actors.rs` 通过 `spawn_sprite_def` 渲染进游戏，
//! 也被 `cargo run --bin preview -- contra <name>` 离线光栅化成 PNG。改这里一处，
//! 游戏与预览同时变。网格一律**按朝右**画，朝向靠 `transform.scale.x = ±1` 翻转。

use bevy::prelude::Vec2;

use crate::common::sprite_def::SpriteDef;
use crate::parts;

use super::constants::{
    BOSS_H, BOSS_W, ENEMY_H, ENEMY_W, FALCON_H, FALCON_W, PLAYER_H, PLAYER_W, PRONE_H,
};
use super::palette::*;

/// 主角 Bill：经典裸上身丛林兵，立姿持枪向右。32 高、18 宽包络。
/// 棕发圆头、4 阶肤色明暗的肌肉躯干（胸肌/腹肌分块）、双手端步枪、宽马步蓝裤、敦实战靴。
pub const PLAYER_BILL: SpriteDef = SpriteDef {
    size: Vec2::new(PLAYER_W, PLAYER_H),
    parts: parts![
        // ===== 头部（棕发圆顶 + 侧脸朝右，无帽）=====
        (-0.5, 13.6, 7.5, 2.0, COLOR_PLAYER_HAIR, 0.06),   // 头发顶
        (-3.5, 13.0, 2.0, 2.0, COLOR_PLAYER_HAIR, 0.06),   // 顶左圆角
        (3.0, 13.0, 2.0, 1.0, COLOR_PLAYER_HAIR, 0.06),    // 顶右圆角
        (-4.2, 11.5, 2.0, 4.0, COLOR_PLAYER_HAIR, 0.06),   // 后脑发
        (3.6, 12.6, 3.0, 1.0, COLOR_PLAYER_HAIR, 0.078),   // 额前刘海
        (0.5, 10.4, 8.0, 5.0, COLOR_PLAYER_SKIN, 0.062),   // 脸
        (-1.5, 11.0, 3.0, 3.0, COLOR_PLAYER_SKIN_HI, 0.066), // 颊受光
        (3.2, 9.6, 3.0, 2.0, COLOR_PLAYER_SKIN_DK, 0.066), // 后颊/腮阴影
        (4.6, 10.2, 1.0, 2.0, COLOR_PLAYER_SKIN_DK, 0.07), // 鼻影
        (1.0, 8.4, 6.0, 1.0, COLOR_PLAYER_SKIN_DK, 0.07),  // 下颌阴影
        (3.0, 11.0, 1.5, 1.6, COLOR_PLAYER_OUTLINE, 0.082), // 眼
        (3.2, 12.2, 3.0, 0.8, COLOR_PLAYER_HAIR, 0.08),    // 眉
        (0.0, 7.2, 4.0, 1.6, COLOR_PLAYER_SKIN, 0.062),    // 颈
        (0.0, 6.5, 4.0, 0.8, COLOR_PLAYER_SKIN_DK, 0.066), // 颈下阴影
        // ===== 裸上身躯干（高光/基础/阴影/深阴影 4 阶 + 胸肌腹肌分块）=====
        (0.0, 2.8, 11.0, 8.4, COLOR_PLAYER_SKIN, 0.06),    // 躯干主体
        (-3.4, 4.6, 4.0, 4.2, COLOR_PLAYER_SKIN_HI, 0.066), // 左胸受光
        (-3.2, 2.6, 3.4, 2.0, COLOR_PLAYER_SKIN_HI, 0.066), // 左腹受光
        (0.0, 4.2, 9.0, 0.9, COLOR_PLAYER_SKIN_DK, 0.07),  // 胸肌下缘
        (0.0, 4.6, 0.9, 4.0, COLOR_PLAYER_SKIN_DK, 0.07),  // 胸骨中线
        (4.2, 2.4, 3.2, 6.6, COLOR_PLAYER_SKIN_SH, 0.067), // 右侧深阴影
        (2.6, 2.4, 2.0, 6.6, COLOR_PLAYER_SKIN_DK, 0.069), // 右侧中阴影
        (-1.6, 1.2, 4.4, 0.8, COLOR_PLAYER_SKIN_DK, 0.072), // 腹肌分界 1
        (-1.6, -0.4, 4.4, 0.8, COLOR_PLAYER_SKIN_DK, 0.072), // 腹肌分界 2
        (0.0, -1.4, 10.0, 1.0, COLOR_PLAYER_SKIN_SH, 0.07), // 腰部深阴影
        // ===== 手臂（体积感：高光 + 阴影 + 深阴影）=====
        (-6.0, 3.6, 3.2, 6.2, COLOR_PLAYER_SKIN, 0.064),   // 远(左)臂上举
        (-6.9, 5.2, 1.3, 3.6, COLOR_PLAYER_SKIN_HI, 0.07), // 左臂受光边
        (-5.2, 2.4, 1.4, 5.0, COLOR_PLAYER_SKIN_DK, 0.068), // 左臂内阴影
        (-6.0, 1.0, 3.2, 1.6, COLOR_PLAYER_SKIN_SH, 0.069), // 左肘深阴影
        (6.0, 2.6, 3.6, 3.2, COLOR_PLAYER_SKIN, 0.085),    // 近(右)前臂端枪
        (7.0, 2.0, 1.6, 3.0, COLOR_PLAYER_SKIN_DK, 0.087), // 右前臂阴影
        // ===== 步枪（朝右，双手端持，加暗部立体）=====
        (7.8, 2.6, 3.0, 4.0, COLOR_PLAYER_GUN, 0.075),     // 枪托
        (12.0, 3.0, 7.0, 2.2, COLOR_PLAYER_GUN, 0.10),     // 枪身
        (12.0, 3.8, 7.0, 0.7, COLOR_PLAYER_GUN_HI, 0.105), // 枪身高光
        (12.0, 2.2, 7.0, 0.6, COLOR_PLAYER_OUTLINE, 0.105), // 枪身暗
        (10.5, 1.3, 1.5, 2.5, COLOR_PLAYER_GUN, 0.10),     // 弹匣
        (17.2, 3.2, 6.0, 1.3, COLOR_PLAYER_GUN, 0.10),     // 枪管
        (17.2, 3.6, 6.0, 0.5, COLOR_PLAYER_GUN_HI, 0.103), // 枪管高光
        (20.6, 3.2, 1.5, 1.0, COLOR_PLAYER_OUTLINE, 0.105), // 枪口
        (10.2, 2.6, 2.2, 2.2, COLOR_PLAYER_SKIN, 0.11),    // 前手扶护木
        // ===== 蓝裤（宽马步，4 阶明暗）=====
        (0.0, -1.4, 11.0, 1.8, COLOR_PLAYER_BOOT, 0.08),   // 腰带
        (0.0, -2.2, 11.0, 0.8, COLOR_PLAYER_OUTLINE, 0.085),
        (-3.6, -6.6, 5.0, 9.0, COLOR_PLAYER_PANTS, 0.06),  // 后(左)腿
        (3.6, -6.6, 5.0, 9.0, COLOR_PLAYER_PANTS, 0.06),   // 前(右)腿
        (-5.0, -6.2, 1.6, 8.0, COLOR_PLAYER_PANTS_HI, 0.07), // 左腿受光
        (2.2, -5.6, 1.4, 7.0, COLOR_PLAYER_PANTS_HI, 0.068), // 右腿受光
        (-1.6, -7.0, 1.5, 8.2, COLOR_PLAYER_PANTS_DK, 0.07), // 后腿内阴影
        (5.4, -7.0, 1.5, 8.2, COLOR_PLAYER_PANTS_DK, 0.07),  // 前腿外阴影
        (-3.6, -4.8, 4.2, 1.0, COLOR_PLAYER_PANTS_DK, 0.072), // 膝
        (3.6, -4.8, 4.2, 1.0, COLOR_PLAYER_PANTS_DK, 0.072),
        (-3.6, -10.5, 4.6, 1.0, COLOR_PLAYER_PANTS_DK, 0.072), // 裤脚阴影
        (3.6, -10.5, 4.6, 1.0, COLOR_PLAYER_PANTS_DK, 0.072),
        // ===== 战靴（敦实，加鞋面高光）=====
        (-3.6, -13.0, 6.0, 3.6, COLOR_PLAYER_BOOT, 0.06),
        (3.6, -13.0, 6.0, 3.6, COLOR_PLAYER_BOOT, 0.06),
        (-5.2, -12.4, 1.6, 1.4, COLOR_PLAYER_PANTS_HI, 0.066), // 靴面高光
        (2.0, -12.4, 1.6, 1.4, COLOR_PLAYER_PANTS_HI, 0.066),
        // ===== 黑色轮廓描边（最底层 +0.04，圆角）=====
        (-0.5, 15.0, 7.5, 1.0, COLOR_PLAYER_OUTLINE, 0.04), // 头顶
        (-3.5, 14.4, 2.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04), // 顶左圆角
        (3.0, 14.4, 2.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04),  // 顶右圆角
        (-5.4, 11.0, 1.0, 5.0, COLOR_PLAYER_OUTLINE, 0.04), // 后脑
        (5.0, 10.0, 1.0, 5.0, COLOR_PLAYER_OUTLINE, 0.04),  // 脸前
        (6.2, 3.2, 1.0, 7.4, COLOR_PLAYER_OUTLINE, 0.04),   // 躯干右
        (-7.8, 3.6, 1.0, 6.2, COLOR_PLAYER_OUTLINE, 0.04),  // 左臂外
        (-6.4, -6.6, 1.0, 9.0, COLOR_PLAYER_OUTLINE, 0.04), // 后腿外
        (6.4, -6.6, 1.0, 9.0, COLOR_PLAYER_OUTLINE, 0.04),  // 前腿外
        (0.0, -7.0, 1.2, 8.4, COLOR_PLAYER_OUTLINE, 0.045), // 两腿间隙
        (-3.6, -15.1, 6.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04), // 后靴底
        (3.6, -15.1, 6.0, 1.0, COLOR_PLAYER_OUTLINE, 0.04),  // 前靴底
    ],
};

/// Bill 卧倒射击：身体贴地、双腿后伸，枪口仍保持清晰的朝右轮廓。
pub const PLAYER_PRONE: SpriteDef = SpriteDef {
    size: Vec2::new(PLAYER_W, PRONE_H),
    parts: parts![
        // 黑色底轮廓
        (-7.0, -1.5, 13.0, 6.0, COLOR_PLAYER_OUTLINE, 0.04),
        (0.0, 0.0, 14.0, 8.0, COLOR_PLAYER_OUTLINE, 0.04),
        (6.0, 1.5, 8.0, 9.0, COLOR_PLAYER_OUTLINE, 0.04),
        (14.0, 0.2, 17.0, 3.5, COLOR_PLAYER_OUTLINE, 0.04),
        // 后伸的腿与战靴
        (-11.0, -1.8, 7.0, 4.0, COLOR_PLAYER_BOOT, 0.06),
        (-7.0, -0.8, 9.0, 5.0, COLOR_PLAYER_PANTS_DK, 0.061),
        (-5.0, 0.2, 8.0, 4.0, COLOR_PLAYER_PANTS, 0.064),
        (-6.0, 1.3, 5.0, 1.2, COLOR_PLAYER_PANTS_HI, 0.068),
        // 贴地躯干
        (-0.5, 0.5, 11.0, 7.0, COLOR_PLAYER_SKIN_SH, 0.06),
        (-1.5, 1.7, 8.0, 4.5, COLOR_PLAYER_SKIN, 0.066),
        (-3.0, 2.7, 4.0, 1.2, COLOR_PLAYER_SKIN_HI, 0.07),
        (0.5, -1.8, 9.0, 1.2, COLOR_PLAYER_SKIN_DK, 0.07),
        // 侧脸与头发
        (5.0, 1.8, 7.0, 6.0, COLOR_PLAYER_SKIN, 0.067),
        (3.8, 4.4, 7.0, 2.0, COLOR_PLAYER_HAIR, 0.074),
        (2.0, 3.2, 2.0, 3.0, COLOR_PLAYER_HAIR, 0.074),
        (7.8, 2.2, 1.3, 1.3, COLOR_PLAYER_OUTLINE, 0.082),
        (7.8, 0.1, 3.0, 1.0, COLOR_PLAYER_SKIN_DK, 0.077),
        // 前臂与水平步枪
        (7.0, -1.0, 7.0, 3.0, COLOR_PLAYER_SKIN, 0.086),
        (11.5, 0.2, 8.0, 2.6, COLOR_PLAYER_GUN, 0.096),
        (12.0, 1.0, 7.0, 0.7, COLOR_PLAYER_GUN_HI, 0.101),
        (18.0, 0.2, 8.0, 1.5, COLOR_PLAYER_GUN, 0.10),
        (22.2, 0.2, 1.5, 1.2, COLOR_PLAYER_OUTLINE, 0.105),
        (10.0, -0.8, 2.3, 2.3, COLOR_PLAYER_SKIN_HI, 0.108),
    ],
};

/// Bill 跳跃翻滚：收腿成团的经典剪影，运行时旋转父节点形成翻滚动画。
pub const PLAYER_FLIP: SpriteDef = SpriteDef {
    size: Vec2::new(PLAYER_W, PLAYER_H),
    parts: parts![
        // 圆形黑色外轮廓
        (0.0, 0.0, 18.0, 20.0, COLOR_PLAYER_OUTLINE, 0.04),
        (-6.5, 0.0, 6.0, 13.0, COLOR_PLAYER_OUTLINE, 0.041),
        (6.5, 0.0, 6.0, 13.0, COLOR_PLAYER_OUTLINE, 0.041),
        // 蜷起的蓝裤与双腿
        (-4.0, -4.0, 7.0, 9.0, COLOR_PLAYER_PANTS_DK, 0.06),
        (2.0, -6.0, 9.0, 6.0, COLOR_PLAYER_PANTS, 0.064),
        (4.0, -4.8, 4.0, 2.0, COLOR_PLAYER_PANTS_HI, 0.069),
        (6.0, -1.0, 5.0, 5.0, COLOR_PLAYER_BOOT, 0.067),
        (-6.0, -5.0, 5.0, 4.0, COLOR_PLAYER_BOOT, 0.067),
        // 收紧的裸上身
        (-2.0, 1.0, 11.0, 10.0, COLOR_PLAYER_SKIN_SH, 0.06),
        (-3.0, 2.5, 7.0, 7.0, COLOR_PLAYER_SKIN, 0.066),
        (-4.5, 4.0, 3.0, 4.0, COLOR_PLAYER_SKIN_HI, 0.071),
        (0.5, 0.5, 2.0, 8.0, COLOR_PLAYER_SKIN_DK, 0.072),
        // 侧脸和头发
        (3.5, 5.0, 7.0, 6.0, COLOR_PLAYER_SKIN, 0.068),
        (2.0, 8.0, 8.0, 2.5, COLOR_PLAYER_HAIR, 0.075),
        (0.0, 6.5, 2.0, 4.0, COLOR_PLAYER_HAIR, 0.075),
        (5.5, 5.8, 1.3, 1.3, COLOR_PLAYER_OUTLINE, 0.083),
        // 抱枪的双臂与短枪形
        (3.0, 0.5, 7.0, 3.0, COLOR_PLAYER_SKIN, 0.084),
        (6.5, 1.0, 8.0, 2.5, COLOR_PLAYER_GUN, 0.096),
        (8.0, 1.8, 6.0, 0.7, COLOR_PLAYER_GUN_HI, 0.101),
        (10.8, 1.0, 2.0, 1.2, COLOR_PLAYER_OUTLINE, 0.105),
    ],
};

/// 敌兵共享几何（4 阶明暗）：躯干主色 `$body` / 受光 `$hi` / 暗面 `$dark`，其余一致。
/// 经典 Contra 大兵：钢盔带檐、制服带肩章腰带、持枪向右。
macro_rules! enemy_def {
    ($body:expr, $hi:expr, $dark:expr) => {
        SpriteDef {
            size: Vec2::new(ENEMY_W, ENEMY_H),
            parts: parts![
                // 钢盔（帽体 + 高光 + 帽檐 + 帽下阴影）
                (0.0, 12.8, 11.0, 2.8, COLOR_ENEMY_HAT, 0.06),
                (-2.0, 13.3, 5.0, 1.3, COLOR_ENEMY_HAT_HI, 0.064),
                (0.0, 11.5, 13.0, 1.4, COLOR_ENEMY_OUTLINE, 0.066),
                (0.0, 10.3, 11.0, 0.9, COLOR_ENEMY_OUTLINE, 0.07),
                // 脸（受光 + 腮影 + 眼）
                (0.0, 7.6, 10.0, 3.6, COLOR_ENEMY_SKIN, 0.06),
                (-2.5, 8.2, 3.0, 2.0, COLOR_PLAYER_SKIN_HI, 0.064),
                (3.0, 6.9, 4.0, 1.4, COLOR_PLAYER_SKIN_DK, 0.065),
                (2.0, 8.2, 1.4, 1.4, COLOR_ENEMY_OUTLINE, 0.072),
                // 颈
                (-1.0, 5.0, 4.0, 1.4, COLOR_ENEMY_SKIN, 0.06),
                (-1.0, 4.4, 4.0, 0.7, COLOR_PLAYER_SKIN_DK, 0.064),
                // 躯干（4 阶：受光 / 主体 / 阴影 / 分界）
                (0.0, 1.5, 13.0, 7.0, $body, 0.06),
                (-3.5, 2.6, 4.0, 4.8, $hi, 0.066),
                (3.6, 0.8, 4.0, 5.4, $dark, 0.067),
                (0.0, -0.2, 11.0, 0.8, $dark, 0.07),
                // 肩章
                (-5.0, 4.6, 3.0, 1.3, COLOR_ENEMY_HAT, 0.08),
                (5.0, 4.6, 3.0, 1.3, COLOR_ENEMY_HAT, 0.08),
                // 腰带
                (0.0, -1.9, 13.0, 1.6, COLOR_PLAYER_BOOT, 0.08),
                (0.0, -2.6, 13.0, 0.7, COLOR_ENEMY_OUTLINE, 0.085),
                // 腿（4 阶）
                (-3.0, -6.5, 5.0, 7.0, COLOR_ENEMY_PANTS, 0.06),
                (3.0, -6.5, 5.0, 7.0, COLOR_ENEMY_PANTS, 0.06),
                (-4.5, -6.0, 1.4, 6.5, COLOR_ENEMY_PANTS_HI, 0.07),
                (-1.5, -6.5, 1.3, 7.0, COLOR_ENEMY_PANTS_DK, 0.07),
                (4.6, -6.5, 1.4, 7.0, COLOR_ENEMY_PANTS_DK, 0.07),
                (-3.0, -5.0, 4.0, 0.8, COLOR_ENEMY_PANTS_DK, 0.072),
                (3.0, -5.0, 4.0, 0.8, COLOR_ENEMY_PANTS_DK, 0.072),
                // 靴
                (-3.0, -12.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
                (3.0, -12.0, 5.0, 3.0, COLOR_PLAYER_BOOT, 0.06),
                (-3.0, -13.4, 5.0, 1.0, COLOR_ENEMY_OUTLINE, 0.065),
                (3.0, -13.4, 5.0, 1.0, COLOR_ENEMY_OUTLINE, 0.065),
                // 手 + 步枪（高光）
                (6.5, 2.0, 3.0, 3.0, COLOR_ENEMY_SKIN, 0.085),
                (10.0, 2.0, 8.0, 2.0, COLOR_ENEMY_GUN, 0.10),
                (10.0, 2.6, 8.0, 0.7, COLOR_PLAYER_GUN_HI, 0.105),
                (15.2, 2.0, 1.4, 1.0, COLOR_ENEMY_OUTLINE, 0.11),
                // 描边
                (0.0, 14.4, 11.0, 1.0, COLOR_ENEMY_OUTLINE, 0.04),
                (-6.7, 2.0, 1.0, 8.0, COLOR_ENEMY_OUTLINE, 0.04),
                (6.7, 2.0, 1.0, 8.0, COLOR_ENEMY_OUTLINE, 0.04),
                (-5.7, -6.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
                (5.7, -6.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.04),
                (0.0, -6.5, 1.0, 7.0, COLOR_ENEMY_OUTLINE, 0.045),
            ],
        }
    };
}

/// 兵卒：红制服。
pub const ENEMY_SOLDIER: SpriteDef =
    enemy_def!(COLOR_ENEMY_BODY, COLOR_ENEMY_BODY_HI, COLOR_ENEMY_BODY_DK);
/// 狙手：深红制服。
pub const ENEMY_SNIPER: SpriteDef =
    enemy_def!(COLOR_ENEMY_RED, COLOR_ENEMY_RED_HI, COLOR_ENEMY_RED_DK);
/// 跳兵：蓝制服。
pub const ENEMY_JUMPER: SpriteDef =
    enemy_def!(COLOR_ENEMY_BLUE, COLOR_ENEMY_BLUE_HI, COLOR_ENEMY_BLUE_DK);
/// 装甲兵：灰色护甲。
pub const ENEMY_HEAVY: SpriteDef =
    enemy_def!(COLOR_ENEMY_GRAY, COLOR_ENEMY_GRAY_HI, COLOR_ENEMY_GRAY_DK);
/// 机枪手：迷彩绿。
pub const ENEMY_GUNNER: SpriteDef =
    enemy_def!(COLOR_ENEMY_GREEN, COLOR_ENEMY_GREEN_HI, COLOR_ENEMY_GREEN_DK);

/// 空投飞鹰：展翅、棕喙、白高光（武器色圆点由游戏单独叠加）。32×18。
pub const FALCON: SpriteDef = SpriteDef {
    size: Vec2::new(FALCON_W, FALCON_H),
    parts: parts![
        // 展开双翼
        (-2.0, 6.0, 18.0, 4.0, COLOR_FALCON_DARK, 0.05),
        (-4.0, 8.0, 10.0, 2.5, COLOR_FALCON, 0.052),
        (-2.0, -6.0, 18.0, 4.0, COLOR_FALCON_DARK, 0.05),
        (-4.0, -8.0, 10.0, 2.5, COLOR_FALCON, 0.052),
        // 身体
        (0.0, 0.0, 14.0, 8.0, COLOR_FALCON, 0.055),
        (-1.0, 2.0, 11.0, 2.0, COLOR_FALCON_HI, 0.058),
        (0.0, -2.5, 14.0, 2.5, COLOR_FALCON_DARK, 0.057),
        // 尾（左）
        (-12.0, 0.0, 8.0, 5.0, COLOR_FALCON_DARK, 0.05),
        (-15.0, 0.0, 3.0, 3.0, COLOR_FALCON_DARK, 0.05),
        // 头（右）+ 眼 + 喙
        (9.0, 1.5, 6.0, 6.0, COLOR_FALCON, 0.06),
        (9.0, 3.0, 4.0, 2.0, COLOR_FALCON_HI, 0.062),
        (11.0, 2.5, 1.6, 1.6, COLOR_PLAYER_OUTLINE, 0.07),
        (14.0, 0.5, 4.0, 2.5, COLOR_FALCON_BEAK, 0.06),
        // 爪
        (3.0, -5.0, 3.0, 2.0, COLOR_FALCON_BEAK, 0.06),
    ],
};

/// 关底 Boss 机械要塞：装甲墙（受光/阴影）+ 警示条 + 装甲缝 + 角铆钉 +
/// 中央能量核（弱点，多环 + 十字光）+ 左右炮塔基座（turret 实体盖其上）。220×240。
pub const BOSS_BODY: SpriteDef = SpriteDef {
    size: Vec2::new(BOSS_W, BOSS_H),
    parts: parts![
        (0.0, 0.0, 220.0, 240.0, COLOR_BOSS_WALL, 0.0),
        (-92.0, 0.0, 30.0, 240.0, COLOR_BOSS_WALL_HI, 0.01),
        (95.0, 0.0, 26.0, 240.0, COLOR_BOSS_WALL_DARK, 0.01),
        (0.0, -104.0, 220.0, 26.0, COLOR_BOSS_WALL_DARK, 0.012),
        (0.0, 110.0, 220.0, 18.0, COLOR_BOSS_WALL_DARK, 0.02),
        (0.0, -114.0, 220.0, 12.0, COLOR_BOSS_PANEL, 0.02),
        (0.0, 96.0, 180.0, 7.0, COLOR_BOSS_WARN, 0.03),
        (0.0, 95.0, 180.0, 2.0, COLOR_BOSS_PANEL, 0.033),
        (-55.0, 0.0, 2.5, 220.0, COLOR_BOSS_PANEL, 0.02),
        (55.0, 0.0, 2.5, 220.0, COLOR_BOSS_PANEL, 0.02),
        (0.0, 55.0, 220.0, 2.5, COLOR_BOSS_PANEL, 0.02),
        (0.0, -55.0, 220.0, 2.5, COLOR_BOSS_PANEL, 0.02),
        (-96.0, 104.0, 7.0, 7.0, COLOR_BOSS_RIVET, 0.04),
        (96.0, 104.0, 7.0, 7.0, COLOR_BOSS_RIVET, 0.04),
        (-96.0, -104.0, 7.0, 7.0, COLOR_BOSS_RIVET, 0.04),
        (96.0, -104.0, 7.0, 7.0, COLOR_BOSS_RIVET, 0.04),
        // 中央能量核（弱点）
        (0.0, 0.0, 88.0, 88.0, COLOR_BOSS_PANEL, 0.05),
        (0.0, 0.0, 76.0, 76.0, COLOR_BOSS_WALL_DARK, 0.052),
        (0.0, 0.0, 60.0, 60.0, COLOR_BOSS_TRIM, 0.054),
        (0.0, 0.0, 48.0, 48.0, COLOR_BOSS_CORE, 0.06),
        (0.0, 0.0, 32.0, 32.0, COLOR_BOSS_CORE_HI, 0.064),
        (0.0, 0.0, 16.0, 16.0, COLOR_BOSS_CORE_WHITE, 0.068),
        (0.0, 0.0, 6.0, 48.0, COLOR_BOSS_CORE_HI, 0.062),
        (0.0, 0.0, 48.0, 6.0, COLOR_BOSS_CORE_HI, 0.062),
        // 炮塔基座（上下两个炮口，turret 实体盖其上）
        (-24.0, 80.0, 50.0, 34.0, COLOR_BOSS_PANEL, 0.045),
        (-24.0, 80.0, 38.0, 22.0, COLOR_BOSS_WALL_DARK, 0.047),
        (-24.0, -80.0, 50.0, 34.0, COLOR_BOSS_PANEL, 0.045),
        (-24.0, -80.0, 38.0, 22.0, COLOR_BOSS_WALL_DARK, 0.047),
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_defs_are_well_formed() {
        PLAYER_BILL.check("PLAYER_BILL");
        PLAYER_PRONE.check("PLAYER_PRONE");
        PLAYER_FLIP.check("PLAYER_FLIP");
        ENEMY_SOLDIER.check("ENEMY_SOLDIER");
        ENEMY_SNIPER.check("ENEMY_SNIPER");
        ENEMY_JUMPER.check("ENEMY_JUMPER");
        ENEMY_HEAVY.check("ENEMY_HEAVY");
        ENEMY_GUNNER.check("ENEMY_GUNNER");
        FALCON.check("FALCON");
        BOSS_BODY.check("BOSS_BODY");
    }

    #[test]
    fn player_envelope_matches_collision_constants() {
        assert_eq!(PLAYER_BILL.size, Vec2::new(PLAYER_W, PLAYER_H));
        assert_eq!(PLAYER_PRONE.size, Vec2::new(PLAYER_W, PRONE_H));
        assert_eq!(PLAYER_FLIP.size, Vec2::new(PLAYER_W, PLAYER_H));
        assert_eq!(ENEMY_SOLDIER.size, Vec2::new(ENEMY_W, ENEMY_H));
    }
}
