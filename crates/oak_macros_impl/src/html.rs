use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Expr};
use syn_markup::*;

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let markup = parse_macro_input!(input as syn_markup::Markup);
    markup_to_tokens(&markup).into()
}

fn markup_to_tokens(markup: &Markup) -> TokenStream {
    let nodes = markup.nodes.iter().map(node_to_tokens);
    quote!(#(#nodes)*)
}

fn node_to_tokens(node: &Node) -> TokenStream {
    match node {
        Node::OpenElement(el) => open_el_to_tokens(el),
        Node::VoidElement(el) => void_el_to_tokens(el),
        Node::Text(text) => text_to_tokens(text),
        Node::Braced(braced) => braced_to_tokens(braced),
    }
}

fn open_el_to_tokens(el: &OpenElement) -> TokenStream {
    let open_tag = &el.open_tag;
    println!(
        "**************s************* {} {}",
        open_tag.name,
        open_tag.name.span().end().column
    );
    let name = format!("{}", open_tag.name);
    let attributes = attributes_to_tokens(&open_tag.attributes);

    let children = if el.children.is_empty() {
        quote!(Vec::new())
    } else {
        let nodes = el.children.iter().map(|child_node| {
            let child_tokens = node_to_tokens(child_node);
            quote!(#child_tokens,)
        });
        quote!(vec![#(#nodes)*])
    };

    quote! {
        VirtualNode::Element(VirtualElement {
            namespace: None,
            name: #name.to_owned(),
            attributes: #attributes,
            children: VirtualChildren::Nodes(#children),
        })
    }
}

fn void_el_to_tokens(el: &VoidElement) -> TokenStream {
    let name = format!("{}", el.name);
    let attributes = attributes_to_tokens(&el.attributes);

    quote! {
        VirtualNode::Element(VirtualElement {
            namespace: None,
            name: #name.to_owned(),
            attributes: #attributes,
            children: VirtualChildren::Void,
        })
    }
}

fn attributes_to_tokens(attrs: &[Attribute]) -> TokenStream {
    if attrs.is_empty() {
        return quote!(std::collections::BTreeMap::new());
    }

    let pairs = attrs.iter().map(|attribute| {
        let key = format!("{}", attribute.key);
        let value = &attribute.value;
        match value {
            Expr::Closure(closure) => {
                // TODO! This struct will probably need to change when this is added.
                quote!()
            }
            _ => quote!((#key.to_owned(), #value.to_owned()),),
        }
    });

    quote! {
        <std::collections::BTreeMap<String, String> as std::iter::FromIterator<(String, String)>>::from_iter(vec![#(#pairs)*].into_iter())
    }
}

fn text_to_tokens(text: &Text) -> TokenStream {
    let text = &text.text;
    println!("!!!! {}", text);
    quote!(VirtualNode::Text(#text.to_owned()))
}

fn braced_to_tokens(braced: &Braced) -> TokenStream {
    let block = &braced.block;
    quote!(#block.into())
}
