use crate::types::PlayerId;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Component, LdtkEntity, Default)]
pub struct Player {
    pub id: PlayerId,
}
