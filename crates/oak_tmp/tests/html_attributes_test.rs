use oak::html::{
    a,
    attributes::{autofocus, checked, class, href},
    input, text, Html,
};

#[test]
fn string_properties() {
    let html: Html<()> = a(
        vec![class("test"), href("#test")],
        vec![text("Hello World")],
    );
    let output = html.to_string();
    assert_eq!(&output, "<a class=\"test\" href=\"#test\">Hello World</a>");
}

#[test]
fn bool_properties() {
    let html: Html<()> = input(vec![checked(), autofocus()]);
    let output = html.to_string();
    assert_eq!(&output, "<input checked autofocus />");
}
