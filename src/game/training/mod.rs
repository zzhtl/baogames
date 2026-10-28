//! 共享交互外壳、独立规则的十款专注力与思维练习。
mod logic;
#[cfg(test)]
mod tests;
mod view;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::pixel_canvas::{PixelCanvasConfig, cursor_world};
use crate::common::settings::{InputAction, InputBindings, PlayerSlot};
use crate::game::model::{AgeTier, AppState, GameKind, GameSession, SaveData};
use bevy::prelude::*;
use logic::{Action, Challenge, Outcome};
pub(super) use view::{TrainingArt, render};

#[derive(Resource)]
pub(super) struct TrainingStage {
    kind: GameKind,
    level: u8,
    age: AgeTier,
    challenge: Challenge,
    cursor: usize,
    started: bool,
    completed: u8,
    mistakes: u32,
    message: String,
    feedback: f32,
    next_round: bool,
    dirty: bool,
}

pub(super) fn setup_stage(commands: &mut Commands, kind: GameKind, level: u8, age: AgeTier) {
    debug_assert!(GameKind::TRAINING.contains(&kind));
    commands.insert_resource(TrainingStage {
        kind,
        level,
        age,
        challenge: Challenge::new(kind, level, age, &mut rand::thread_rng()),
        cursor: 0,
        started: false,
        completed: 0,
        mistakes: 0,
        message: String::new(),
        feedback: 0.0,
        next_round: false,
        dirty: true,
    });
}

fn finish(stage: &TrainingStage, session: &mut GameSession, save: &mut SaveData, won: bool) {
    session.finished = true;
    session.won = won;
    session.status = if won {
        format!("得分 {} · 尝试纠正 {} 次", session.score, stage.mistakes)
    } else {
        "休息一下，再试一次吧".to_string()
    };
    let index = stage.kind.index();
    save.high_scores[index] = save.high_scores[index].max(session.score);
    if won {
        save.unlocked_levels[index] =
            save.unlocked_levels[index].max((stage.level + 1).min(stage.kind.max_level()));
    }
    save.store();
}

fn respond(
    stage: &mut TrainingStage,
    outcome: Outcome,
    session: &mut GameSession,
    save: &mut SaveData,
    sfx: &mut MessageWriter<PlaySfx>,
) {
    match outcome {
        Outcome::Quiet => return,
        Outcome::Changed => {
            sfx.write(PlaySfx(SfxKind::MenuMove));
        }
        Outcome::Solved => {
            stage.completed += 1;
            session.score += 100;
            sfx.write(PlaySfx(SfxKind::Match));
            if stage.completed >= logic::rounds(stage.kind, stage.level) {
                finish(stage, session, save, true);
            } else {
                stage.message = "做得好！准备下一题".to_string();
                stage.feedback = 0.8;
                stage.next_round = true;
            }
        }
        Outcome::Mistake(message) => {
            stage.mistakes += 1;
            stage.message = message.to_string();
            stage.feedback = 1.0;
            stage.next_round = stage.kind == GameKind::GoNoGo;
            sfx.write(PlaySfx(SfxKind::Deny));
            if stage.kind == GameKind::GoNoGo {
                session.lives -= 1;
                if session.lives <= 0 {
                    finish(stage, session, save, false);
                }
            }
        }
    }
    stage.dirty = true;
}

