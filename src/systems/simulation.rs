use crate::resources::*;
use crate::types::PlayerInput;
use bevy::prelude::*;

pub fn simulate_frame(mut events: EventReader<AdvanceFrameEvent>, input_buffer: Res<InputBuffer>) {
    for event in events.read() {
        let frame_inputs = &input_buffer.inputs[&event.0];
        for (player_id, input) in frame_inputs {
            println!("Simulating player {} with input {:?}", player_id, input);
            // TODO: mutate ECS world state here deterministically
        }
    }
}
