#[cfg(feature = "browser")]
mod browser;
#[cfg(feature = "browser")]
pub use crate::browser::*;

use oak_core::{EventChannel, ReaderId,Component, Read, Resources, System, SystemData, VecStorage, Write};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Event<T: Debug> {
    pub name: String,
    pub event: T,
}

pub struct EventListener<In, Out> {
    name: String,
    func: Box<dyn Fn(&In) -> Out + Send + Sync>,
}

impl<In, Out> EventListener<In, Out> {
    pub fn new<F: Fn(&In) -> Out + Send + Sync + 'static>(name: &str, func: F) -> Self {
        Self {
            name: name.to_owned(),
            func: Box::new(func),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self, i: &In) -> Out {
        (self.func)(i)
    }
}

pub struct EventListeners<In, Out>(pub Vec<EventListener<In, Out>>);

impl<In: 'static, Out: 'static> Component for EventListeners<In, Out> {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct LogSystem<T: Debug + 'static> {
    reader_id: Option<ReaderId<T>>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Debug + 'static> LogSystem<T> {
    pub fn new() -> Self {
        Self {
            reader_id: None,
            phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl<'a, T: Send + Sync + Debug + 'static> System<'a> for LogSystem<T> {
    type SystemData = Read<'a, EventChannel<T>>;

    fn run(&mut self, msgs: Self::SystemData) {
        let reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for msg in msgs.read(reader_id) {
            log::info!("{:#?}", msg);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res
            .try_fetch_mut::<EventChannel<T>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<T>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}

/// A trait which defines game states that can be used by the state machine.
pub trait Handler<Msg: Send + Sync + 'static>: std::fmt::Debug {
    /// Executed on every frame before updating, for use in reacting to events.
    fn handle(&mut self, _msg: &Msg) {}
}

pub struct DispatchSystem<Msg: Send + Sync + 'static, S: Handler<Msg> + 'static> {
    pub reader_id: Option<ReaderId<Msg>>,
    pub phantom: std::marker::PhantomData<(Msg, S)>,
}

impl<'a, Msg, S> System<'a> for DispatchSystem<Msg, S>
where
    Msg: Send + Sync + 'static,
    S: Handler<Msg> + Send + Sync + Default + std::fmt::Debug + 'static,
{
    type SystemData = (Read<'a, EventChannel<Msg>>, Write<'a, S>);

    fn run(&mut self, (msgs, mut state): Self::SystemData) {
        let reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for msg in msgs.read(reader_id) {
            state.handle(&msg);
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res
            .try_fetch_mut::<EventChannel<Msg>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<Msg>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}
