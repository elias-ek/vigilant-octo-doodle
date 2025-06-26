use bevy::prelude::*;
mod components;
mod level;
mod level_select;
mod player;
mod resources;
mod states;
mod systems;
mod types;

use bevy_ecs_ldtk::prelude::*;
use level::*;
use level_select::*;
use resources::*;
use states::AppState;
use systems::{frame_advance::*, input_collection::*, simulation::*};

fn main() {
    App::new()
        .insert_resource(LockstepFrame { current_frame: 0 })
        .insert_resource(InputBuffer::default())
        .insert_resource(KeyBinds::default())
        .insert_resource(PlayerIds(vec![0])) // Simplified: local + 1 remote player
        .add_event::<AdvanceFrameEvent>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::LevelSelect), select_setup)
        .add_systems(OnEnter(AppState::InGame), level_setup)
        .add_systems(Update, collect_local_input)
        .add_systems(
            Update,
            advance_simulation_if_ready.run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, simulate_frame.run_if(in_state(AppState::InGame)))
        .add_systems(Update, scale_cam.run_if(in_state(AppState::InGame)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.set_state(AppState::LevelSelect);
}
