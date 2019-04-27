use super::{Element, Text};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Text(Text),
    Element(Element),
    Fragment(Vec<Node>),
}

impl From<Element> for Node {
    fn from(el: Element) -> Node {
        Node::Element(el)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Node::Text(text)
    }
}

impl<T> From<Vec<T>> for Node
where
    T: Into<Node>,
{
    fn from(val: Vec<T>) -> Node {
        Node::Fragment(val.into_iter().map(std::convert::Into::into).collect())
    }
}

macro_rules! from_into_text_types {
    ($($t:ty)*) => ($(
        impl From<$t> for Node {
            fn from(val: $t) -> Self {
                Node::Text(val.into())
            }
        }
    )*)
}

from_into_text_types! {
    String
    usize
    u8
    u16
    u32
    u64
    u128
    isize
    i8
    i16
    i32
    i64
    i128
    f32
    f64
}

// macro_rules! for_each_tuple_ {
//     ( $m:ident !! ) => (
//         $m! { }
//     );
//     ( $m:ident !! $h:ident, $($t:ident,)* ) => (
//         $m! { $h $($t)* }
//         for_each_tuple_! { $m !! $($t,)* }
//     );
// }

// macro_rules! for_each_tuple {
//     ( $m:ident ) => (
//         for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, }
//     );
// }

macro_rules! from_tuple_types {
    ($($x:ident)*) => (
        impl<$($x: Into<Node>),*> From<($($x,)*)> for Node {
        #[allow(non_snake_case)]
            fn from(($($x,)*): ($($x,)*)) -> Self {
                Node::Fragment(vec![
                    $($x.into(),)*
                ])
            }
        }
    )
}

from_tuple_types!(A);
from_tuple_types!(A B);
from_tuple_types!(A B C);
from_tuple_types!(A B C D);
from_tuple_types!(A B C D E);
from_tuple_types!(A B C D E F);
from_tuple_types!(A B C D E F G);
from_tuple_types!(A B C D E F G H);
from_tuple_types!(A B C D E F G H I);
from_tuple_types!(A B C D E F G H I J);
from_tuple_types!(A B C D E F G H I J K);
from_tuple_types!(A B C D E F G H I J K L);
from_tuple_types!(A B C D E F G H I J K L M);
from_tuple_types!(A B C D E F G H I J K L M N);
from_tuple_types!(A B C D E F G H I J K L M N O);
from_tuple_types!(A B C D E F G H I J K L M N O P);
from_tuple_types!(A B C D E F G H I J K L M N O P Q);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U V);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U V W);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U V W X);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U V W X Y);
from_tuple_types!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
