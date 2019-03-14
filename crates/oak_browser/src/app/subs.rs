use oak_core::Sub;

pub trait Subscriber<Model, Msg, S> where S: Sub<Msg> {
    fn subs(&self, model: &Model) -> S;
}

impl<Msg> Subscriber<(), Msg> for () {
    fn subs(&self, _: ()) {}
}