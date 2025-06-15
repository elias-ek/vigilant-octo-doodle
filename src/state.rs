use bevy::ecs::resource::Resource;

/// this state is intended to be the absolute descriptor of the gamestate
#[derive(Resource)]
pub struct State {}

impl State {
    pub fn init() -> Self {
        State {}
    }
}
