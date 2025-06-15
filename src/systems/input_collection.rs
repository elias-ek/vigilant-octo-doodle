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

fn keyboard_input(keys: ButtonInput<KeyCode>, binds: KeyBinds) -> PlayerInput {
    if keys.just_pressed(binds.up) {
        // send jump/menu up action
    }

    if keys.just_pressed(binds.down) {
        //send crouch/menu down action
    }

    if keys.just_pressed(binds.left) {
        // send move/menu left action
    }

    if keys.just_pressed(binds.right) {
        // send move/menu right action
    }

    if keys.just_pressed(binds.shoot) {
        // send shoot action
    }

    if keys.just_pressed(binds.dodge) {
        // send dodge action
    }

    if keys.just_pressed(binds.accept) {
        // send menu accept action
    }

    if keys.just_pressed(binds.cancel) {
        // send menu cancel action
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

pub fn collect_remote_inputs() {
    return;
}
