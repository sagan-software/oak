use crate::html::{Attribute, Children, Element, Html};

macro_rules! declare_elements {
    ($($x:ident)*) => ($(
        pub fn $x<Msg: Clone, A: AsRef<[Attribute<Msg>]>, H: Into<Html<Msg>> + Clone, C: AsRef<[H]>>(
            attrs: A,
            children: C,
        ) -> Html<Msg> {
            Html::Element(Element {
                name: stringify!($x).to_owned(),
                children: Children::Nodes(children.as_ref().to_vec().into_iter().map(|c| c.into()).collect()),
                attrs: attrs.as_ref().to_vec(),
            })
        }
    )*)
}

macro_rules! declare_void_elements {
    ($($x:ident)*) => ($(
        pub fn $x<Msg: Clone, A: AsRef<[Attribute<Msg>]>>(attrs: A) -> Html<Msg> {
            Html::Element(Element {
                name: stringify!($x).to_owned(),
                children: Children::SelfClosing,
                attrs: attrs.as_ref().to_vec(),
            })
        }
    )*)
}

declare_elements!(
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
);

declare_void_elements! {
    link
    meta
    hr
    br
    input
    iframe
    canvas
    img
}
