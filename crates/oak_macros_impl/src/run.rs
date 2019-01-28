use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let expanded = quote! {
        #[oak::browser::wasm_bindgen(start)]
        #input
    };
    TokenStream::from(expanded)
}
