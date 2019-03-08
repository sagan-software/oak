pub type Html = crate::Node;

crate::declare_elements! {
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

crate::declare_text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

crate::declare_text_attributes! {
    type_, "type"
    for_, "for"
}

crate::declare_bool_attributes! {
    autofocus
    checked
    hidden
}

pub fn click<Msg: Clone + 'static>(msg: Msg) -> crate::Event {
    let func = move |_: ()| -> Msg { msg.clone() };
    let handler = crate::EventHandler::new(func);
    crate::Event("click".to_owned(), handler)
}
