use bevy::prelude::*;
mod components;
mod level;
mod resources;
mod systems;
mod types;

use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use components::Player;
use level::*;
use resources::*;
use systems::{frame_advance::*, input_collection::*, simulation::*};

fn main() {
    App::new()
        .insert_resource(LockstepFrame { current_frame: 0 })
        .insert_resource(InputBuffer::default())
        .insert_resource(KeyBinds::default())
        .insert_resource(PlayerIds(vec![0])) // Simplified: local + 1 remote player
        .insert_resource(LevelSelection::index(0))
        .add_event::<AdvanceFrameEvent>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, level_setup)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, collect_local_input)
        .add_systems(Update, collect_remote_inputs)
        .add_systems(Update, advance_simulation_if_ready)
        .add_systems(Update, simulate_frame)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Player { id: 0 })
        .insert(Velocity::zero())
        .insert(Collider::cuboid(16.0, 16.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform::from_xyz(-100.0, 100.0, 0.0));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.0))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
