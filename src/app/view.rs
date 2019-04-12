use crate::vdom::{Element as VirtualElement, Node as VirtualNode};

pub trait Viewer<Model, Msg> {
    fn view(&self, model: &Model) -> VirtualNode<Msg>;
}

impl<Model, Msg> Viewer<Model, Msg> for () {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text("".to_owned())
    }
}

impl<Model, Msg> Viewer<Model, Msg> for String {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text(self.clone())
    }
}

impl<'a, Model, Msg> Viewer<Model, Msg> for &'a str {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text(self.to_string())
    }
}

// impl<Model, Msg> Viewer<Model, Msg> for VirtualNode<Msg> {
//     fn view(&self, _: &Model) -> VirtualNode<Msg> {
//         *self.clone()
//     }
// }

// impl<Model, Msg> Viewer<Model, Msg> for VirtualElement<Msg> {
//     fn view(&self, _: &Model) -> VirtualNode<Msg> {
//         (*self.clone()).into()
//     }
// }

impl<Model, Msg, T, V> Viewer<Model, Msg> for T
where
    T: Fn(&Model) -> V,
    V: Into<VirtualNode<Msg>>,
{
    fn view(&self, model: &Model) -> VirtualNode<Msg> {
        (self)(model).into()
    }
}

// impl<'a, Model, Msg, T> Viewer<Model, Msg> for T
// where
//     T: Fn(&'a Model) -> VirtualNode<Msg>,
//     Model: ?Copy + 'a,
// {
// }
