use oak_core::Cmd;

pub trait Initializer<Model, Msg> {
    fn init(self) -> Model;
}

impl<Msg> Initializer<(), Msg> for () {
    fn init(self) {}
}

impl<Model, Msg, T> Initializer<Model, Msg> for T
where
    T: Fn() -> Model,
{
    fn init(self) -> Model {
        (self)()
    }
}

impl<Msg> Initializer<i32, Msg> for i32 {
    fn init(self) -> Self {
        self
    }
}
