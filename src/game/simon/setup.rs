use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::common::constants::FONT_BODY;
use crate::common::px::{px, snap2};
use crate::common::render::{UiFont, rect};
use crate::common::theme::{ACCENT, BORDER, SURFACE, TEXT_MUTED, TEXT_PRIMARY};
use crate::game::hud::{hud_bar, hud_text_anchored};
use crate::game::model::{AgeTier, GameEntity};
use crate::game::puzzle::PuzzleControls;

use super::components::*;
use super::constants::*;
use super::palette;
use super::resources::{SimonPhase, SimonStage, build_sequence};

const HUD_BORDER: Color = Color::srgb(0.98, 0.42, 0.66);
/// 音板顺序固定为 上 / 下 / 左 / 右，和方向键一一对应。
pub const PAD_OFFSETS: [(f32, f32); 4] = [
    (0.0, PAD_OFFSET),
    (0.0, -PAD_OFFSET),
    (-PAD_OFFSET, 0.0),
    (PAD_OFFSET, 0.0),
];

pub fn setup_stage(
    commands: &mut Commands,
    font: &UiFont,
    hud_root: Entity,
    level: u8,
    age: AgeTier,
) {
    commands.insert_resource(PuzzleControls::default());

    let sequence = build_sequence(start_len_for(level, age), PAD_OFFSETS.len(), &mut rand::thread_rng());

    for (pad, (dx, dy)) in PAD_OFFSETS.iter().copied().enumerate() {
        let center = snap2(Vec2::new(px(dx), px(dy)));
        rect(commands, center, Vec2::splat(px(PAD_SIZE)), palette::PAD_EDGE, GameEntity);
        commands.spawn((
            Sprite::from_color(
                palette::PAD_DIM[pad],
                Vec2::splat(px(PAD_SIZE) - BORDER * 2.0),
            ),
            Transform::from_translation(center.extend(0.1)),
            SimonPad { pad },
            GameEntity,
        ));
    }
    // 中央方块只做呼吸提示：亮 = 该你按了，暗 = 看着就好。
    rect(commands, Vec2::ZERO, Vec2::splat(px(CORE_SIZE)), palette::CORE_EDGE, GameEntity);
    commands.spawn((
        Sprite::from_color(palette::CORE, Vec2::splat(px(CORE_SIZE)) - Vec2::splat(BORDER * 2.0)),
        Transform::from_xyz(0.0, 0.0, 0.1),
        SimonCore,
        GameEntity,
    ));

    spawn_hud(commands, font, hud_root);

    commands.insert_resource(SimonStage {
        sequence,
        phase: SimonPhase::Ready,
        clock: READY_TIME,
        play_index: 0,
        input_index: 0,
        lit: None,
        lit_clock: 0.0,
        round: 1,
        rounds_total: rounds_for(level),
        reverse: reverse_for(level, age),
        step_time: step_time_for(level, age),
        input_time: input_time_for(age),
        retry: false,
        failed: false,
        message: "看好顺序".to_string(),
    });
}

fn spawn_hud(commands: &mut Commands, font: &UiFont, hud_root: Entity) {
    hud_bar(commands, hud_root, 82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "记忆序列",
        Vec2::new(px(-113.0), px(82.0)), FONT_BODY, TEXT_PRIMARY, Anchor::CENTER_LEFT, (),
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SimonHud,
    );

    hud_bar(commands, hud_root, -82.0, SURFACE, HUD_BORDER);
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(-113.0), px(-82.0)),
        FONT_BODY, TEXT_MUTED, Anchor::CENTER_LEFT, SimonScoreHud,
    );
    hud_text_anchored(
        commands, font, hud_root, "", Vec2::new(px(113.0), px(-82.0)),
        FONT_BODY, ACCENT, Anchor::CENTER_RIGHT, SimonMessage,
    );
}
