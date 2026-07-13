use bevy::prelude::*;

use crate::common::input::ActionState;
use crate::common::settings::{InputAction, PlayerSlot};
use crate::game::model::{GameKind, GameSession};

use super::super::components::{ModeSelectUi, P2Hud};
use super::super::resources::TankStage;
use super::super::setup::spawn_initial_players_for_mode;

pub fn tank_mode_select(
    mut commands: Commands,
    actions: Res<ActionState>,
    session: Res<GameSession>,
    mut stage: ResMut<TankStage>,
    ui: Query<Entity, With<ModeSelectUi>>,
    p2_hud: Query<Entity, With<P2Hud>>,
) {
    if session.kind != GameKind::Tank || session.paused || session.finished || stage.mode_selected
    {
        return;
    }
    let two_player = if actions.just_pressed(PlayerSlot::One, InputAction::Primary) {
        false
    } else if actions.just_pressed(PlayerSlot::One, InputAction::Secondary) {
        true
    } else {
        return;
    };

    stage.two_player = two_player;
    stage.mode_selected = true;
    if !two_player {
        // 单人模式：P2 永不参战，命数清零，避免触发 respawn 与“全员阵亡”分支
        stage.p2_lives = 0;
        stage.p2_respawn = -1.0;
        for e in &p2_hud {
            commands.entity(e).despawn();
        }
    }
    for e in &ui {
        commands.entity(e).despawn();
    }
    spawn_initial_players_for_mode(&mut commands, two_player);
}

pub fn tank_playing(stage: Option<Res<TankStage>>) -> bool {
    stage.map(|s| s.mode_selected).unwrap_or(false)
}
