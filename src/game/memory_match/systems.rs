use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::input::ActionState;
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::components::*;
use super::constants::*;
use super::resources::{MemoryControls, MemoryStage};

pub fn memory_sample_input(
    actions: Res<ActionState>,
    session: Res<GameSession>,
    mut controls: ResMut<MemoryControls>,
) {
    if session.kind != GameKind::MemoryMatch || session.paused || session.finished {
        controls.clear();
        return;
    }
    controls.sample(&actions);
}

pub fn memory_input(
    mut controls: ResMut<MemoryControls>,
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut stage: ResMut<MemoryStage>,
    mut card_q: Query<(Entity, &mut MemoryCard)>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    if session.kind != GameKind::MemoryMatch || session.paused || session.finished {
        return;
    }
    let dt = time.delta_secs();

    if stage.message_clock > 0.0 {
        stage.message_clock = (stage.message_clock - dt).max(0.0);
    }
    if stage.preview_timer > 0.0 {
        stage.preview_timer = (stage.preview_timer - dt).max(0.0);
        controls.take_move();
        controls.take_flip();
        if stage.preview_timer <= 0.0 {
            stage.message = "开始配对！".to_string();
            stage.message_clock = 0.9;
        }
        return;
    }
    stage.time_left = (stage.time_left - dt).max(0.0);

    // 双牌对比窗口
    if let (Some(first), Some(second)) = (stage.first_pick, stage.second_pick) {
        stage.resolve_timer -= dt;
        if stage.resolve_timer <= 0.0 {
            let mut p1: Option<u32> = None;
            let mut p2: Option<u32> = None;
            for (e, card) in card_q.iter() {
                if e == first {
                    p1 = Some(card.pair_id);
                }
                if e == second {
                    p2 = Some(card.pair_id);
                }
            }
            let matched = p1.is_some() && p1 == p2;
            for (e, mut card) in &mut card_q {
                if e == first || e == second {
                    card.state = if matched {
                        CardState::Matched
                    } else {
                        CardState::FaceDown
                    };
                    card.feedback = if matched { 0.34 } else { -0.34 };
                }
            }
            if matched {
                stage.pairs_done += 1;
                stage.combo_streak = stage.combo_streak.saturating_add(1).min(9);
                stage.best_combo = stage.best_combo.max(stage.combo_streak);
                let points = match_score(stage.combo_streak);
                session.score += points;
                stage.message = if stage.combo_streak > 1 {
                    format!("{} 连对 · +{}", stage.combo_streak, points)
                } else {
                    format!("配对成功 · +{}", points)
                };
                stage.message_clock = 1.0;
                sfx.write(PlaySfx(SfxKind::Match));
            } else {
                stage.combo_streak = 0;
                stage.message = "不一样，再想想～".to_string();
                stage.message_clock = 1.0;
                sfx.write(PlaySfx(SfxKind::Deny));
            }
            stage.first_pick = None;
            stage.second_pick = None;
            stage.resolve_timer = 0.0;
        }
    }

    // 光标移动
    let (dc, dr) = controls.take_move().unwrap_or((0, 0));
    let col = stage.cursor_col + dc;
    let row = stage.cursor_row + dr;
    let (prev_col, prev_row) = (stage.cursor_col, stage.cursor_row);
    stage.cursor_col = col.clamp(0, stage.cols as i32 - 1);
    stage.cursor_row = row.clamp(0, stage.rows as i32 - 1);
    if stage.cursor_col != prev_col || stage.cursor_row != prev_row {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    let pressed = controls.take_flip();
    // 等待对比期间禁止翻牌
    if stage.second_pick.is_some() {
        return;
    }

    if !pressed {
        return;
    }

    let mut target: Option<Entity> = None;
    for (e, card) in card_q.iter() {
        if card.col == stage.cursor_col
            && card.row == stage.cursor_row
            && card.state == CardState::FaceDown
        {
            target = Some(e);
            break;
        }
    }
    let Some(target) = target else {
        sfx.write(PlaySfx(SfxKind::Deny));
        return;
    };
    // 不能翻自己（同一张已经作为 first_pick 翻起来了；FaceDown 过滤已经规避了）
    if Some(target) == stage.first_pick {
        return;
    }
    for (e, mut card) in &mut card_q {
        if e == target {
            card.state = CardState::FaceUp;
        }
    }
    stage.flips += 1;
    sfx.write(PlaySfx(SfxKind::Flip));
    if stage.first_pick.is_none() {
        stage.first_pick = Some(target);
    } else {
        stage.second_pick = Some(target);
        stage.resolve_timer = PEEK_TIME;
    }
}

pub fn match_score(streak: u8) -> u32 {
    10 + streak.saturating_sub(1) as u32 * 5
}

#[allow(clippy::type_complexity)]
pub fn memory_render_sync(
    session: Res<GameSession>,
    card_q: Query<(&CardFlip, &Children)>,
    mut back_q: Query<
        &mut Visibility,
        (With<CardBack>, Without<CardFace>, Without<CardFaceMatched>),
    >,
    mut face_q: Query<
        &mut Visibility,
        (With<CardFace>, Without<CardBack>, Without<CardFaceMatched>),
    >,
    mut matched_q: Query<
        &mut Visibility,
        (With<CardFaceMatched>, Without<CardBack>, Without<CardFace>),
    >,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    // 三个组按 card.state 互斥显示；set-if-changed 避免每帧标脏
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
    for (flip, children) in &card_q {
        for c in children {
            let child = *c;
            if let Ok(mut v) = back_q.get_mut(child) {
                apply(&mut v, flip.shown == CardState::FaceDown);
            }
            if let Ok(mut v) = face_q.get_mut(child) {
                apply(&mut v, flip.shown == CardState::FaceUp);
            }
            if let Ok(mut v) = matched_q.get_mut(child) {
                apply(&mut v, flip.shown == CardState::Matched);
            }
        }
    }
}

pub fn memory_card_flip_update(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<MemoryStage>,
    mut cards: Query<(&mut MemoryCard, &mut CardFlip, &mut Transform)>,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    let dt = time.delta_secs();
    for (mut card, mut flip, mut transform) in &mut cards {
        let desired = if stage.preview_timer > 0.0 {
            CardState::FaceUp
        } else {
            card.state
        };
        if flip.target != desired {
            flip.target = desired;
            flip.progress = 0.0;
        }
        if flip.progress < 1.0 {
            flip.progress = (flip.progress + dt / CARD_FLIP_TIME).min(1.0);
            if flip.progress >= 0.5 {
                flip.shown = flip.target;
            }
        }
        let flip_scale = if flip.progress < 1.0 {
            (flip.progress * std::f32::consts::PI).cos().abs()
        } else {
            1.0
        };
        let mut y_scale = 1.0;
        let mut rotation = 0.0;
        if card.feedback != 0.0 {
            let positive = card.feedback > 0.0;
            let left = card.feedback.abs();
            let phase = (0.34 - left) * 42.0;
            if positive {
                y_scale += phase.sin().max(0.0) * 0.08;
                card.feedback = (card.feedback - dt).max(0.0);
            } else {
                rotation = phase.sin() * 0.055;
                card.feedback = (card.feedback + dt).min(0.0);
            }
        }
        transform.scale = Vec3::new(flip_scale.max(0.04), y_scale, 1.0);
        transform.rotation = Quat::from_rotation_z(rotation);
    }
}

pub fn memory_cursor_follow(
    time: Res<Time>,
    session: Res<GameSession>,
    stage: Res<MemoryStage>,
    mut q: Query<(&CardCursor, &mut Transform, &mut Sprite)>,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    let center = stage.cell_center(stage.cursor_col, stage.cursor_row);
    let blend = (time.delta_secs() * 24.0).min(1.0);
    let pulse = 0.88 + (time.elapsed_secs() * 7.0).sin() * 0.12;
    for (cursor, mut t, mut sprite) in &mut q {
        let target = center + cursor.offset;
        t.translation.x += (target.x - t.translation.x) * blend;
        t.translation.y += (target.y - t.translation.y) * blend;
        sprite.color = COLOR_CURSOR.with_alpha(pulse);
    }
}

pub fn memory_check_finish(
    mut session: ResMut<GameSession>,
    mut save: ResMut<SaveData>,
    stage: Res<MemoryStage>,
) {
    if session.kind != GameKind::MemoryMatch || session.finished {
        return;
    }
    let idx = session.kind.index();
    if stage.pairs_done >= stage.pairs_total {
        // 通关：剩余时间奖励 + 翻牌效率奖励
        let time_bonus = (stage.time_left * 2.0) as u32;
        let min_flips = stage.pairs_total * 2;
        let efficiency_bonus = if stage.flips <= min_flips {
            50
        } else {
            (50u32).saturating_sub((stage.flips - min_flips) * 2)
        };
        session.score += time_bonus + efficiency_bonus;
        session.finished = true;
        session.won = true;
        session.status = format!("翻牌 {} 次 · 时间奖励 +{}", stage.flips, time_bonus);
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
        session.status = format!("时间到，配对 {}/{}", stage.pairs_done, stage.pairs_total);
        if session.score > save.high_scores[idx] {
            save.high_scores[idx] = session.score;
            save.store();
        }
    }
}

pub fn memory_hud_update(
    session: Res<GameSession>,
    save: Res<SaveData>,
    stage: Res<MemoryStage>,
    mut hud: Query<&mut Text2d, (With<MemoryHud>, Without<MemoryMessage>, Without<MemoryScoreHud>)>,
    mut score_hud: Query<&mut Text2d, (With<MemoryScoreHud>, Without<MemoryMessage>, Without<MemoryHud>)>,
    mut msg: Query<&mut Text2d, (With<MemoryMessage>, Without<MemoryHud>, Without<MemoryScoreHud>)>,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    let high = save.high_scores[GameKind::MemoryMatch.index()].max(session.score);
    // 顶栏右侧只放当前进度，分数与纪录挪到底栏——240 像素宽的画布放不下一整行
    if let Ok(mut t) = hud.single_mut() {
        set_text(
            &mut t,
            &format!(
                "第{}关  时间 {:.0}  配对 {}/{}",
                session.level,
                stage.time_left.max(0.0),
                stage.pairs_done,
                stage.pairs_total,
            ),
        );
    }
    if let Ok(mut t) = score_hud.single_mut() {
        set_text(&mut t, &format!("分数 {}  纪录 {}  翻牌 {}", session.score, high, stage.flips));
    }
    // 暂停/结束由统一覆盖层显示，这里只放玩法瞬时消息
    if let Ok(mut t) = msg.single_mut() {
        let combo = format!("连对 x{}", stage.combo_streak);
        let value = if stage.message_clock > 0.0 && !session.finished {
            stage.message.as_str()
        } else if stage.combo_streak > 0 {
            combo.as_str()
        } else {
            ""
        };
        set_text(&mut t, value);
    }
}
