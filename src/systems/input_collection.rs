use crate::{resources::*, types::PlayerInput};
use bevy::prelude::*;

fn bind_key(keys: Res<ButtonInput<KeyCode>>, mut binds: KeyBinds, in_type: char) -> KeyBinds {
    for key in keys.get_just_pressed() {
        match in_type {
            'u' => binds.up = *key,
            'd' => binds.down = *key,
            'l' => binds.left = *key,
            'r' => binds.right = *key,
            's' => binds.shoot = *key,
            'D' => binds.dodge = *key,
            'a' => binds.accept = *key,
            'c' => binds.cancel = *key,
            _ => panic!("Erm, what the sigma?!"),
        }
    }

    binds
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, binds: Res<KeyBinds>) -> PlayerInput {
    if keys.just_pressed(binds.up) {
        return PlayerInput::Up;
    }

    if keys.pressed(binds.down) {
        return PlayerInput::Down;
    }

    if keys.pressed(binds.left) {
        return PlayerInput::Left;
    }

    if keys.pressed(binds.right) {
        return PlayerInput::Right;
    }

    if keys.just_pressed(binds.shoot) {
        return PlayerInput::Shoot;
    }

    if keys.just_pressed(binds.dodge) {
        return PlayerInput::Dodge;
    }

    if keys.just_pressed(binds.accept) {
        return PlayerInput::Accept;
    }

    if keys.just_pressed(binds.cancel) {
        return PlayerInput::Cancel;
    }

    PlayerInput::Noop
}

// Simplified local input system
pub fn collect_local_input(
    keys: Res<ButtonInput<KeyCode>>,
    binds: Res<KeyBinds>,
    mut input_buffer: ResMut<InputBuffer>,
    frame: Res<LockstepFrame>,
) {
    let input = keyboard_input(keys, binds);

    input_buffer
        .inputs
        .entry(frame.current_frame)
        .or_default()
        .insert(0, input); // assuming player_id 0 is local

    // TODO: send to remote players via networking
}
