use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parser};
use syn::{parse, parse_macro_input, ItemStruct, DeriveInput};

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
        #[derive(polywrap_plugin_macro::Plugin)]
        #item_struct
    }
    .into();
}


#[proc_macro_derive(Plugin)]
pub fn derive_plugin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl polywrap_plugin::module::PluginWithEnv for #name {
            fn set_env(&mut self, env: polywrap_core::env::Env) {
                self.env = env;
            }
            
            fn get_env(&self, key: String) -> Option<&polywrap_core::env::Env> {
                if let Some(env) = self.env.get(&key) {
                  Some(env)
                } else {
                  None
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}