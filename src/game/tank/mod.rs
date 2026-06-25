//! 坦克大战：模块入口。
//!
//! - `components`：所有 Component 与方向枚举
//! - `constants`：地图布局、速度、Z 序等常量
//! - `geometry`：纯函数工具（坐标转换、AABB、轴吸附），含单元测试
//! - `resources`：`TankStage` 资源
//! - `setup`：关卡 / 实体生成
//! - `systems`：每帧驱动的 Bevy 系统

mod components;
mod constants;
mod geometry;
pub mod palette;
mod resources;
mod setup;
pub mod sprites;
mod systems;

pub(super) use resources::TankStage;
pub(super) use setup::setup_stage;
pub(super) use systems::{
    tank_bullet_update, tank_enemy_ai, tank_enemy_spawner, tank_freeze_tick, tank_hud_update,
    tank_lifetime_tick, tank_mode_select, tank_movement, tank_player_input, tank_player_respawn,
    tank_playing, tank_powerup_pickup, tank_spawn_effect,
};
