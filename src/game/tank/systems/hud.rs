use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::TankHud;
use super::super::constants::STAGE_TOTAL_ENEMIES;
use super::super::resources::TankStage;

pub fn tank_hud_update(
    session: Res<GameSession>,
    stage: Option<Res<TankStage>>,
    mut hud: Query<&mut Text2d, With<TankHud>>,
) {
    if session.kind != GameKind::Tank {
        return;
    }
    let Some(stage) = stage else { return };
    if let Ok(mut text) = hud.single_mut() {
        let remaining = STAGE_TOTAL_ENEMIES - stage.kills;
        **text = format!("{}", remaining);
    }
}
