use bevy::prelude::*;
mod resources;
mod systems;
mod types;

use resources::*;
use systems::{frame_advance::*, input_collection::*, simulation::*};

fn main() {
    App::new()
        .insert_resource(LockstepFrame { current_frame: 0 })
        .insert_resource(InputBuffer::default())
        .insert_resource(PlayerIds(vec![0, 1])) // Simplified: local + 1 remote player
        .add_event::<AdvanceFrameEvent>()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, collect_local_input)
        .add_systems(Update, collect_remote_inputs)
        .add_systems(Update, advance_simulation_if_ready)
        .add_systems(Update, simulate_frame)
        .run();
}
