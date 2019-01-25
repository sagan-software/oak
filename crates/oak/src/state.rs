use std::fmt::{Display, Formatter, Result as FmtResult};

use specs::prelude::World;

/// Error type for errors occurring in StateMachine
#[derive(Debug)]
pub enum StateError {
    NoStatesPresent,
}

impl Display for StateError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match *self {
            StateError::NoStatesPresent => write!(
                fmt,
                "Tried to start state machine without any states present"
            ),
        }
    }
}

/// State data encapsulates the data sent to all state functions from the application main loop.
pub struct StateData<'a, Model> {
    /// Main `World`
    pub world: &'a mut World,
    /// User defined game data
    pub model: &'a mut Model,
}

impl<'a, Model> StateData<'a, Model>
    where
        Model: 'a,
{
    /// Create a new state data
    pub fn new(world: &'a mut World, model: &'a mut Model) -> Self {
        StateData { world, model }
    }
}

/// Types of state transitions.
/// T is the type of shared data between states.
/// E is the type of events
pub enum Trans<Model, Msg> {
    /// Continue as normal.
    None,
    /// Remove the active state and resume the next state on the stack or stop
    /// if there are none.
    Pop,
    /// Pause the active state and push a new state onto the stack.
    Push(Box<dyn State<Model, Msg>>),
    /// Remove the current state on the stack and insert a different one.
    Switch(Box<dyn State<Model, Msg>>),
    /// Stop and remove all states and shut down the engine.
    Quit,
}

/// Event queue to trigger state `Trans` from other places than a `State`'s methods.
/// # Example:
/// ```rust, ignore
/// world.write_resource::<EventChannel<TransEvent<MyAppData, StateEvent>>>().single_write(Box::new(|| Trans::Quit));
/// ```
///
/// Transitions will be executed sequentially by Amethyst's `CoreApplication` update loop.
pub type TransEvent<Model, Msg> = Box<dyn Fn() -> Trans<Model, Msg> + Send + Sync + 'static>;

/// An empty `Trans`. Made to be used with `EmptyState`.
pub type EmptyTrans = Trans<(), ()>;

/// A trait which defines game states that can be used by the state machine.
pub trait State<Model, Msg: Send + Sync + 'static> {
    /// Executed when the game state begins.
    fn on_start(&mut self, _data: StateData<'_, Model>) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self, _data: StateData<'_, Model>) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _data: StateData<'_, Model>) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _data: StateData<'_, Model>) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_msg(&mut self, _data: StateData<'_, Model>, _msg: Msg) -> Trans<Model, Msg> {
        Trans::None
    }
}

/// A simple stack-based state machine (pushdown automaton).
pub struct StateMachine<'a, Model, Msg> {
    running: bool,
    state_stack: Vec<Box<dyn State<Model, Msg> + 'a>>,
}

impl<'a, Model, Msg: Send + Sync + 'static> StateMachine<'a, Model, Msg> {
    /// Creates a new state machine with the given initial state.
    pub fn new<S: State<Model, Msg> + 'a>(initial_state: S) -> StateMachine<'a, Model, Msg> {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    /// Checks whether the state machine is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initializes the state machine.
    pub fn start(&mut self, data: StateData<'_, Model>) -> Result<(), StateError> {
        if !self.running {
            let state = self
                .state_stack
                .last_mut()
                .ok_or(StateError::NoStatesPresent)?;
            state.on_start(data);
            self.running = true;
        }
        Ok(())
    }

    /// Passes a single event to the active state to handle.
    pub fn handle_event(&mut self, data: StateData<'_, Model>, msg: Msg) {
        let StateData { world, model } = data;
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_msg(StateData { world, model }, msg),
                None => Trans::None,
            };

            self.transition(trans, StateData { world, model });
        }
    }

    /// Performs a state transition.
    /// Usually called by update or fixed_update by the user's defined `State`.
    /// This method can also be called when there are one or multiple `Trans` stored in the
    /// global `EventChannel<TransEvent<T, E>>`. Such `Trans` will be passed to this method
    /// sequentially in the order of insertion.
    pub fn transition(&mut self, request: Trans<Model, Msg>, data: StateData<'_, Model>) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(data),
                Trans::Push(state) => self.push(state, data),
                Trans::Switch(state) => self.switch(state, data),
                Trans::Quit => self.stop(data),
            }
        }
    }

    /// Removes the current state on the stack and inserts a different one.
    fn switch(&mut self, state: Box<dyn State<Model, Msg>>, data: StateData<'_, Model>) {
        if self.running {
            let StateData { world, model } = data;
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(StateData { world, model });
            }

            self.state_stack.push(state);

            // State was just pushed, thus pop will always succeed
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(StateData { world, model });
        }
    }

    /// Pauses the active state and pushes a new state onto the state stack.
    fn push(&mut self, state: Box<dyn State<Model, Msg>>, data: StateData<'_, Model>) {
        if self.running {
            let StateData { world, model } = data;
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(StateData { world, model });
            }

            self.state_stack.push(state);

            //State was just pushed, thus pop will always succeed
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(StateData { world, model });
        }
    }

    /// Stops and removes the active state and un-pauses the next state on the
    /// stack (if any).
    fn pop(&mut self, data: StateData<'_, Model>) {
        if self.running {
            let StateData { world, model } = data;
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(StateData { world, model });
            }

            if let Some(state) = self.state_stack.last_mut() {
                state.on_resume(StateData { world, model });
            } else {
                self.running = false;
            }
        }
    }

    /// Shuts the state machine down.
    pub(crate) fn stop(&mut self, data: StateData<'_, Model>) {
        if self.running {
            let StateData { world, model } = data;
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(StateData { world, model });
            }

            self.running = false;
        }
    }
}