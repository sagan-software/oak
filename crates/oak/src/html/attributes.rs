use crate::html::Attribute;

macro_rules! declare_text_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x<Msg>(value: &str) -> Attribute<Msg> {
            Attribute::Text($tag.to_owned(), value.to_owned())
        }
    )*);

    ($($x:ident)*) => ($(
        declare_text_attributes!($x, stringify!($x));
    )*);
}

macro_rules! declare_bool_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x<Msg>() -> Attribute<Msg> {
            Attribute::Bool($tag.to_owned())
        }
    )*);

    ($($x:ident)*) => ($(
        declare_bool_attributes!($x, stringify!($x));
    )*);
}

pub fn class_list<Msg>(classes: &[(&str, bool)]) -> Attribute<Msg> {
    let active = classes
        .iter()
        .filter(|(_, active)| *active)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    // TODO: Change `class` to use Into<Cow> and use it here
    Attribute::Text("className".to_owned(), active.join(" "))
}

pub fn key<Msg>(key: String) -> Attribute<Msg> {
    Attribute::Key(key)
}

declare_text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

declare_text_attributes! {
    type_, "type"
    for_, "for"
}

declare_bool_attributes! {
    autofocus
    checked
    hidden
}
