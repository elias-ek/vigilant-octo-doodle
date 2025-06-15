use crate::types::PlayerInput;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct LockstepFrame {
    pub current_frame: u32,
}

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub inputs: HashMap<u32, HashMap<u32, PlayerInput>>, // frame -> player_id -> input
}

#[derive(Resource)]
pub struct PlayerIds(pub Vec<u32>);

#[derive(Event)]
pub struct AdvanceFrameEvent(pub u32);

#[derive(Resource)]
pub struct KeyBinds {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub shoot: KeyCode,
    pub dodge: KeyCode,
    pub accept: KeyCode,
    pub cancel: KeyCode,
}

impl KeyBinds {
    pub fn default() -> KeyBinds {
        KeyBinds {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            shoot: KeyCode::KeyJ,
            dodge: KeyCode::KeyK,
            accept: KeyCode::KeyE,
            cancel: KeyCode::Backspace,
        }
    }
}
