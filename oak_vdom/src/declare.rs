#[macro_export]
macro_rules! declare_open_elements {
    ($($x:ident)*) => ($(
        #[allow(non_upper_case_globals)]
        pub const $x: $crate::BareOpenElement = $crate::BareOpenElement {
            tag: stringify!($x)
        };
    )*)
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
    use crate::{BareOpenElement, BareVoidElement};

    #[test]
    fn open_elements() {
        declare_open_elements! {
            div
            span
        }
        assert_eq!(div, BareOpenElement { tag: "div" });
        assert_eq!(span, BareOpenElement { tag: "span" });
    }

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
