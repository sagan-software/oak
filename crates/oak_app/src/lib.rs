use oak_core::{
    Component, DenseVecStorage, Entities, EventChannel, Join, Read, ReadStorage, ReaderId,
    Resources, System, SystemData, Write, WriteStorage,
};
use oak_events::Handler;
use std::ops::Deref;

pub struct Stateful<Msg: Send + Sync + 'static, S: Handler<Msg>, C: Component> {
    pub func: Box<dyn Fn(&S) -> C + Send + Sync>,
    pub phantom: std::marker::PhantomData<Msg>,
}

impl<Msg: Send + Sync + 'static, S: Handler<Msg> + 'static, C: Component> Component
    for Stateful<Msg, S, C>
{
    type Storage = DenseVecStorage<Self>;
}

pub struct StatefulSystem<Msg: Send + Sync + 'static, S: Handler<Msg> + 'static, C: Component> {
    pub phantom: std::marker::PhantomData<(Msg, S, C)>,
}

impl<'a, Msg, S, C> System<'a> for StatefulSystem<Msg, S, C>
where
    Msg: Send + Sync + 'static,
    S: Handler<Msg> + Send + Sync + Default + std::fmt::Debug + 'static,
    C: Component,
{
    type SystemData = (
        Entities<'a>,
        Read<'a, S>,
        ReadStorage<'a, Stateful<Msg, S, C>>,
        WriteStorage<'a, C>,
    );

    fn run(
        &mut self,
        (entities, state, stateful_components, mut inner_components): Self::SystemData,
    ) {
        for (entity, stateful) in (&entities, &stateful_components).join() {
            let new_inner_component = (stateful.func)(&state);
            inner_components
                .insert(entity, new_inner_component)
                .unwrap();
        }
    }
}
