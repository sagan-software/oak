extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod html;

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    html::expand(input)
}
