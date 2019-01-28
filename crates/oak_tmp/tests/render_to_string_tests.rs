use oak::markup::{Attribute, Children, Markup, Tag};

#[test]
fn text() {
    let node: Markup<()> = Markup::Text("Testing!".to_owned());
    let output = node.to_string();
    assert_eq!(&output, "Testing!");
}

#[test]
fn tag() {
    let tag: Tag<()> = Tag {
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
        namespace: None,
        name: "div".to_owned(),
        attributes: Vec::new(),
        children: Children::Nodes(vec![
            Markup::Text("Test".to_owned()),
            Markup::Tag(Tag {
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
        namespace: None,
        name: "input".to_owned(),
        attributes: Vec::new(),
        children: Children::SelfClosing,
    };
    let output = tag.to_string();
    assert_eq!(&output, "<input />");
}
