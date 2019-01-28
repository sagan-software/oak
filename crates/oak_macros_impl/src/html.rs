use crate::proc_macro::TokenStream;
use quote::quote;

pub fn expand(input: TokenStream) -> TokenStream {
    let expanded = quote! {
        oak::vdom::VNode::Text("Balls".to_owned())
    };
    expanded.into()
}
