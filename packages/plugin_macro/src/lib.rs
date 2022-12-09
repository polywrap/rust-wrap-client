use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parser};
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn plugin_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub env: polywrap_core::env::Env })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}
