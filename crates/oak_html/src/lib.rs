pub mod attributes;
pub mod events;

pub type Html<Msg> = oak_vdom::Node<Msg>;
pub type HtmlElement<Msg> = oak_vdom::Element<Msg>;
pub type Attribute = oak_vdom::Attribute;
pub type Event<Msg> = oak_vdom::Event<Msg>;

oak_vdom::declare_elements! {
    html
    head
    body
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
    link
    meta
    hr
    br
    input
    iframe
    canvas
    img
}