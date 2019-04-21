use crate::{write_attrs, AttrValue, CowStr, ElementBuilder};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BareOpenElement {
    pub tag: &'static str,
}

impl<Msg> ElementBuilder<Msg> for BareOpenElement {
    type Output = SimpleOpenElement<Msg>;
    fn attr(self, key: CowStr, val: AttrValue<Msg>) -> Self::Output {
        let mut attrs = BTreeMap::new();
        attrs.insert(key, val);
        Self::Output {
            tag: self.tag.into(),
            attrs,
        }
    }
}

impl std::fmt::Display for BareOpenElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}></{}>", self.tag, self.tag)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpenElement<Msg> {
    pub tag: CowStr,
    pub attrs: BTreeMap<CowStr, AttrValue<Msg>>,
}

impl<Msg> ElementBuilder<Msg> for OpenElement<Msg> {
    type Output = Self;
    fn attr(self, key: CowStr, val: AttrValue<Msg>) -> Self::Output {
        let Self { tag, mut attrs } = self;
        attrs.insert(key, val);
        Self { tag, attrs }
    }
}

impl<A: Render + 'static> FnOnce<(A,)> for Tag {
    type Output = FinalTag<A>;
    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        FinalTag {
            tag: self.tag,
            attrs: self.attrs,
            inn: args.0,
        }
    }
}
