use bevy::{input::keyboard::KeyCode, prelude::*};
use(crate) types;

struct KeyBinds {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    shoot: KeyCode,
    dodge: KeyCode,
    accept: KeyCode,
    cancel: KeyCode,
}

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

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, binds: KeyBinds) {
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
}
