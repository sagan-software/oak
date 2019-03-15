use oak_core::Sub;

pub trait Subscriber<Model, Msg> {
    fn subs(&self, model: &Model);
}

impl<Model, Msg> Subscriber<Model, Msg> for () {
    fn subs(&self, _: &Model) {}
}
