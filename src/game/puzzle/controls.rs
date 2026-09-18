//! 益智游戏共用的固定步输入缓冲。
//!
//! `ActionState` 每帧在 `PreUpdate` 刷新，而玩法跑在 `FixedUpdate`（一帧可能
//! 0 或 N 个 tick）。直接读 `just_pressed` 会丢边沿或重复消费，所以先在
//! `PreUpdate` 锁存到这里，玩法系统再 `take_*` 消费一次。
//!
//! 八款益智游戏共用同一个资源：同一时刻只有一个游戏在跑，资源由各自的
//! `setup_stage` 插入、`cleanup_stage_resources` 移除。

use bevy::prelude::*;

use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};
use crate::game::model::GameSession;

#[derive(Resource, Default)]
pub struct PuzzleControls {
    held_dir: Option<(i32, i32)>,
    buffered_dir: Option<(i32, i32)>,
    last_dir: Option<(i32, i32)>,
    primary: bool,
    secondary: bool,
    reset: bool,
}

impl PuzzleControls {
    pub fn sample(&mut self, actions: &ActionState) {
        // 网格 row 0 在顶部而世界 Y 向上，方向的 Y 轴翻转统一写在这里。
        for (action, direction) in [
            (InputAction::Left, (-1, 0)),
            (InputAction::Right, (1, 0)),
            (InputAction::Up, (0, -1)),
            (InputAction::Down, (0, 1)),
        ] {
            if actions.just_pressed(PlayerSlot::One, action) {
                self.buffered_dir = Some(direction);
                self.last_dir = Some(direction);
            }
        }
        self.held_dir = held_direction(actions.movement(PlayerSlot::One), self.last_dir);
        self.primary |= actions.just_pressed(PlayerSlot::One, InputAction::Primary);
        self.secondary |= actions.just_pressed(PlayerSlot::One, InputAction::Secondary);
        self.reset |= actions.just_pressed(PlayerSlot::One, InputAction::Reset);
    }

    pub fn take_dir(&mut self) -> Option<(i32, i32)> {
        self.buffered_dir.take()
    }

    pub fn held_dir(&self) -> Option<(i32, i32)> {
        self.held_dir
    }

    pub fn take_primary(&mut self) -> bool {
        std::mem::take(&mut self.primary)
    }

    pub fn take_secondary(&mut self) -> bool {
        std::mem::take(&mut self.secondary)
    }

    pub fn take_reset(&mut self) -> bool {
        std::mem::take(&mut self.reset)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// 斜向输入消歧：两轴同时按下时优先保留最后一次单独按下的那一轴。
fn held_direction(movement: Vec2, last: Option<(i32, i32)>) -> Option<(i32, i32)> {
    if movement == Vec2::ZERO {
        return None;
    }
    if movement.x != 0.0
        && movement.y != 0.0
        && let Some(last) = last
        && ((last.0 != 0 && movement.x.signum() == last.0 as f32)
            || (last.1 != 0 && movement.y.signum() == -last.1 as f32))
    {
        return Some(last);
    }
    if movement.x != 0.0 {
        Some((movement.x.signum() as i32, 0))
    } else {
        Some((0, -movement.y.signum() as i32))
    }
}

/// 八款益智游戏共用的采样系统：`PuzzleControls` 存在即说明有益智关卡在跑。
///
/// 可见性跟着参数里的 `GameSession` 走，否则会漏出一个外部根本调不了的签名。
pub(in crate::game) fn puzzle_sample_input(
    actions: Res<ActionState>,
    session: Res<GameSession>,
    mut controls: ResMut<PuzzleControls>,
) {
    if session.paused || session.finished {
        controls.clear();
        return;
    }
    controls.sample(&actions);
}
