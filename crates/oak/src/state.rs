use std::ops::Deref;

use shrev::{EventChannel, ReaderId};
use specs::prelude::{Component, DenseVecStorage, Entities, Join, Read, ReadStorage, Resources, System, SystemData, Write, WriteStorage};

/// A trait which defines game states that can be used by the state machine.
pub trait State<Msg: Send + Sync + 'static>: std::fmt::Debug {
    /// Executed on every frame before updating, for use in reacting to events.
    fn update(&mut self, _msg: &Msg) {}
}

pub struct Stateful<Msg: Send + Sync + 'static, S: State<Msg>, C: Component> {
    pub func: Box<dyn Fn(&S) -> C + Send + Sync>,
    pub phantom: std::marker::PhantomData<Msg>,
}

impl<Msg: Send + Sync + 'static, S: State<Msg> + 'static, C: Component> Component for Stateful<Msg, S, C> {
    type Storage = DenseVecStorage<Self>;
}

pub struct StatefulSystem<Msg: Send + Sync + 'static, S: State<Msg> + 'static, C: Component> {
    pub phantom: std::marker::PhantomData<(Msg, S, C)>,
}

impl<'a, Msg, S, C> System<'a> for StatefulSystem<Msg, S, C>
    where
        Msg: Send + Sync + 'static,
        S: State<Msg> + Send + Sync + Default + std::fmt::Debug + 'static,
        C: Component {
    type SystemData = (
        Entities<'a>,
        Read<'a, S>,
        ReadStorage<'a, Stateful<Msg, S, C>>,
        WriteStorage<'a, C>
    );

    fn run(&mut self, (entities, state, stateful_components, mut inner_components): Self::SystemData) {
        for (entity, stateful) in (&entities, &stateful_components).join() {
            let new_inner_component = (stateful.func)(&state);
            inner_components.insert(entity, new_inner_component);
        }
    }
}

pub struct StateUpdater<Msg: Send + Sync + 'static, S: State<Msg> + 'static> {
    pub reader_id: Option<ReaderId<Msg>>,
    pub phantom: std::marker::PhantomData<(Msg, S)>,
}


impl<'a, Msg, S> System<'a> for StateUpdater<Msg, S>
    where
        Msg: Send + Sync + 'static,
        S: State<Msg> + Send + Sync + Default + std::fmt::Debug + 'static {
    type SystemData = (
        Read<'a, EventChannel<Msg>>,
        Write<'a, S>,
    );

    fn run(&mut self, (msgs, mut state): Self::SystemData) {
        let reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for msg in msgs.read(reader_id) {
            state.update(&msg);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res.try_fetch_mut::<EventChannel<Msg>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<Msg>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}