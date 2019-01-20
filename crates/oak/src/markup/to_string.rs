use crate::markup::{Attribute, Children, Markup, Tag};

impl<Msg> ToString for Markup<Msg> {
    fn to_string(&self) -> String {
        match self {
            Markup::Tag(tag) => tag.to_string(),
            Markup::Text(text) => text.clone(),
            Markup::Fragment(nodes) => nodes.iter().fold(String::new(), |mut acc, node| {
                let s = node.to_string();
                acc.push_str(s.as_str());
                acc
            }),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::markup::{Attribute, Children, Markup, Tag};

    #[test]
    fn text() {
        let node: Markup<()> = Markup::Text("Testing!".to_owned());
        let output = node.to_string();
        assert_eq!(&output, "Testing!");
    }

    #[test]
    fn tag() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: Vec::new(),
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div></div>");
    }

    #[test]
    fn tag_with_namespace() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: Some("x".to_owned()),
            name: "for-each".to_owned(),
            attributes: Vec::new(),
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<x:for-each></x:for-each>");
    }

    #[test]
    fn tag_with_text_attr() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: vec![Attribute::Text("class".to_owned(), "test".to_owned())],
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div class=\"test\"></div>");
    }

    #[test]
    fn tag_with_text_attrs() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "a".to_owned(),
            attributes: vec![
                Attribute::Text("class".to_owned(), "test".to_owned()),
                Attribute::Text("href".to_owned(), "#test".to_owned()),
            ],
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<a class=\"test\" href=\"#test\"></a>");
    }

    #[test]
    fn tag_with_bool_attr() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: vec![Attribute::Bool("test".to_owned())],
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div test></div>");
    }

    #[test]
    fn tag_with_bool_attrs() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: vec![
                Attribute::Bool("test1".to_owned()),
                Attribute::Bool("test2".to_owned()),
            ],
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div test1 test2></div>");
    }

    #[test]
    fn tag_with_mixed_attrs() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: vec![
                Attribute::Text("class".to_owned(), "test1".to_owned()),
                Attribute::Bool("test2".to_owned()),
            ],
            children: Children::Nodes(Vec::new()),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div class=\"test1\" test2></div>");
    }

    #[test]
    fn tag_with_children() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "div".to_owned(),
            attributes: Vec::new(),
            children: Children::Nodes(vec![
                Markup::Text("Test".to_owned()),
                Markup::Tag(Tag {
                    key: None,
                    namespace: None,
                    name: "input".to_owned(),
                    attributes: Vec::new(),
                    children: Children::SelfClosing,
                }),
            ]),
        };
        let output = tag.to_string();
        assert_eq!(&output, "<div>Test<input /></div>");
    }

    #[test]
    fn self_closing_tag() {
        let tag: Tag<()> = Tag {
            key: None,
            namespace: None,
            name: "input".to_owned(),
            attributes: Vec::new(),
            children: Children::SelfClosing,
        };
        let output = tag.to_string();
        assert_eq!(&output, "<input />");
    }

    #[test]
    fn fragment() {
        let fragment: Markup<()> = Markup::Fragment(vec![
            Markup::Tag(Tag {
                key: None,
                namespace: None,
                name: "div".to_owned(),
                attributes: Vec::new(),
                children: Children::Nodes(vec![
                    Markup::Text("Test".to_owned()),
                    Markup::Tag(Tag {
                        key: None,
                        namespace: None,
                        name: "input".to_owned(),
                        attributes: Vec::new(),
                        children: Children::SelfClosing,
                    }),
                ]),
            }),
            Markup::Tag(Tag {
                key: None,
                namespace: None,
                name: "input".to_owned(),
                attributes: Vec::new(),
                children: Children::SelfClosing,
            }),
            Markup::Text("Hello World".to_owned()),
        ]);
        let output = fragment.to_string();
        assert_eq!(&output, "<div>Test<input /></div><input />Hello World");
    }

}
