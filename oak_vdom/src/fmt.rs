use crate::{AttrValue, BareOpenElement, BareVoidElement, CowStr, OpenElement, VoidElement};
use std::fmt::{Display, Formatter, Result};

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::BTreeMap;

    #[test]
    fn bare_open_element() {
        let el = BareOpenElement { tag: "div" };
        assert_eq!(el.to_string(), "<div></div>");
    }

    #[test]
    fn bare_void_element() {
        let el = BareVoidElement { tag: "input" };
        assert_eq!(el.to_string(), "<input/>");
    }

    #[test]
    fn open_element() {
        let el: OpenElement<()> = OpenElement {
            tag: "div".into(),
            attrs: BTreeMap::new(),
        };
        assert_eq!(el.to_string(), "<div></div>");

        {
            let el = el.clone().class("testing");
            assert_eq!(el.to_string(), r#"<div class="testing"></div>"#);
        }

        {
            let el = el.clone().hidden(true);
            assert_eq!(el.to_string(), r#"<div hidden></div>"#);
        }

        {
            let el = el.clone().hidden(false);
            assert_eq!(el.to_string(), r#"<div></div>"#);
        }
    }

    #[test]
    fn void_element() {
        let el: VoidElement<()> = VoidElement {
            tag: "input".into(),
            attrs: BTreeMap::new(),
        };
        assert_eq!(el.to_string(), "<input/>");

        {
            let el = el.clone().class("testing");
            assert_eq!(el.to_string(), r#"<input class="testing"/>"#);
        }

        {
            let el = el.clone().checked(true);
            assert_eq!(el.to_string(), r#"<input checked/>"#);
        }

        {
            let el = el.clone().checked(false);
            assert_eq!(el.to_string(), r#"<input/>"#);
        }
    }

}
