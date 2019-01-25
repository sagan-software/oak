use specs::prelude::{Dispatcher, World};

use crate::state::{State, StateMachine};

pub struct Application<'a, 'b, Model, Msg> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    states: StateMachine<'a, Model, Msg>,
}

impl<'a, 'b, Model, Msg> Application<'a, 'b, Model, Msg>
    where
        Model: State<Model, Msg> + 'static,
        Msg: Send + Sync + 'static
{
    /// Create new app data
    pub fn new(init: Model, dispatcher: Dispatcher<'a, 'b>) -> Self {
        let mut world = World::new();
        Self {
            world,
            dispatcher,
            states: StateMachine::new(init),
        }
    }

    pub fn update(&mut self, world: &World) {
        self.dispatcher.dispatch(&world.res);
    }
}