oak_vdom::declare_text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

oak_vdom::declare_text_attributes! {
    type_, "type"
    for_, "for"
}

oak_vdom::declare_bool_attributes! {
    autofocus
    checked
    hidden
}

pub fn data<Msg, K: ToString, V: ToString>(key: K, val: V) -> oak_vdom::Attribute<Msg> {
    let name = "data-".to_string() + key.to_string().as_str();
    oak_vdom::Attribute::Text(name, val.to_string())
}

pub fn style<Msg, V: ToString>(val: V) -> oak_vdom::Attribute<Msg> {
    oak_vdom::Attribute::Text("style".to_owned(), val.to_string())
}

pub fn classes<Msg, T: AsRef<[V]>, V: AsRef<str>>(val: T) -> oak_vdom::Attribute<Msg> {
    let mut s = String::new();
    for v in val.as_ref() {
        s.push_str(v.as_ref());
        s.push(' ');
    }
    class(s)
}
