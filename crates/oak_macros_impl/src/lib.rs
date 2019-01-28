extern crate proc_macro;

mod html;
mod run;

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_attribute]
pub fn start(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::run::expand(args, input)
}

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    crate::html::expand(input)
}
