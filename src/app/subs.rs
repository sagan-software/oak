use crate::idle::Idle;
use crate::sub::Sub;

pub trait Subscriber<Model, Msg, S>
where
    S: Sub<Msg>,
{
    fn subs(&self, model: &Model) -> S;
}

impl<Model, Msg> Subscriber<Model, Msg, Idle<Msg>> for () {
    fn subs(&self, _: &Model) -> Idle<Msg> {
        Idle::new()
    }
}

impl<Model, Msg, S, T> Subscriber<Model, Msg, S> for T
where
    T: Fn(&Model) -> S,
    S: Sub<Msg>,
{
    fn subs(&self, model: &Model) -> S {
        (self)(model)
    }
}
