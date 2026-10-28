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
//! 只抓窗口。大厅与游戏都保留原生分辨率；游戏截图只裁掉黑边，不再降采样。
//! 模拟时间固定为每帧 1/60 秒，预览、倒计时与路线执行可复现。

use bevy::app::AppExit;
use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::window::screenshot::{Screenshot, save_to_disk};
use std::path::PathBuf;

use crate::common::input::ActionInputSet;
use crate::common::settings::{InputAction, InputBindings, PlayerSlot};
use crate::common::settings::DisplayMode;

use super::model::{AgeTier, AppState, GameKind, GameSession, SaveData, SelectedGame};

/// 截图落盘是异步的，拍完要留几帧给它写完。
const SHOT_SETTLE: u32 = 6;

enum Step {
    Press(InputAction),
    Release(InputAction),
    Click(Vec2),
    ClickWorld(Vec2),
    ReleaseMouse,
    Key(KeyCode, bool),
    CheckSelection(GameKind),
    CheckLevel(u8),
    CheckAge { toggled: bool },
    CheckState(AppState),
    CheckMusicIncrease,
    SetAge(AgeTier),
    SetDisplay(DisplayMode),
    SetDpi(Option<f32>),
    CheckResult { won: bool },
    CheckSessionLevel(u8),
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
    fn click_world(mut self, x: f32, y: f32) -> Self {
        self.0.extend([Step::ClickWorld(Vec2::new(x, y)), Step::Wait(1), Step::ReleaseMouse, Step::Wait(3)]);
        self
    }

    fn click(mut self, x: f32, y: f32) -> Self {
        self.0.extend([Step::Click(Vec2::new(x, y)), Step::Wait(1), Step::ReleaseMouse, Step::Wait(3)]);
        self
    }

