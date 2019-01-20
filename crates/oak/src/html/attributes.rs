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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::{a, input, text, Html};

    #[test]
    fn string_properties() {
        let html: Html<()> = a(&[class("test"), href("#test")], &[text("Hello World")]);
        let output = html.to_string();
        assert_eq!(&output, "<a class=\"test\" href=\"#test\">Hello World</a>");
    }

    #[test]
    fn bool_properties() {
        let html: Html<()> = input(&[checked(), autofocus()]);
        let output = html.to_string();
        assert_eq!(&output, "<input checked autofocus />");
    }

}
