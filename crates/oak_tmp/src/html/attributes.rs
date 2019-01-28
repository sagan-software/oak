use crate::markup::Attribute;

macro_rules! text_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x<Msg>(value: &str) -> Attribute<Msg> {
            Attribute::Text($tag.to_owned(), value.to_owned())
        }
    )*);

    ($($x:ident)*) => ($(
        text_attributes!($x, stringify!($x));
    )*);
}

macro_rules! bool_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x<Msg>() -> Attribute<Msg> {
            Attribute::Bool($tag.to_owned())
        }
    )*);

    ($($x:ident)*) => ($(
        bool_attributes!($x, stringify!($x));
    )*);
}

text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

text_attributes! {
    type_, "type"
    for_, "for"
}

bool_attributes! {
    autofocus
    checked
    hidden
}
