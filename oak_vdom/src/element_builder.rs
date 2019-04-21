use crate::{AttrValue, CowStr};

macro_rules! declare_text_attributes {
    ($($t1:ident, $t2:expr)*) => ($(
        fn $t1<V: Into<CowStr>>(self, val: V) -> Self::Output {
            self.attr($t2.into(), AttrValue::Text(val.into()))
        }
    )*);

    ($($x:ident)*) => ($(
        declare_text_attributes!($x, stringify!($x));
    )*);
}

macro_rules! declare_bool_attributes {
    ($($t1:ident, $t2:expr)*) => ($(
        fn $t1(self, val: bool) -> Self::Output {
            self.attr($t2.into(), AttrValue::Bool(val))
        }
    )*);

    ($($x:ident)*) => ($(
        declare_bool_attributes!($x, stringify!($x));
    )*);
}

pub trait ElementBuilder<Msg>: Sized {
    type Output;

    fn attr(self, key: CowStr, val: AttrValue<Msg>) -> Self::Output;

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
}

pub(crate) fn write_attrs<'a, 'b, Msg: 'b, I: Iterator<Item = (&'a CowStr, &'b AttrValue<Msg>)>>(
    f: &mut std::fmt::Formatter,
    attrs: I,
) -> std::fmt::Result {
    for (key, val) in attrs {
        match val {
            AttrValue::Text(text) => {
                // TODO escape text value
                write!(f, " {}=\"{}\"", key, text)?;
            }
            AttrValue::Bool(show) => {
                if *show {
                    write!(f, " {}", key)?;
                }
            }
            AttrValue::EventHandler(_) => (),
        }
    }
    Ok(())
}
