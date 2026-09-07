//! 无头截图驱动（dev-only，需要 `--features devtools`）。
//!
//! 这台开发机没有显示器，而「界面好不好看」只能看图判断。所以用 Xvfb + lavapipe
//! 软件 Vulkan 把游戏真的跑起来，按脚本注入输入，再把画面存成 PNG。
//!
//! 两条原则：
//!
//! * **输入写 `ButtonInput<KeyCode>` 而不是 `ActionState`**。后者每帧被
//!   `update_action_state` 无条件覆写，且字段是私有的；写键盘则会走完整的
//!   bindings → ActionState 链路，边沿检测天然正确，还顺带验证了键位映射。
//! * **按帧计时，不按秒**。lavapipe 只有 5~15 FPS，用秒完全不可复现。
//!
//! 只抓窗口。画布纹理走 `Screenshot::image` 在 lavapipe 下读回来是全黑的，而窗口
//! 图本就是画布的整数倍最近邻放大（已验证 4×4 块内零差异），所以 `scripts/capture.sh`
//! 直接把它降采样回 240×180，拿到的就是逐像素精确的画布 —— 判描边是不是恰好 1 像素、
//! 字有没有掉行，看那张。

use bevy::app::AppExit;
use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::window::screenshot::{Screenshot, save_to_disk};
use std::path::PathBuf;

use crate::common::input::ActionInputSet;
use crate::common::settings::{InputAction, InputBindings, PlayerSlot};

use super::model::{AppState, GameKind, GameSession, SaveData, SelectedGame};

/// 截图落盘是异步的，拍完要留几帧给它写完。
const SHOT_SETTLE: u32 = 6;

enum Step {
    Press(InputAction),
    Release(InputAction),
    /// 等 n 帧。
    Wait(u32),
    /// 跳过菜单导航直接进关（菜单导航本身另有场景覆盖）。
    EnterGame(GameKind, u8),
    /// 直接写 `GameSession`，用于截结算画面。
    ForceResult { won: bool },
    /// 回菜单。`setup_menu` 会把页面 / 焦点复位，所以这也是场景之间的清场手段。
    ToMenu,
    Shot(&'static str),
}

#[derive(Default)]
struct Script(Vec<Step>);

impl Script {
    fn tap(mut self, action: InputAction) -> Self {
        self.0.push(Step::Press(action));
        self.0.push(Step::Wait(1));
        self.0.push(Step::Release(action));
        self.0.push(Step::Wait(2));
        self
    }

    fn wait(mut self, frames: u32) -> Self {
        self.0.push(Step::Wait(frames));
        self
    }

    fn enter(mut self, kind: GameKind, level: u8) -> Self {
        self.0.push(Step::EnterGame(kind, level));
        self.0.push(Step::Wait(2));
        self
    }

    fn result(mut self, won: bool) -> Self {
        self.0.push(Step::ForceResult { won });
        self.0.push(Step::Wait(4));
        self
    }

    fn goto_menu(mut self) -> Self {
        self.0.push(Step::ToMenu);
        self.0.push(Step::Wait(4));
        self
    }

