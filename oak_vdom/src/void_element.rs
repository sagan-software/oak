use crate::{AttrValue, CowStr, ElementBuilder};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BareVoidElement {
    pub tag: &'static str,
}

impl<Msg> ElementBuilder<Msg> for BareVoidElement {
    type Output = VoidElement<Msg>;
    fn attr(self, key: CowStr, val: AttrValue<Msg>) -> Self::Output {
        let mut attrs = BTreeMap::new();
        attrs.insert(key, val);
        Self::Output {
            tag: self.tag.into(),
            attrs,
        }
    }
}

impl std::fmt::Display for BareVoidElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}/>", self.tag)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VoidElement<Msg> {
    pub tag: CowStr,
    pub attrs: BTreeMap<CowStr, AttrValue<Msg>>,
}

impl<Msg> ElementBuilder<Msg> for VoidElement<Msg> {
    type Output = Self;
    fn attr(self, key: CowStr, val: AttrValue<Msg>) -> Self::Output {
        let Self { tag, mut attrs } = self;
        attrs.insert(key, val);
        Self { tag, attrs }
    }
}

impl<Msg> std::fmt::Display for VoidElement<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}", self.tag)?;
        write_attrs(f, self.attrs.iter())?;
        write!(f, "/>")
    }
}
