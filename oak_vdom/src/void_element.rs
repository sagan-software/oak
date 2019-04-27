use super::{Attributes, Text};

#[derive(Debug, PartialEq, Clone)]
pub struct BareVoidElement {
    pub tag: &'static str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VoidElement {
    pub tag: Text,
    pub attrs: Attributes,
}

#[macro_export]
macro_rules! declare_void_elements {
    ($($x:ident)*) => ($(
        #[allow(non_upper_case_globals)]
        pub const $x: $crate::BareVoidElement = $crate::BareVoidElement {
            tag: stringify!($x)
        };
    )*)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn void_elements() {
        declare_void_elements! {
            input
            br
        }
        assert_eq!(input, BareVoidElement { tag: "input" });
        assert_eq!(br, BareVoidElement { tag: "br" });
    }
}
