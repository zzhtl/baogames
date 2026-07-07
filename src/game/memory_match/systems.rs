use bevy::prelude::*;

use crate::common::audio::{PlaySfx, SfxKind};
use crate::common::render::set_text;
use crate::game::model::{GameKind, GameSession, SaveData};

use super::components::*;
use super::constants::*;
use super::resources::MemoryStage;

pub fn memory_input(
    keys: Res<ButtonInput<KeyCode>>,
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
                }
            }
            if matched {
                stage.pairs_done += 1;
                session.score += 10;
                stage.message = "配对成功！".to_string();
                stage.message_clock = 1.0;
                sfx.write(PlaySfx(SfxKind::Match));
            } else {
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
    let mut col = stage.cursor_col;
    let mut row = stage.cursor_row;
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        col -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        col += 1;
    }
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        row -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        row += 1;
    }
    let (prev_col, prev_row) = (stage.cursor_col, stage.cursor_row);
    stage.cursor_col = col.clamp(0, stage.cols as i32 - 1);
    stage.cursor_row = row.clamp(0, stage.rows as i32 - 1);
    if stage.cursor_col != prev_col || stage.cursor_row != prev_row {
        sfx.write(PlaySfx(SfxKind::MenuMove));
    }

    // 等待对比期间禁止翻牌
    if stage.second_pick.is_some() {
        return;
    }

    let pressed = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::KeyJ);
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

#[allow(clippy::type_complexity)]
pub fn memory_render_sync(
    session: Res<GameSession>,
    card_q: Query<(&MemoryCard, &Children)>,
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
    for (card, children) in &card_q {
        for c in children {
            let child = *c;
            if let Ok(mut v) = back_q.get_mut(child) {
                apply(&mut v, card.state == CardState::FaceDown);
            }
            if let Ok(mut v) = face_q.get_mut(child) {
                apply(&mut v, card.state == CardState::FaceUp);
            }
            if let Ok(mut v) = matched_q.get_mut(child) {
                apply(&mut v, card.state == CardState::Matched);
            }
        }
    }
}

pub fn memory_cursor_follow(
    session: Res<GameSession>,
    stage: Res<MemoryStage>,
    mut q: Query<(&CardCursor, &mut Transform)>,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    let center = stage.cell_center(stage.cursor_col, stage.cursor_row);
    for (cursor, mut t) in &mut q {
        t.translation.x = center.x + cursor.offset.x;
        t.translation.y = center.y + cursor.offset.y;
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
    mut hud: Query<&mut Text2d, (With<MemoryHud>, Without<MemoryMessage>)>,
    mut msg: Query<&mut Text2d, (With<MemoryMessage>, Without<MemoryHud>)>,
) {
    if session.kind != GameKind::MemoryMatch {
        return;
    }
    if let Ok(mut t) = hud.single_mut() {
        let high = save.high_scores[GameKind::MemoryMatch.index()].max(session.score);
        set_text(
            &mut t,
            &format!(
                "剩余时间 {:>3.0}s   配对 {}/{}   翻牌 {}   分数 {}   纪录 {}",
                stage.time_left, stage.pairs_done, stage.pairs_total, stage.flips, session.score, high,
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
