use bevy::prelude::*;

pub struct PlayerInput {
    pub move_dir: Vec2,
    pub fire: bool,
    pub jump: bool,
}

pub fn input_for(keys: &ButtonInput<KeyCode>, player: usize) -> PlayerInput {
    let mut move_dir = Vec2::ZERO;
    if player == 0 {
        if keys.pressed(KeyCode::KeyA) {
            move_dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            move_dir.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyW) {
            move_dir.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.0;
        }
        PlayerInput {
            move_dir,
            fire: keys.pressed(KeyCode::KeyJ),
            jump: keys.just_pressed(KeyCode::KeyK) || keys.just_pressed(KeyCode::Space),
        }
    } else {
        if keys.pressed(KeyCode::ArrowLeft) {
            move_dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            move_dir.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            move_dir.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            move_dir.y -= 1.0;
        }
        PlayerInput {
            move_dir,
            fire: keys.pressed(KeyCode::Numpad1),
            jump: keys.just_pressed(KeyCode::Numpad2) || keys.just_pressed(KeyCode::ShiftRight),
        }
    }
}
