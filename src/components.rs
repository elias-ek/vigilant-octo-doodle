use crate::types::PlayerId;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
}
