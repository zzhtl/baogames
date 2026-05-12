use bevy::prelude::*;

use crate::game::model::{GameKind, GameSession};

use super::super::components::{BubbleColor, NextBubbleSprite, palette};
use super::super::resources::BubbleStage;

pub fn bubble_next_preview_update(
    session: Res<GameSession>,
    stage: Res<BubbleStage>,
    mut q: Query<(&mut Sprite, &mut BubbleColor), With<NextBubbleSprite>>,
) {
    if session.kind != GameKind::BubbleBobble {
        return;
    }
    for (mut sp, mut bc) in &mut q {
        if bc.0 != stage.next {
            bc.0 = stage.next;
            sp.color = palette(stage.next);
        }
    }
}
