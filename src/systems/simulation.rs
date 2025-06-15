use crate::components::*;
use crate::resources::*;
use crate::types::PlayerInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn simulate_frame(
    mut events: EventReader<AdvanceFrameEvent>,
    input_buffer: Res<InputBuffer>,
    mut command: Commands,
    mut query: Query<(&mut KinematicCharacterController, &Player)>,
) {
    for event in events.read() {
        let frame_inputs = &input_buffer.inputs[&event.0];
        for (player_id, input) in frame_inputs {
            for (mut controller, player) in query.iter_mut() {
                if player.id == *player_id {
                    // Apply input deterministically
                    let translation = match *input {
                        PlayerInput::Up => Vec2::new(0.0, 100.0),
                        PlayerInput::Down => Vec2::new(0.0, -10.0),
                        PlayerInput::Right => Vec2::new(10.0, 0.0),
                        PlayerInput::Left => Vec2::new(-10.0, 0.0),
                        _ => Vec2::new(0.0, 0.0),
                    };
                    controller.translation = Some(translation);
                    // TODO: mutate ECS world state here deterministically
                    break; // found the player, no need to keep looking
                }
            }
        }
    }
}
