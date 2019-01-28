use oak::html::{div, input, Html};

#[test]
fn basic_div() {
    let html: Html<()> = div(vec![], vec![]);
    let output = html.to_string();
    assert_eq!(&output, "<div></div>");
}

#[test]
fn basic_input() {
    let html: Html<()> = input(vec![]);
    let output = html.to_string();
    assert_eq!(&output, "<input />");
}
