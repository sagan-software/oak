use crate::types::*;

impl From<Element> for Node {
    fn from(el: Element) -> Self {
        Node::Element(el)
    }
}

impl From<String> for Node {
    fn from(val: String) -> Self {
        Node::Text(val)
    }
}

impl<'a> From<&'a str> for Node {
    fn from(val: &'a str) -> Self {
        val.to_owned().into()
    }
}

macro_rules! from_num_types {
    ($($t:ty)*) => ($(
        impl From<$t> for Node {
            fn from(val: $t) -> Self {
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
}

impl IntoIterator for Node {
    type Item = Node;
    type IntoIter = ::std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl Into<::std::vec::IntoIter<Node>> for Node {
    fn into(self) -> ::std::vec::IntoIter<Node> {
        self.into_iter()
    }
}
