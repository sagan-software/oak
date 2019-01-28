#[derive(Clone)]
pub enum Markup<Msg> {
    Tag(Tag<Msg>),
    Text(String),
}

pub fn tag<Msg>(
    name: &str,
    attributes: Vec<Attribute<Msg>>,
    children: Vec<Markup<Msg>>,
) -> Markup<Msg> {
    Markup::Tag(Tag {
        namespace: None,
        name: name.to_owned(),
        attributes,
        children: Children::Nodes(children),
    })
}

pub fn text<Msg>(value: &str) -> Markup<Msg> {
    Markup::Text(value.to_owned())
}

impl<Msg> From<Tag<Msg>> for Markup<Msg> {
    fn from(tag: Tag<Msg>) -> Self {
        Markup::Tag(tag)
    }
}

impl<Msg> From<String> for Markup<Msg> {
    fn from(val: String) -> Self {
        Markup::Text(val)
    }
}

impl<Msg> From<&str> for Markup<Msg> {
    fn from(val: &str) -> Self {
        Markup::Text(val.to_owned())
    }
}

impl<Msg> ToString for Markup<Msg> {
    fn to_string(&self) -> String {
        match self {
            Markup::Tag(tag) => tag.to_string(),
            Markup::Text(text) => text.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Tag<Msg> {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: Vec<Attribute<Msg>>,
    pub children: Children<Msg>,
}

#[derive(Clone)]
pub enum Children<Msg> {
    SelfClosing,
    Nodes(Vec<Markup<Msg>>),
}

#[derive(Clone)]
pub enum Attribute<Msg> {
    Text(String, String),
    Bool(String),
    Key(String),
    SimpleEvent(String, Msg),
}

impl<Msg> ToString for Tag<Msg> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push('<');
        if let Some(ns) = &self.namespace {
            s.push_str(ns.as_str());
            s.push(':');
        }
        s.push_str(self.name.as_str());
        for attribute in self.attributes.iter() {
            match attribute {
                Attribute::Text(key, val) => {
                    s.push(' ');
                    s.push_str(key.as_str());
                    s.push('=');
                    s.push('"');
                    s.push_str(val.as_str());
                    s.push('"');
                }
                Attribute::Bool(key) => {
                    s.push(' ');
                    // TODO escape double quotes
                    s.push_str(key.as_str());
                }
                _ => (),
            }
        }

        match self.children {
            Children::SelfClosing => {
                s.push(' ');
                s.push('/');
                s.push('>');
                return s;
            }
            Children::Nodes(ref nodes) => {
                s.push('>');
                for markup in nodes.iter() {
                    s.push_str(markup.to_string().as_str());
                }
            }
        }

        s.push_str("</");
        if let Some(ns) = &self.namespace {
            s.push_str(ns.as_str());
            s.push(':');
        }
        s.push_str(self.name.as_str());
        s.push('>');
        s
    }
}