#[allow(clippy::too_many_arguments)]
pub(super) fn input(
    time: Res<Time>,
    actions: Res<ActionState>,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    canvas: Res<PixelCanvasConfig>,
    buttons: Query<&view::GameButton>,
    mut stage: ResMut<TrainingStage>,
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    mut next: ResMut<NextState<AppState>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.paused || session.finished {
        return;
    }
    let mut clicked = None;
    if mouse.just_pressed(MouseButton::Left)
        && let Ok(window) = windows.single()
        && let Some(pos) = window
            .cursor_position()
            .and_then(|p| cursor_world(window, &canvas, p))
    {
        clicked = buttons
            .iter()
            .find(|button| button.bounds.contains(pos))
            .map(|button| button.action);
    }
    if clicked == Some(view::UiAction::Back) {
        next.set(AppState::Menu);
        return;
    }
    if clicked == Some(view::UiAction::Restart)
        || actions.just_pressed(PlayerSlot::One, InputAction::Reset)
    {
        next.set(AppState::Playing);
        return;
    }
    let primary = actions.just_pressed(PlayerSlot::One, InputAction::Primary);
    if !stage.started {
        if primary || clicked == Some(view::UiAction::Game(Action::Primary)) {
            stage.started = true;
            stage.dirty = true;
            sfx.write(PlaySfx(SfxKind::MenuConfirm));
        }
        return;
    }
    if stage.feedback > 0.0 {
        stage.feedback = (stage.feedback - time.delta_secs()).max(0.0);
        if stage.feedback == 0.0 {
            if stage.next_round {
                stage.challenge =
                    Challenge::new(stage.kind, stage.level, stage.age, &mut rand::thread_rng());
                stage.cursor = 0;
                stage.next_round = false;
            }
            stage.message.clear();
            stage.dirty = true;
        }
        return;
    }
    let before = stage.challenge.countdown();
    let timed = stage.challenge.tick(time.delta_secs());
    if before != stage.challenge.countdown() {
        stage.dirty = true;
    }
    respond(&mut stage, timed, &mut session, &mut save, &mut sfx);
    if stage.feedback > 0.0 || session.finished {
        return;
    }

    let fallback = |key| {
        keys.just_pressed(key)
            && !InputAction::ALL
                .iter()
                .any(|action| bindings.0[0].binding(*action).keyboard.key_code() == key)
    };
    let direction = [
        (InputAction::Up, KeyCode::ArrowUp, (0, -1)),
        (InputAction::Down, KeyCode::ArrowDown, (0, 1)),
        (InputAction::Left, KeyCode::ArrowLeft, (-1, 0)),
        (InputAction::Right, KeyCode::ArrowRight, (1, 0)),
    ]
    .into_iter()
    .find(|(action, key, _)| actions.just_pressed(PlayerSlot::One, *action) || fallback(*key));
    let action = if let Some(view::UiAction::Game(action)) = clicked {
        Some(action)
    } else if primary {
        Some(Action::Primary)
    } else if actions.just_pressed(PlayerSlot::One, InputAction::Secondary) {
        Some(Action::Secondary)
    } else {
        direction.map(|(_, _, (dx, dy))| Action::Move(dx, dy))
    };
    if let Some(action) = action {
        let stage = &mut *stage;
        let outcome = stage.challenge.act(action, &mut stage.cursor);
        respond(stage, outcome, &mut session, &mut save, &mut sfx);
    }
}

fn instructions(kind: GameKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        GameKind::Pattern => (
            "规律推理",
            "观察上排的变化，选择下一项",
            "方向键选择答案，确认作答；也可以直接点击答案。",
        ),
        GameKind::FlashGrid => (
            "视觉工作记忆",
            "先记住亮格，隐藏后选回原来的位置",
            "点击或确认标记格子，标完后按提交；再次点击可取消。",
        ),
        GameKind::Mirror => (
            "空间对称",
            "在右边画出左图的镜像",
            "中间虚线是镜子。点击右侧格子涂色，再提交检查。",
        ),
        GameKind::GoNoGo => (
            "反应抑制",
            "认准本关目标，该按才按，其他图案等待",
            "绿灯时确认，红灯时等待；8–12 岁档第 6 关起只点绿色圆形。",
        ),
        GameKind::Balance => (
            "数量组合",
            "选择砝码，让总和等于目标",
            "点击或确认选中砝码，再次点击可取消，配好后提交。",
        ),
        GameKind::Hanoi => (
            "步骤规划",
            "把左柱圆盘全部搬到右柱",
            "先选起点柱，再选终点柱。大盘不能压小盘，可以撤销。",
        ),
        GameKind::LightsOut => (
            "因果推理",
            "翻转十字相邻的灯，把灯全部关掉",
            "点击一格会翻转它及上下左右的灯。可以逐步撤销。",
        ),
        GameKind::Coding => (
            "路线规划",
            "先安排指令，再让机器人走到星标",
            "方向键或屏幕箭头添加指令。运行查看结果，撤销修改。",
        ),
        GameKind::Rotation => (
            "空间旋转",
            "旋转右侧图形，让它与左侧一致",
            "左右键或旋转按钮调整方向，完全一致后按提交。",
        ),
        GameKind::Detective => (
            "选择性注意",
            "找齐与目标颜色、形状都相同的图案",
            "点击或确认标记目标，已经找到的图案会出现对勾。",
        ),
        _ => unreachable!("training instructions only"),
    }
}
