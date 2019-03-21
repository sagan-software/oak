use oak_core::Cmd;

pub trait Updater<Model, Msg> {
    fn update(&self, model: Model, msg: Msg) -> Model;
}

impl<Model, Msg> Updater<Model, Msg> for () {
    fn update(&self, model: Model, _: Msg) -> Model {
        model
    }
}

impl<Model, Msg, T> Updater<Model, Msg> for T
where
    T: Fn(Model, Msg) -> Model,
{
    fn update(&self, model: Model, msg: Msg) -> Model {
        (self)(model, msg)
    }
}

// impl<Model, Msg, C> Updater<Model, Msg> for fn(Model, Msg) -> (Model, C)
// where
//     C: Cmd<Msg>,
// {
//     fn update(&self, model: Model, msg: Msg) -> Model {
//         (self)(model, msg).0
//     }
// }
