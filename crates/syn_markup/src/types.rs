use syn::{Block, Expr, Ident, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Markup {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    OpenElement(OpenElement),
    VoidElement(VoidElement),
    Text(Text),
    Braced(Braced),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenElement {
    pub open_tag: OpenTag,
    pub children: Vec<Node>,
    pub close_tag: CloseTag,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenTag {
    pub lt_token: Token![<],
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub gt_token: Token![>],
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloseTag {
    pub lt_token: Token![<],
    pub slash_token: Token![/],
    pub name: Ident,
    pub gt_token: Token![>],
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoidElement {
    pub lt_token: Token![<],
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub slash_token: Token![/],
    pub gt_token: Token![>],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub key: Ident,
    pub eq_token: Token![=],
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Braced {
    pub block: Box<Block>,
}

impl Node {
    pub fn open_element(&self) -> Option<&OpenElement> {
        if let Node::OpenElement(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn void_element(&self) -> Option<&VoidElement> {
        if let Node::VoidElement(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn text(&self) -> Option<&Text> {
        if let Node::Text(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn braced(&self) -> Option<&Braced> {
        if let Node::Braced(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