    fn shot(mut self, name: &'static str) -> Self {
        self.0.push(Step::Shot(name));
        self
    }
}

/// 每个游戏进关后稳定下来需要的帧数（含开场演出）。
const SETTLE_IN_GAME: u32 = 45;

fn game_scene(kind: GameKind, name: &'static str) -> Script {
    Script::default()
        .goto_menu()
        .enter(kind, 1)
        .wait(SETTLE_IN_GAME)
        .shot(name)
}

fn script_for(scene: &str) -> Option<Script> {
    let s = match scene {
        "menu_library" => Script::default().goto_menu().wait(6).shot("menu_library"),
        // 动作一：卡带墙 → 选关焦点
        "menu_stage" => Script::default()
            .goto_menu()
            .wait(4)
            .tap(InputAction::Primary)
            .wait(4)
            .shot("menu_stage"),
        // 动作二：卡带墙 → 系统设置
        "menu_settings" => Script::default()
            .goto_menu()
            .wait(4)
            .tap(InputAction::Secondary)
            .wait(4)
            .shot("menu_settings"),
        // 设置页第 7 行（下标 6）是「按键配置」
        "menu_controls" => {
            let mut s = Script::default()
                .goto_menu()
                .wait(4)
                .tap(InputAction::Secondary)
                .wait(4);
            for _ in 0..6 {
                s = s.tap(InputAction::Down);
            }
            s.tap(InputAction::Primary).wait(4).shot("menu_controls")
        }
        "game_tank" => game_scene(GameKind::Tank, "game_tank"),
        "game_bomb_maze" => game_scene(GameKind::BombMaze, "game_bomb_maze"),
        "game_space_shooter" => game_scene(GameKind::SpaceShooter, "game_space_shooter"),
        "game_super_mario" => game_scene(GameKind::SuperMario, "game_super_mario"),
        "game_contra" => game_scene(GameKind::Contra, "game_contra"),
        "game_bubble" => game_scene(GameKind::BubbleBobble, "game_bubble"),
        "game_memory" => game_scene(GameKind::MemoryMatch, "game_memory"),
        "game_sokoban" => game_scene(GameKind::Sokoban, "game_sokoban"),
        "paused" => Script::default()
            .goto_menu()
            .enter(GameKind::Tank, 1)
            .wait(SETTLE_IN_GAME)
            .tap(InputAction::Start)
            .wait(6)
            .shot("paused"),
        "result_win" => Script::default()
            .goto_menu()
            .enter(GameKind::Tank, 1)
            .wait(20)
            .result(true)
            .wait(6)
            .shot("result_win"),
        "result_lose" => Script::default()
            .goto_menu()
            .enter(GameKind::Tank, 1)
            .wait(20)
            .result(false)
            .wait(6)
            .shot("result_lose"),
        // 回归用：结算瞬间连点开火键，覆盖层必须还在
        "result_spam" => {
            let mut s = Script::default()
                .goto_menu()
                .enter(GameKind::Tank, 1)
                .wait(20)
                .result(false);
            for _ in 0..15 {
                s = s.tap(InputAction::Primary);
            }
            s.shot("result_spam")
        }
        _ => return None,
    };
    Some(s)
}

const ALL_SCENES: [&str; 16] = [
    "menu_library",
    "menu_stage",
    "menu_settings",
    "menu_controls",
    "game_tank",
    "game_bomb_maze",
    "game_space_shooter",
    "game_super_mario",
    "game_contra",
    "game_bubble",
    "game_memory",
    "game_sokoban",
    "paused",
    "result_win",
    "result_lose",
    "result_spam",
];

#[derive(Resource)]
struct CaptureRunner {
    steps: Vec<Step>,
    cursor: usize,
    wait: u32,
    out: PathBuf,
}

/// 解析 `--capture <scene>` / `--out <dir>`；没有 `--capture` 就返回 None（正常游戏）。
pub(super) fn requested_scene() -> Option<(String, PathBuf)> {
    let args: Vec<String> = std::env::args().collect();
    let scene = args
        .iter()
        .position(|a| a == "--capture")
        .and_then(|i| args.get(i + 1))
        .cloned()?;
    let out = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("preview_out/capture"));
    Some((scene, out))
}

pub(super) struct CapturePlugin {
    pub(super) scene: String,
    pub(super) out: PathBuf,
}

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        let mut steps = Vec::new();
        if self.scene == "all" {
            for name in ALL_SCENES {
                let script = script_for(name).expect("ALL_SCENES 里的场景必须都有脚本");
                steps.extend(script.0);
            }
        } else {
            let script = script_for(&self.scene)
                .unwrap_or_else(|| panic!("未知场景 {}，可用：{ALL_SCENES:?}", self.scene));
            steps.extend(script.0);
        }
        std::fs::create_dir_all(&self.out).expect("截图输出目录必须可创建");
        app.insert_resource(CaptureRunner {
            steps,
            cursor: 0,
            wait: 0,
            out: self.out.clone(),
        })
        .add_systems(
            PreUpdate,
            drive_capture.after(InputSystems).before(ActionInputSet),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn drive_capture(
    mut commands: Commands,
    mut runner: ResMut<CaptureRunner>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    mut selected: ResMut<SelectedGame>,
    mut save: ResMut<SaveData>,
    mut session: Option<ResMut<GameSession>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if runner.wait > 0 {
        runner.wait -= 1;
        return;
    }
    let p1 = PlayerSlot::One.index();
    while runner.cursor < runner.steps.len() {
        let index = runner.cursor;
        runner.cursor += 1;
        match &runner.steps[index] {
            Step::Press(action) => {
                keys.press(bindings.0[p1].binding(*action).keyboard.key_code());
            }
            Step::Release(action) => {
                keys.release(bindings.0[p1].binding(*action).keyboard.key_code());
            }
            Step::Wait(frames) => {
                runner.wait = *frames;
                return;
            }
            Step::EnterGame(kind, level) => {
                let idx = kind.index();
                let level = (*level).clamp(1, kind.max_level());
                save.unlocked_levels[idx] = save.unlocked_levels[idx].max(level);
                save.selected_levels[idx] = level;
                selected.0 = *kind;
                next_state.set(AppState::Playing);
                return;
            }
            Step::ForceResult { won } => {
                if let Some(session) = session.as_mut() {
                    session.finished = true;
                    session.won = *won;
                }
                return;
            }
            Step::ToMenu => {
                next_state.set(AppState::Menu);
                return;
            }
            Step::Shot(name) => {
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(runner.out.join(format!("{name}_win.png"))));
                runner.wait = SHOT_SETTLE;
                return;
            }
        }
    }
    exit.write(AppExit::Success);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_listed_scene_has_a_script() {
        for name in ALL_SCENES {
            assert!(script_for(name).is_some(), "场景 {name} 没有脚本");
        }
    }

    #[test]
    fn every_scene_takes_exactly_one_shot() {
        for name in ALL_SCENES {
            let script = script_for(name).unwrap();
            let shots = script
                .0
                .iter()
                .filter(|s| matches!(s, Step::Shot(_)))
                .count();
            assert_eq!(shots, 1, "场景 {name} 的截图数应为 1");
        }
    }
}
