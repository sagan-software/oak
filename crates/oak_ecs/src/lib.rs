pub mod browser;
pub mod vdom;

pub type Entity = generational_arena::Index;

pub type EntityAllocator = generational_arena::Arena<usize>;

pub trait Component {}

pub trait System {
    type Data;
    fn run(&mut self, data: Self::Data);
}

pub struct World {
    pub entity_allocator: EntityAllocator,
    pub entities: Vec<Entity>,
}
