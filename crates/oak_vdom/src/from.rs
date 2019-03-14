use crate::{Element, Node};

impl<Msg> From<Element<Msg>> for Node<Msg> {
    fn from(el: Element<Msg>) -> Self {
        Node::Element(el)
    }
}

impl<Msg> From<String> for Node<Msg> {
    fn from(val: String) -> Self {
        Node::Text(val)
    }
}

impl<'a, Msg> From<&'a str> for Node<Msg> {
    fn from(val: &'a str) -> Self {
        val.to_owned().into()
    }
}

macro_rules! from_num_types {
    ($($t:ty)*) => ($(
        impl<Msg> From<$t> for Node<Msg> {
            fn from(val: $t) -> Self {
                val.to_string().into()
            }
        }

        impl<'a, Msg> From<&'a $t> for Node<Msg> {
            fn from(val: &'a $t) -> Self {
                val.to_string().into()
            }
        }
    )*)
}

from_num_types! {
    u8
    u16
    u32
    u64
    usize
    i8
    i16
    i32
    i64
    isize
    f32
    f64
}
