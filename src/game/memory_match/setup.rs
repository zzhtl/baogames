use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::common::render::{UiFont, panel, text};
use crate::game::model::GameEntity;

use super::components::*;
use super::constants::*;
use super::resources::MemoryStage;

pub fn setup_stage(commands: &mut Commands, font: &UiFont, level: u8) {
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
    spawn_hud(commands, font, level);

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
        message: format!("第 {} 关 - 翻开同样的牌！", level),
        message_clock: 2.4,
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
            },
            GameEntity,
        ))
        .id();

    // 卡背边框
    commands.spawn((
        Sprite::from_color(COLOR_BACK_BORDER, Vec2::splat(size)),
        Transform::from_xyz(0.0, 0.0, 0.1),
        CardBack,
        GameEntity,
        ChildOf(parent),
    ));
    // 卡背内层
    commands.spawn((
        Sprite::from_color(COLOR_BACK_INNER, Vec2::splat(size - 8.0)),
        Transform::from_xyz(0.0, 0.0, 0.15),
        CardBack,
        GameEntity,
        ChildOf(parent),
    ));
    // 卡背问号
    commands.spawn((
        Text2d::new("?"),
        TextFont::from_font_size(size * 0.55).with_font(font.0.clone()),
        TextColor(COLOR_BACK_MARK),
        Transform::from_xyz(0.0, 0.0, 0.2),
        CardBack,
        GameEntity,
        ChildOf(parent),
    ));

    // 卡面边框
    commands.spawn((
        Sprite::from_color(COLOR_FACE_BORDER, Vec2::splat(size)),
        Transform::from_xyz(0.0, 0.0, 0.1),
        Visibility::Hidden,
        CardFace,
        CardFaceBorder,
        GameEntity,
        ChildOf(parent),
    ));
    // 卡面内层
    commands.spawn((
        Sprite::from_color(COLOR_FACE_INNER, Vec2::splat(size - 8.0)),
        Transform::from_xyz(0.0, 0.0, 0.15),
        Visibility::Hidden,
        CardFace,
        CardFaceInner,
        GameEntity,
        ChildOf(parent),
    ));
    // 卡面字符
    commands.spawn((
        Text2d::new(PAIR_CHARS[(pair_id as usize) % PAIR_CHARS.len()].to_string()),
        TextFont::from_font_size(size * 0.55).with_font(font.0.clone()),
        TextColor(pair_color(pair_id)),
        Transform::from_xyz(0.0, 0.0, 0.2),
        Visibility::Hidden,
        CardFace,
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

fn spawn_hud(commands: &mut Commands, font: &UiFont, level: u8) {
    let hud_x = 0.0;
    let hud_y = 230.0;
    panel(
        commands,
        Vec2::new(hud_x, hud_y),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.10, 0.13, 0.20),
        Color::srgb(0.45, 0.75, 0.96),
        GameEntity,
    );
    text(
        commands,
        font,
        &format!("记忆翻翻乐 · 第 {} 关", level),
        Vec2::new(-350.0, hud_y),
        20.0,
        Color::srgb(1.0, 0.96, 0.78),
        GameEntity,
    );
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(15.0).with_font(font.0.clone()),
        TextColor(Color::srgb(0.86, 0.94, 1.0)),
        Transform::from_translation(Vec3::new(160.0, hud_y, 10.0)),
        MemoryHud,
        GameEntity,
    ));

    // 底部操作提示 + 消息条
    panel(
        commands,
        Vec2::new(0.0, -230.0),
        Vec2::new(900.0, 50.0),
        Color::srgb(0.08, 0.10, 0.15),
        Color::srgb(0.96, 0.72, 0.32),
        GameEntity,
    );
    text(
        commands,
        font,
        "方向键 / WASD 移动光标，Enter / Space 翻牌；Esc 暂停，Backspace 返回",
        Vec2::new(-330.0, -230.0),
        13.0,
        Color::srgb(0.86, 0.92, 1.0),
        GameEntity,
    );
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(18.0).with_font(font.0.clone()),
        TextColor(Color::srgb(1.0, 0.86, 0.42)),
        Transform::from_translation(Vec3::new(280.0, -230.0, 10.0)),
        MemoryMessage,
        GameEntity,
    ));

}
