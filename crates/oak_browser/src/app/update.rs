pub trait Updater<Model, Msg> {
    fn update(&self, model: Model, msg: Msg) -> Model;
}

impl<Msg> Updater<(), Msg> for () {
    fn update(&self, _: (), _: Msg) {}
}

impl<Model, Msg, T> Updater<Model, Msg> for T
where
    T: Fn(Model, Msg) -> Model,
{
    fn update(&self, model: Model, msg: Msg) -> Model {
        (self)(model, msg)
    }
}