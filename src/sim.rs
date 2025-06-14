#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move,
    Jump,
}

#[derive(Clone, Copy, Debug)]
pub struct Actor {}

/// this state is intended to be the absolute descriptor of the gamestate
/// it should not be mutated by anything else but itself
pub struct Simulation {
    action_history: Vec<Vec<Action>>,
    actors: Vec<Actor>,
    player_count: usize,

    current_frame: u64,
}

impl Simulation {
    /// advances one frame by applying all actions.
    /// actions is order-sensetive and should relate to the actor via index
    ///
    /// NOTE: in non-optimized builds:
    ///     this procedure also may panic if the state at the point of invocation is
    ///     invalid or dangerous.
    ///
    /// this function is pure and shall not become impure
    pub fn step(self: &mut Self, actions: Vec<Action>) {
        debug_assert!(!actions.is_empty());
        debug_assert_eq!(actions.len(), self.player_count);
        // if this is raised, there is an implication that we have skipped actions, or frames
        debug_assert_eq!(self.action_history.len(), self.current_frame as usize);
        // if this is raised, there is an implication that we are missing actors / players
        debug_assert_eq!(self.actors.len(), self.player_count);

        self.action_history.push(actions.to_vec());

        for i in 0..self.player_count {
            let command = actions[i];
            let actor = self.actors[i];
            self.apply_command(command, actor);
        }

        self.current_frame += 1;
    }

    fn apply_command(self: &mut Self, action: Action, actor: Actor) {
        _ = actor;
        match action {
            Action::Move => {}
            Action::Jump => {}
        }
    }

    pub fn save_action_history(self: &Self) {
        let mut action_history = self.action_history.to_vec();
        action_history.shrink_to_fit();

        // TODO not yet impliment
        debug_assert!(false)
    }

    pub fn init() -> Self {
        Simulation {
            action_history: Vec::<Vec<Action>>::new(),
            actors: Vec::<Actor>::new(),
            player_count: 0,

            current_frame: 0,
        }
    }

    pub fn reset(self: &mut Self) {
        self.action_history.clear();
        self.actors.clear();
        self.current_frame = 0;
        self.player_count = 0;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn init() {
        let sim = Simulation::init();
        debug_assert!(sim.action_history.is_empty());
        debug_assert!(sim.actors.is_empty());

        debug_assert_eq!(sim.current_frame, 0);
        debug_assert_eq!(sim.player_count, 0);
    }

    #[test]
    fn step() {
        let mut sim = Simulation::init();
        sim.actors.push(Actor {});
        sim.actors.push(Actor {});
        sim.player_count = 2;

        let mut actions_this_frame = Vec::<Action>::new();
        actions_this_frame.push(Action::Move); // player 1
        actions_this_frame.push(Action::Jump); // player 2
        sim.step(actions_this_frame);

        debug_assert_eq!(sim.current_frame, 1);
        debug_assert_eq!(sim.action_history[0][0], Action::Move);
        debug_assert_eq!(sim.action_history[0][1], Action::Jump);
    }

    #[test]
    fn reset() {
        let mut sim = Simulation::init();
        sim.actors.push(Actor {});
        sim.actors.push(Actor {});
        sim.player_count = 2;

        let mut actions_this_frame = Vec::<Action>::new();
        actions_this_frame.push(Action::Move); // player 1
        actions_this_frame.push(Action::Jump); // player 2
        sim.step(actions_this_frame);

        debug_assert_eq!(sim.current_frame, 1);
        debug_assert_eq!(sim.action_history[0][0], Action::Move);
        debug_assert_eq!(sim.action_history[0][1], Action::Jump);

        sim.reset();

        debug_assert!(sim.action_history.is_empty());
        debug_assert!(sim.actors.is_empty());

        debug_assert_eq!(sim.current_frame, 0);
        debug_assert_eq!(sim.player_count, 0);
    }

    #[test]
    #[should_panic]
    /// failing for now as this is not implemented yet
    fn save_action_history() {
        let mut sim = Simulation::init();
        sim.actors.push(Actor {});
        sim.actors.push(Actor {});
        sim.player_count = 2;

        let mut actions_this_frame = Vec::<Action>::new();
        actions_this_frame.push(Action::Move); // player 1
        actions_this_frame.push(Action::Jump); // player 2
        sim.step(actions_this_frame);

        debug_assert_eq!(sim.current_frame, 1);
        debug_assert_eq!(sim.action_history[0][0], Action::Move);
        debug_assert_eq!(sim.action_history[0][1], Action::Jump);

        sim.save_action_history();
    }
}
