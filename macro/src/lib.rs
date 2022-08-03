#![allow(unused_variables)]
use proc_macro::{TokenStream};
//use syn::parse;
use syn::{parse_macro_input};
use quote::quote;
mod element;
use element::Element;


#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let element =  parse_macro_input!(input as Element);
    println!("element: {}", quote!{#element}.to_string());
    quote!{#element}.into()
}
