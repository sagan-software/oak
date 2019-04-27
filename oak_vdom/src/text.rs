use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub struct Text(Cow<'static, str>);

impl From<&'static str> for Text {
    fn from(v: &'static str) -> Self {
        Self(v.into())
    }
}

impl From<String> for Text {
    fn from(v: String) -> Self {
        Self(v.into())
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<T> From<T> for Text
// where
//     T: ToString,
// {
//     fn from(val: T) -> Self {
//         Text(val.to_string().into())
//     }
// }

macro_rules! from_to_string_types {
    ($($t:ty)*) => ($(
        impl From<$t> for Text {
            fn from(val: $t) -> Self {
                Text(val.to_string().into())
            }
        }
    )*)
}

from_to_string_types! {
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