    fn key(mut self, key: KeyCode) -> Self {
        self.0.extend([Step::Key(key, true), Step::Wait(1), Step::Key(key, false), Step::Wait(3)]);
        self
    }

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

fn training_scene(kind: GameKind, age: AgeTier, name: &'static str) -> Script {
    let mut script = Script::default().goto_menu();
    script.0.push(Step::SetAge(age));
    script.enter(kind, if age.is_junior() { 1 } else { 10 }).wait(6)
        .click_world(0.0, -120.0).wait(8).shot(name)
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
        "menu_puzzle" => {
            let mut s = Script::default().goto_menu().wait(6)
                .click(240.0, 218.0).click(120.0, 380.0);
            s.0.push(Step::CheckSelection(GameKind::Sudoku));
            s.shot("menu_puzzle")
        }
        "menu_mouse_stage" => {
            let mut s = Script::default().enter(GameKind::Tank, 3).goto_menu().wait(6)
                .click(1000.0, 438.0);
            s.0.push(Step::CheckLevel(2));
            s.shot("menu_mouse_stage")
        }
        "menu_mouse_age" => {
            let mut s = Script::default().goto_menu().wait(6).click(240.0, 218.0);
            // 切换两次必须回到原年龄档；不用依赖已有截图存档的年龄值。
            s = s.click(1050.0, 494.0);
            s.0.push(Step::CheckAge { toggled: true });
            s = s.click(1050.0, 494.0);
            s.0.push(Step::CheckAge { toggled: false });
            s.shot("menu_mouse_age")
        }
        "menu_mouse_play" => {
            let mut s = Script::default().goto_menu().wait(6).click(100.0, 218.0)
                .click(120.0, 380.0).click(1000.0, 566.0).wait(20);
            s.0.push(Step::CheckSelection(GameKind::SpaceShooter));
            s.0.push(Step::CheckState(AppState::Playing));
            s = s.tap(InputAction::Back).wait(6);
            s.0.push(Step::CheckState(AppState::Menu));
            s.shot("menu_mouse_play")
        }
        "menu_keyboard" => {
            let mut s = Script::default().goto_menu().wait(6).click(100.0, 218.0)
                .click(120.0, 280.0).key(KeyCode::Tab).key(KeyCode::ArrowRight);
            s.0.push(Step::CheckSelection(GameKind::Stroop));
            s.shot("menu_keyboard")
        }
        "menu_capture" => Script::default().goto_menu().wait(6)
            .click(1140.0, 44.0).click(600.0, 527.0).click(800.0, 178.0)
            .wait(4).shot("menu_capture"),
        "menu_mouse_settings" => {
            let mut s = Script::default().goto_menu().wait(6)
                .click(1140.0, 44.0).click(1194.0, 399.0);
            s.0.push(Step::CheckMusicIncrease);
            s.shot("menu_mouse_settings")
        }
        "game_tank" => game_scene(GameKind::Tank, "game_tank"),
        "game_bomb_maze" => game_scene(GameKind::BombMaze, "game_bomb_maze"),
        "game_space_shooter" => game_scene(GameKind::SpaceShooter, "game_space_shooter"),
        "game_super_mario" => game_scene(GameKind::SuperMario, "game_super_mario"),
        "game_contra" => game_scene(GameKind::Contra, "game_contra"),
        "game_bubble" => game_scene(GameKind::BubbleBobble, "game_bubble"),
        "game_memory" => game_scene(GameKind::MemoryMatch, "game_memory"),
        "game_sokoban" => game_scene(GameKind::Sokoban, "game_sokoban"),
        "game_schulte" => game_scene(GameKind::Schulte, "game_schulte"),
        "game_stroop" => game_scene(GameKind::Stroop, "game_stroop"),
        "game_sudoku" => game_scene(GameKind::Sudoku, "game_sudoku"),
        "game_sliding" => game_scene(GameKind::Sliding, "game_sliding"),
        "game_maze" => game_scene(GameKind::MazeRun, "game_maze"),
        "game_linkup" => game_scene(GameKind::LinkUp, "game_linkup"),
        "game_simon" => game_scene(GameKind::Simon, "game_simon"),
        "game_spotdiff" => game_scene(GameKind::SpotDiff, "game_spotdiff"),
        "game_pattern_junior" => training_scene(GameKind::Pattern, AgeTier::Junior, "game_pattern_junior"),
        "game_pattern_senior" => training_scene(GameKind::Pattern, AgeTier::Senior, "game_pattern_senior"),
        "game_flash_grid_junior" => training_scene(GameKind::FlashGrid, AgeTier::Junior, "game_flash_grid_junior"),
        "game_flash_grid_senior" => training_scene(GameKind::FlashGrid, AgeTier::Senior, "game_flash_grid_senior"),
        "game_mirror_junior" => training_scene(GameKind::Mirror, AgeTier::Junior, "game_mirror_junior"),
        "game_mirror_senior" => training_scene(GameKind::Mirror, AgeTier::Senior, "game_mirror_senior"),
        "game_reaction_junior" => training_scene(GameKind::GoNoGo, AgeTier::Junior, "game_reaction_junior"),
        "game_reaction_senior" => training_scene(GameKind::GoNoGo, AgeTier::Senior, "game_reaction_senior"),
        "game_balance_junior" => training_scene(GameKind::Balance, AgeTier::Junior, "game_balance_junior"),
        "game_balance_senior" => training_scene(GameKind::Balance, AgeTier::Senior, "game_balance_senior"),
        "game_hanoi_junior" => training_scene(GameKind::Hanoi, AgeTier::Junior, "game_hanoi_junior"),
        "game_hanoi_senior" => training_scene(GameKind::Hanoi, AgeTier::Senior, "game_hanoi_senior"),
        "game_lights_junior" => training_scene(GameKind::LightsOut, AgeTier::Junior, "game_lights_junior"),
        "game_lights_senior" => training_scene(GameKind::LightsOut, AgeTier::Senior, "game_lights_senior"),
        "game_coding_junior" => training_scene(GameKind::Coding, AgeTier::Junior, "game_coding_junior"),
        "game_coding_senior" => training_scene(GameKind::Coding, AgeTier::Senior, "game_coding_senior"),
        "game_rotation_junior" => training_scene(GameKind::Rotation, AgeTier::Junior, "game_rotation_junior"),
        "game_rotation_senior" => training_scene(GameKind::Rotation, AgeTier::Senior, "game_rotation_senior"),
        "game_detective_junior" => training_scene(GameKind::Detective, AgeTier::Junior, "game_detective_junior"),
        "game_detective_senior" => training_scene(GameKind::Detective, AgeTier::Senior, "game_detective_senior"),
        "menu_puzzle_page2" => {
            let mut s = Script::default().goto_menu().wait(6).click(100.0, 218.0)
                .click(240.0, 218.0).click(586.0, 218.0).click(120.0, 280.0);
            s.0.push(Step::CheckSelection(GameKind::Pattern));
            s.shot("menu_puzzle_page2")
        }
        "menu_puzzle_page3" => {
            let mut s = Script::default().goto_menu().wait(6).click(100.0, 218.0)
                .click(240.0, 218.0).key(KeyCode::PageDown).key(KeyCode::PageDown)
                .click(520.0, 280.0);
            s.0.push(Step::CheckSelection(GameKind::Detective));
            s = s.shot("menu_puzzle_page3").key(KeyCode::Tab);
            s.0.push(Step::CheckSelection(GameKind::Sokoban));
            s
        }
        "game_hanoi_mouse_win" => {
            let mut s = Script::default().goto_menu();
            s.0.push(Step::SetAge(AgeTier::Junior));
            s = s.enter(GameKind::Hanoi, 1).wait(6).click_world(0.0, -120.0)
                .click_world(-210.0, -20.0).click_world(0.0, -20.0)
                .click_world(-210.0, -20.0).click_world(210.0, -20.0)
                .click_world(0.0, -20.0).click_world(210.0, -20.0).wait(8);
            s.0.push(Step::CheckResult { won: true });
            s = s.shot("game_hanoi_mouse_win").tap(InputAction::Start).wait(8);
            s.0.push(Step::CheckSessionLevel(2));
            s = s.tap(InputAction::Back).wait(6);
            s.0.push(Step::CheckState(AppState::Menu));
            s
        }
        "game_flash_recall" => {
            let mut s = Script::default().goto_menu();
            s.0.push(Step::SetAge(AgeTier::Junior));
            s.enter(GameKind::FlashGrid, 1).wait(6).click_world(0.0, -120.0)
                .wait(290).shot("game_flash_recall")
        }
        "game_hanoi_wide" | "game_hanoi_hidpi" => {
            let mut s = Script::default().goto_menu();
            s.0.push(Step::SetAge(AgeTier::Senior));
            let name = if scene == "game_hanoi_wide" {
                s.0.push(Step::SetDisplay(DisplayMode::Widescreen16x9));
                "game_hanoi_wide"
            } else {
                s.0.push(Step::SetDpi(Some(2.0)));
                "game_hanoi_hidpi"
            };
            s = s.enter(GameKind::Hanoi, 10).wait(12).click_world(0.0, -120.0).wait(6).shot(name);
            s.0.push(Step::SetDisplay(DisplayMode::Classic4x3));
            s.0.push(Step::SetDpi(None));
            s.wait(6)
        }
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

const ALL_SCENES: &[&str] = &[
    "menu_library",
    "menu_stage",
    "menu_settings",
    "menu_controls",
    "menu_puzzle",
    "menu_mouse_stage",
    "menu_mouse_age",
    "menu_mouse_play",
    "menu_keyboard",
    "menu_capture",
    "menu_mouse_settings",
    "game_tank",
    "game_bomb_maze",
    "game_space_shooter",
    "game_super_mario",
    "game_contra",
    "game_bubble",
    "game_memory",
    "game_sokoban",
    "game_schulte",
    "game_stroop",
    "game_sudoku",
    "game_sliding",
    "game_maze",
    "game_linkup",
    "game_simon",
    "game_spotdiff",
    "paused",
    "result_win",
    "result_lose",
    "result_spam",
    "menu_puzzle_page2",
    "menu_puzzle_page3",
    "game_pattern_junior",
    "game_pattern_senior",
    "game_flash_grid_junior",
    "game_flash_grid_senior",
    "game_mirror_junior",
    "game_mirror_senior",
    "game_reaction_junior",
    "game_reaction_senior",
    "game_balance_junior",
    "game_balance_senior",
    "game_hanoi_junior",
    "game_hanoi_senior",
    "game_lights_junior",
    "game_lights_senior",
    "game_coding_junior",
    "game_coding_senior",
    "game_rotation_junior",
    "game_rotation_senior",
    "game_detective_junior",
    "game_detective_senior",
    "game_hanoi_mouse_win",
    "game_flash_recall",
    "game_hanoi_wide",
    "game_hanoi_hidpi",
];

#[derive(Resource)]
struct CaptureRunner {
    steps: Vec<Step>,
    cursor: usize,
    wait: u32,
    out: PathBuf,
    initial_age: AgeTier,
    initial_music: f32,
    keyboard: ButtonInput<KeyCode>,
    mouse: ButtonInput<MouseButton>,
    pointer: Option<Vec2>,
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
        let initial_age = app.world().resource::<SaveData>().age_tier;
        let initial_music = app.world().resource::<SaveData>().settings.music_volume;
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(1.0 / 60.0)));
        app.insert_resource(CaptureRunner {
            steps,
            cursor: 0,
            wait: 0,
            out: self.out.clone(),
            initial_age,
            initial_music,
            keyboard: ButtonInput::default(),
            mouse: ButtonInput::default(),
            pointer: None,
        })
        .add_systems(
            PreUpdate,
            (drive_capture, remember_script_input).chain().after(InputSystems).before(ActionInputSet),
        );
    }
}

