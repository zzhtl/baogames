use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::common::constants::{FONT_BODY};
use bevy::sprite::Anchor;
use crate::common::px::px;
use crate::common::theme::{ACCENT, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::common::render::{UiFont, attach_sprite_parts};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{MemoryControls, MemoryStage};
use super::sprites::{CARD_BACK, CARD_CANONICAL, CARD_FACE, CARD_FACE_MATCHED};

pub fn setup_stage(commands: &mut Commands, font: &UiFont, hud_root: Entity, level: u8) {
    commands.insert_resource(MemoryControls::default());
    let idx = (level.clamp(1, 10) - 1) as usize;
    let (cols, rows) = LEVEL_GRID[idx];
    let total_time = LEVEL_TIME[idx];

    let cell_size = (BOARD_W / cols as f32).min(BOARD_H / rows as f32);
    let card_size = cell_size * (1.0 - CARD_GAP_RATIO);
    let total_w = cell_size * cols as f32;
    let total_h = cell_size * rows as f32;
    let origin_x = -total_w * 0.5 + cell_size * 0.5;
    let origin_y = total_h * 0.5 - cell_size * 0.5 + BOARD_CENTER_Y;

    let pairs_total = (cols * rows) / 2;
    let pair_ids = shuffled_pair_ids(pairs_total);

    for row in 0..rows as i32 {
        for col in 0..cols as i32 {
            let pid = pair_ids[(row as u32 * cols + col as u32) as usize];
            let center = Vec2::new(
                origin_x + col as f32 * cell_size,
                origin_y - row as f32 * cell_size,
            );
            spawn_card(commands, font, center, card_size, col, row, pid);
        }
    }

    spawn_cursor(commands, Vec2::new(origin_x, origin_y), card_size);
    spawn_hud(commands, font, hud_root);

    commands.insert_resource(MemoryStage {
        cols,
        rows,
        cell_size,
        origin: Vec2::new(origin_x, origin_y),
        pairs_total,
        pairs_done: 0,
        flips: 0,
        time_left: total_time,
        cursor_col: 0,
        cursor_row: 0,
        first_pick: None,
        second_pick: None,
        resolve_timer: 0.0,
        message: "先记住所有牌！".to_string(),
        message_clock: 2.4,
        preview_timer: PREVIEW_TIME,
        combo_streak: 0,
        best_combo: 0,
    });
}

/// 生成洗好牌的 pair_id 序列：每个 id 出现两次。抽出来便于单元测试。
pub fn shuffled_pair_ids(pairs_total: u32) -> Vec<u32> {
    let mut ids: Vec<u32> = (0..pairs_total).flat_map(|i| [i, i]).collect();
    let mut rng = thread_rng();
    ids.shuffle(&mut rng);
    ids
}

fn spawn_card(
    commands: &mut Commands,
    font: &UiFont,
    center: Vec2,
    size: f32,
    col: i32,
    row: i32,
    pair_id: u32,
) {
    // 父实体：透明 sprite 作占位，承载 Transform 与 Visibility，让子的 Inherited 生效。
    let parent = commands
        .spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(size)),
            Transform::from_translation(center.extend(0.0)),
            MemoryCard {
                col,
                row,
                pair_id,
                state: CardState::FaceDown,
                feedback: 0.0,
            },
            CardFlip {
                shown: CardState::FaceUp,
                target: CardState::FaceUp,
                progress: 1.0,
            },
            GameEntity,
        ))
        .id();

    // 三个互斥显示的组节点：卡背 / 卡面 / 已配对。
    // 组按 canonical 64×64 绘制并整体缩放到实机卡尺寸，精灵与离线预览同源。
    let scale = size / CARD_CANONICAL;
    let ch = PAIR_CHARS[(pair_id as usize) % PAIR_CHARS.len()];
    let spawn_group = |commands: &mut Commands,
                       visible: bool,
                       marker_spawn: fn(&mut EntityCommands)|
     -> Entity {
        let mut ec = commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.0), Vec2::splat(CARD_CANONICAL)),
            Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::splat(scale)),
            if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
            GameEntity,
            ChildOf(parent),
        ));
        marker_spawn(&mut ec);
        ec.id()
    };

    let back = spawn_group(commands, true, |ec| {
        ec.insert(CardBack);
    });
    attach_sprite_parts(commands, back, &CARD_BACK, GameEntity);

    let face = spawn_group(commands, false, |ec| {
        ec.insert(CardFace);
    });
    attach_sprite_parts(commands, face, &CARD_FACE, GameEntity);

    let matched = spawn_group(commands, false, |ec| {
        ec.insert(CardFaceMatched);
    });
    attach_sprite_parts(commands, matched, &CARD_FACE_MATCHED, GameEntity);

    // 文本不放缩放组内（挂卡片父实体、直接用实机字号）：
    // Text2d 按字号光栅化后才被 Transform.scale 拉伸，放组里会发虚。
    // 文本带所属组的 marker，由 memory_render_sync 与组一起切换显隐。
    commands.spawn((
        Text2d::new("?"),
        TextFont::from_font_size(size * 0.55).with_font(font.0.clone()),
        TextColor(palette::BACK_PATTERN),
        Transform::from_xyz(0.0, 0.0, 0.2),
        CardBack,
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Text2d::new(ch.to_string()),
        TextFont::from_font_size(size * 0.55).with_font(font.0.clone()),
        TextColor(pair_color(pair_id)),
        Transform::from_xyz(0.0, 0.0, 0.2),
        Visibility::Hidden,
        CardFace,
        GameEntity,
        ChildOf(parent),
    ));
    commands.spawn((
        Text2d::new(ch.to_string()),
        TextFont::from_font_size(size * 0.55).with_font(font.0.clone()),
        TextColor(pair_color(pair_id)),
        Transform::from_xyz(0.0, 0.0, 0.2),
        Visibility::Hidden,
        CardFaceMatched,
        GameEntity,
        ChildOf(parent),
    ));
}

fn spawn_cursor(commands: &mut Commands, origin: Vec2, card_size: f32) {
    let frame = card_size + 10.0;
    let bar = 4.0;
    // 4 条边：上、下、左、右。offset 是相对当前光标所在卡中心的位置偏移。
    let edges: [(Vec2, Vec2); 4] = [
        (Vec2::new(0.0, frame * 0.5), Vec2::new(frame + bar, bar)),
        (Vec2::new(0.0, -frame * 0.5), Vec2::new(frame + bar, bar)),
        (Vec2::new(-frame * 0.5, 0.0), Vec2::new(bar, frame + bar)),
        (Vec2::new(frame * 0.5, 0.0), Vec2::new(bar, frame + bar)),
    ];
    for (offset, size) in edges {
        commands.spawn((
            Sprite::from_color(COLOR_CURSOR, size),
            Transform::from_translation((origin + offset).extend(2.0)),
            CardCursor { offset },
            GameEntity,
        ));
    }
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    // 顶栏：左标题 / 右进度；底栏：左分数 / 右瞬时消息。
    // 原来是一条 45 字的单行文本，在 240 像素宽的画布上要 540 像素才放得下。
    hud_bar(commands, hud_root, 82.0, SURFACE, Color::srgb(0.45, 0.75, 0.96));
    hud_text_anchored(
        commands, font, hud_root, "记忆翻翻乐",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, MemoryHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, Color::srgb(0.45, 0.75, 0.96));
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, MemoryScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, MemoryMessage,
    );
}
