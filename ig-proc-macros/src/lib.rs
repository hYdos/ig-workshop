extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MetaEnum)]
pub fn derive_meta_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let key_lit = name.to_string();

    let expanded = quote! {
        impl MetaEnumImpl for #name {
            const META_KEY: &'static str = #key_lit;
        }
    };
    TokenStream::from(expanded)
}