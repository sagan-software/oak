use crate::{Children, Element, Node};

impl<Msg> ToString for Node<Msg> {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl<Msg> ToString for Element<Msg> {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl<Msg> ToString for Children<Msg> {
    fn to_string(&self) -> String {
        String::new()
    }
}