fn remember_script_input(keys: Res<ButtonInput<KeyCode>>, mouse: Res<ButtonInput<MouseButton>>, mut runner: ResMut<CaptureRunner>) {
    runner.keyboard = keys.clone();
    runner.mouse = mouse.clone();
}

#[allow(clippy::too_many_arguments)]
fn drive_capture(
    mut commands: Commands,
    mut runner: ResMut<CaptureRunner>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
    bindings: Res<InputBindings>,
    mut canvas: ResMut<crate::common::pixel_canvas::PixelCanvasConfig>,
    mut selected: ResMut<SelectedGame>,
    mut save: ResMut<SaveData>,
    mut session: Option<ResMut<GameSession>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    // 自动化期间只使用脚本键鼠，避免桌面真实输入和窗口焦点事件污染回归结果。
    *keys = runner.keyboard.clone();
    keys.clear();
    *mouse = runner.mouse.clone();
    mouse.clear();
    if let Ok(mut window) = windows.single_mut() { window.set_cursor_position(runner.pointer); }
    if runner.wait > 0 {
        runner.wait -= 1;
        return;
    }
    let p1 = PlayerSlot::One.index();
    while runner.cursor < runner.steps.len() {
        let index = runner.cursor;
        runner.cursor += 1;
        match &runner.steps[index] {
            Step::ClickWorld(position) => {
                if let Ok(mut window) = windows.single_mut() {
                    let (w, h) = canvas.display_mode.canvas_size();
                    let scale = (window.width() / w as f32).min(window.height() / h as f32).floor().max(1.0);
                    let center = Vec2::new(window.width(), window.height()) * 0.5;
                    let point = center + *position * Vec2::new(1.0, -1.0) * (scale / crate::common::px::WORLD_PER_PX);
                    window.set_cursor_position(Some(point));
                    runner.pointer = Some(point);
                }
                mouse.press(MouseButton::Left);
            }
            Step::Click(position) => {
                if let Ok(mut window) = windows.single_mut() {
                    let scale = (window.width() / 1280.0).min(window.height() / 720.0);
                    let offset = (Vec2::new(window.width(), window.height()) - Vec2::new(1280.0, 720.0) * scale) * 0.5;
                    let point = *position * scale + offset;
                    window.set_cursor_position(Some(point));
                    runner.pointer = Some(point);
                }
                mouse.press(MouseButton::Left);
            }
            Step::ReleaseMouse => { mouse.release(MouseButton::Left); }
            Step::Key(key, pressed) => {
                if *pressed { keys.press(*key); } else { keys.release(*key); }
            }
            Step::SetAge(age) => save.age_tier = *age,
            Step::SetDisplay(mode) => { canvas.display_mode = *mode; save.settings.display_mode = *mode; }
            Step::SetDpi(scale) => {
                if let Ok(mut window) = windows.single_mut() { window.resolution.set_scale_factor_override(*scale); }
            }
            Step::CheckResult { won } => {
                let result = session.as_ref().expect("result needs a session");
                assert!(result.finished && result.won == *won, "实际操作未得到预期结算");
                if *won {
                    assert!(save.unlocked_levels[result.kind.index()] >= (result.level + 1).min(result.kind.max_level()));
                    let persisted = SaveData::load();
                    assert!(persisted.high_scores[result.kind.index()] >= result.score);
                    assert!(persisted.unlocked_levels[result.kind.index()] >= (result.level + 1).min(result.kind.max_level()));
                }
            }
            Step::CheckSessionLevel(level) => {
                let playing = session.as_ref().expect("level needs a session");
                assert_eq!(playing.level, *level);
                assert!(!playing.finished);
            }
            Step::CheckSelection(kind) => assert_eq!(selected.0, *kind, "菜单选择与预期不符"),
            Step::CheckLevel(level) => assert_eq!(save.selected_levels[selected.0.index()], *level, "鼠标选关未生效"),
            Step::CheckAge { toggled } => assert_eq!(save.age_tier, if *toggled { runner.initial_age.toggled() } else { runner.initial_age }, "年龄档切换未生效"),
            Step::CheckState(expected) => assert_eq!(state.get(), expected, "菜单/游戏状态与预期不符"),
            Step::CheckMusicIncrease => assert!((save.settings.music_volume - (runner.initial_music + 0.1).min(1.0)).abs() < 0.001, "鼠标音量加号未生效"),
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
