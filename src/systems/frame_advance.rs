use crate::resources::*;
use bevy::prelude::*;

pub fn advance_simulation_if_ready(
    mut frame: ResMut<LockstepFrame>,
    input_buffer: Res<InputBuffer>,
    player_ids: Res<PlayerIds>,
    mut next_frame_events: EventWriter<AdvanceFrameEvent>,
) {
    let inputs = input_buffer.inputs.get(&frame.current_frame);
    if let Some(map) = inputs {
        if map.len() == player_ids.0.len() {
            next_frame_events.send(AdvanceFrameEvent(frame.current_frame));
            frame.current_frame += 1;
        }
    }
}
