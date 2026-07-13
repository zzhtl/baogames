use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::components::*;
use super::constants::*;
use super::resources::{MoveSnapshot, SokobanControls, SokobanStage, Tile};

pub fn sokoban_sample_input(
    actions: Res<ActionState>,
    session: Res<GameSession>,
    mut controls: ResMut<SokobanControls>,
) {
    if session.kind != GameKind::Sokoban || session.paused || session.finished {
        controls.clear();
        return;
    }
    controls.sample(&actions);
}

pub fn sokoban_input(
    mut controls: ResMut<SokobanControls>,
    time: Res<Time>,
    session: Res<GameSession>,
    mut stage: ResMut<SokobanStage>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::Sokoban || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();
    if stage.message_clock > 0.0 {
        stage.message_clock = (stage.message_clock - dt).max(0.0);
    }
    stage.time_left = (stage.time_left - dt).max(0.0);
    stage.move_cd = (stage.move_cd - dt).max(0.0);

    // 重来当前关
    if controls.take_reset() {
        stage.boxes = stage.initial_boxes.clone();
        stage.player = stage.initial_player;
        stage.moves = 0;
        stage.pushes = 0;
        stage.time_left = stage.initial_time;
        stage.move_cd = 0.0;
        stage.history.clear();
        stage.message = "本关已重置".to_string();
        stage.message_clock = 1.2;
        sfx.write(PlaySfx(SfxKind::Flip));
        return;
    }

    if controls.take_undo() {
        if undo_last(&mut stage) {
            stage.move_cd = 0.0;
            stage.message = "已撤销一步".to_string();
            stage.message_clock = 0.8;
            sfx.write(PlaySfx(SfxKind::Flip));
        } else {
            stage.message = "没有可撤销的步骤".to_string();
            stage.message_clock = 0.8;
            sfx.write(PlaySfx(SfxKind::Deny));
        }
        return;
    }

    let buffered = controls.take_step();
    let immediate = buffered.is_some();
    let Some(dir) = buffered.or_else(|| controls.held_dir()) else {
        return;
    };
    if !immediate && stage.move_cd > 0.0 {
        return;
    }

    let snapshot = MoveSnapshot {
        boxes: stage.boxes.clone(),
        player: stage.player,
        moves: stage.moves,
        pushes: stage.pushes,
    };
    let pushes_before = stage.pushes;
    if try_move(&mut stage, dir) {
        stage.history.push(snapshot);
        stage.move_cd = MOVE_COOLDOWN;
        if stage.pushes > pushes_before {
            // 被推箱子的新位置 = 玩家新位置 + dir
            let (bc, br) = (stage.player.0 + dir.0, stage.player.1 + dir.1);
            if stage.tile_at(bc, br) == Tile::Goal {
                sfx.write(PlaySfx(SfxKind::Match));
            } else if box_in_dead_corner(&stage, (bc, br)) {
                stage.message = "箱子卡在死角，可用动作二撤销".to_string();
                stage.message_clock = 1.8;
                sfx.write(PlaySfx(SfxKind::Deny));
            } else {
                sfx.write(PlaySfx(SfxKind::Place));
            }
        } else {
            sfx.write(PlaySfx(SfxKind::MenuMove));
        }
    } else {
        // 撞墙时缩短冷却，长按时手感更跟手。
        stage.move_cd = MOVE_COOLDOWN * 0.5;
        if immediate {
            sfx.write(PlaySfx(SfxKind::Deny));
        }
    }
}

pub fn undo_last(stage: &mut SokobanStage) -> bool {
    let Some(snapshot) = stage.history.pop() else {
        return false;
    };
    stage.boxes = snapshot.boxes;
    stage.player = snapshot.player;
    stage.moves = snapshot.moves;
    stage.pushes = snapshot.pushes;
    true
}

pub fn box_in_dead_corner(stage: &SokobanStage, position: (i32, i32)) -> bool {
    if stage.tile_at(position.0, position.1) == Tile::Goal {
        return false;
    }
    let wall = |dc: i32, dr: i32| {
        stage.tile_at(position.0 + dc, position.1 + dr) == Tile::Wall
    };
    (wall(-1, 0) || wall(1, 0)) && (wall(0, -1) || wall(0, 1))
}

/// 尝试让玩家朝 dir 走一步。返回是否真的发生移动。抽出来便于单元测试。
pub fn try_move(stage: &mut SokobanStage, dir: (i32, i32)) -> bool {
    let (pc, pr) = stage.player;
    let nc = pc + dir.0;
    let nr = pr + dir.1;

    if stage.tile_at(nc, nr) == Tile::Wall {
        return false;
    }

    if let Some(box_idx) = stage.box_at(nc, nr) {
        let bnc = nc + dir.0;
        let bnr = nr + dir.1;
        if stage.tile_at(bnc, bnr) == Tile::Wall {
            return false;
        }
        if stage.box_at(bnc, bnr).is_some() {
            return false;
        }
        stage.boxes[box_idx] = (bnc, bnr);
        stage.pushes += 1;
    }

    stage.player = (nc, nr);
    stage.moves += 1;
    true
}

