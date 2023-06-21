use polywrap_core::uri::Uri;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn uri(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitStr);

    // Get the string value
    let uri_str = input.value();

    let uri_result = Uri::try_from(uri_str);

    match uri_result {
        Ok(uri) => {
            let authority = uri.authority();
            let path = uri.path();
            let uri = uri.uri();
            // Return the Uri struct
            let expanded = quote! {
                unsafe {
                    Uri::from_parts(#authority.to_owned(), #path.to_owned(), #uri.to_owned())
                }
            };
            TokenStream::from(expanded)
        }
        Err(err) => {
            // Error handling, this will be a compile-time error
            panic!("Failed to parse uri: {:?}", err);
        }
    }
}
