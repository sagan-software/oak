pub mod attributes;
pub mod events;

pub use crate::markup::{fragment, tag, text, Attribute, Children, Markup, Tag};

pub type Html<Msg> = Markup<Msg>;

macro_rules! declare_tags {
    ($($x:ident)*) => ($(
        pub fn $x<Msg: Clone>(attributes: &[Attribute<Msg>], children: &[Html<Msg>]) -> Html<Msg> {
            Markup::Tag(Tag {
                key: None,
                namespace: None,
                name: stringify!($x).to_owned(),
                attributes: attributes.to_vec(),
                children: Children::Nodes(children.to_vec()),
            })
        }
    )*)
}

declare_tags! {
    h1
    h2
    h3
    h4
    h5
    h6
    div
    span
    p
    pre
    blockquote
    a
    code
    em
    strong
    i
    b
    u
    sub
    sup
    ol
    ul
    li
    dl
    dt
    dd
    form
    textarea
    button
    select
    option
    section
    nav
    article
    aside
    header
    footer
    address
    main
    figure
    figcaption
    table
    caption
    colgroup
    col
    tbody
    thead
    tfoot
    tr
    td
    th
    fieldset
    legend
    label
    datalist
    optgroup
    output
    progress
    meter
    audio
    video
    source
    track
    embed
    object
    param
    ins
    del
    small
    cite
    dfn
    abbr
    time
    var
    samp
    kbd
    s
    q
    mark
    ruby
    rt
    rp
    bdi
    bdo
    wbr
    details
    summary
    menuitem
    menu
}

macro_rules! declare_void_tags {
    ($($x:ident)*) => ($(
        pub fn $x<Msg: Clone>(attributes: &[Attribute<Msg>]) -> Html<Msg> {
            Markup::Tag(Tag {
                key: None,
                namespace: None,
                name: stringify!($x).to_owned(),
                attributes: attributes.to_vec(),
                children: Children::SelfClosing,
            })
        }
    )*)
}

declare_void_tags! {
    link
    meta
    hr
    br
    input
    iframe
    canvas
    img
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn basic_div() {
        let html: Html<()> = div(&[], &[]);
        let output = html.to_string();
        assert_eq!(&output, "<div></div>");
    }

    #[test]
    fn basic_input() {
        let html: Html<()> = input(&[]);
        let output = html.to_string();
        assert_eq!(&output, "<input />");
    }

}
