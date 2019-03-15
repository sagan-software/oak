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

pub fn data<K: ToString, V: ToString>(key: K, val: V) -> oak_vdom::Attribute {
    let name = "data-".to_string() + key.to_string().as_str();
    oak_vdom::Attribute(name, val.to_string())
}


pub fn style<V: ToString>(val: V) -> oak_vdom::Attribute {
    oak_vdom::Attribute("style".to_owned(), val.to_string())
}