pub fn sokoban_render_sync(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<SokobanStage>,
    mut player_q: Query<&mut Transform, (With<SokoPlayer>, Without<SokoBox>)>,
    mut box_q: Query<(&SokoBox, &mut Transform), Without<SokoPlayer>>,
) {
    if session.kind != GameKind::Sokoban {
        return;
    }
    let blend = (time.delta_secs() * 22.0).min(1.0);
    if let Ok(mut t) = player_q.single_mut() {
        let c = stage.cell_center(stage.player.0, stage.player.1);
        t.translation.x += (c.x - t.translation.x) * blend;
        t.translation.y += (c.y - t.translation.y) * blend;
    }
    for (b, mut t) in &mut box_q {
        if let Some(&(c, r)) = stage.boxes.get(b.index) {
            let center = stage.cell_center(c, r);
            t.translation.x += (center.x - t.translation.x) * blend;
            t.translation.y += (center.y - t.translation.y) * blend;
        }
    }
}

pub fn sokoban_box_visual_sync(
    session: Res<GameSession>,
    stage: Res<SokobanStage>,
    box_q: Query<(&SokoBox, &Children)>,
    mut normal_q: Query<&mut Visibility, (With<SokoBoxNormal>, Without<SokoBoxDone>)>,
    mut done_q: Query<&mut Visibility, (With<SokoBoxDone>, Without<SokoBoxNormal>)>,
) {
    if session.kind != GameKind::Sokoban {
        return;
    }
    // 普通 / 完成两组按是否压在目标点上互斥显示；set-if-changed 避免每帧标脏
    let apply = |v: &mut Visibility, on: bool| {
        let target = if on {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *v != target {
            *v = target;
        }
    };
    for (b, children) in &box_q {
        let Some(&(c, r)) = stage.boxes.get(b.index) else {
            continue;
        };
        let on_goal = stage.tile_at(c, r) == Tile::Goal;
        for child in children {
            if let Ok(mut v) = normal_q.get_mut(*child) {
                apply(&mut v, !on_goal);
            }
            if let Ok(mut v) = done_q.get_mut(*child) {
                apply(&mut v, on_goal);
            }
        }
    }
}

pub fn sokoban_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<SokobanStage>,
) {
    if session.kind != GameKind::Sokoban || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.all_boxes_done() {
        // 通关：剩余时间奖励 + 推箱效率奖励
        let time_bonus = (stage.time_left * 2.0) as u32;
        let push_min = stage.boxes.len() as u32;
        let efficiency_bonus = if stage.pushes <= push_min * 3 {
            60
        } else {
            (60u32).saturating_sub((stage.pushes - push_min * 3) * 2)
        };
        session.score += 100 + time_bonus + efficiency_bonus;
        session.finished = true;
        session.won = true;
        session.status = format!(
            "步数 {} · 推箱 {} · 时间奖励 +{}",
            stage.moves, stage.pushes, time_bonus
        );
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
        }
        let next = (session.level + 1).min(10);
        if next > save.unlocked_levels[idx] {
            save.unlocked_levels[idx] = next;
        }
        save.store();
        return;
    }
    if stage.time_left <= 0.0 {
        session.finished = true;
        session.won = false;
        session.status = "时间到，箱子还没归位".to_string();
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn sokoban_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<SokobanStage>,
    mut hud: Query<&mut Text2d, (With<SokobanHud>, Without<SokobanMessage>)>,
    mut msg: Query<&mut Text2d, (With<SokobanMessage>, Without<SokobanHud>)>,
) {
    if session.kind != GameKind::Sokoban {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let done = stage.boxes.iter().filter(|&&(c, r)| stage.tile_at(c, r) == Tile::Goal).count();
        let total = stage.boxes.len();
        let high = save.high_scores[GameKind::Sokoban.index()].max(session.score);
        set_text(
            &mut t,
            &format!(
                "剩余 {:>3.0}s   箱子 {}/{}   步数 {}   推箱 {}   撤销 {}   分数 {}   纪录 {}",
                stage.time_left,
                done,
                total,
                stage.moves,
                stage.pushes,
                stage.history.len(),
                session.score,
                high,
            ),
        );
    }
    // 暂停/结束由统一覆盖层显示，这里只放玩法瞬时消息
    if let Ok(mut t) = msg.single_mut() {
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
