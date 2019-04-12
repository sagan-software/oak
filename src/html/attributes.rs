declare_text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

declare_text_attributes! {
    type_, "type"
    for_, "for"
}

declare_bool_attributes! {
    autofocus
    checked
    hidden
}

pub fn data<Msg, K: ToString, V: ToString>(key: K, val: V) -> crate::vdom::Attribute<Msg> {
    let name = "data-".to_string() + key.to_string().as_str();
    crate::vdom::Attribute::Text(name, val.to_string())
}

pub fn style<Msg, V: ToString>(val: V) -> crate::vdom::Attribute<Msg> {
    crate::vdom::Attribute::Text("style".to_owned(), val.to_string())
}

pub fn classes<Msg, T: AsRef<[V]>, V: AsRef<str>>(val: T) -> crate::vdom::Attribute<Msg> {
    let mut s = String::new();
    for v in val.as_ref() {
        s.push_str(v.as_ref());
        s.push(' ');
    }
    class(s)
}
