use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput, FnArg, ItemImpl, ItemStruct, ImplItem};

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

#[proc_macro_attribute]
pub fn plugin_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(input as ItemImpl);
    let _ = parse_macro_input!(args as parse::Nothing);

    let struct_ident = item_impl.clone().self_ty;

    let mut method_idents: Vec<(Ident, String, Ident)> = vec![];

    for item in item_impl.clone().items {
        match item {
            syn::ImplItem::Method(method) => {
                match method.sig.clone().inputs.len() {
                    3 => {
                        let function_input = match &method.sig.inputs[1] {
                          syn::FnArg::Typed(pat_type) => {
                              if let syn::Type::Reference(type_reference) = &*pat_type.ty {
                                  if let syn::Type::Path(type_path) = &*type_reference.elem {
                                      Some(type_path.path.segments[0].ident.clone())
                                  } else {
                                    None
                                  }
                              } else {
                                None
                              }
                          }
                          _ => panic!("Wrong argument type")
                      };
                        let function_ident = &method.sig.ident;
                        let function_ident_str = function_ident.to_string();

                        method_idents.push((
                            function_ident.clone(),
                            function_ident_str.clone(),
                            function_input.unwrap().clone(),
                        ))
                    }
                    _ => panic!("Wrong number of arguments")
                };
            }
            _ => panic!("Wrong function signature"),
        }
    }

    dbg!(method_idents.clone());

    let supported_methods =
        method_idents.clone()
            .into_iter()
            .enumerate()
            .map(|(_, (_, ident_str, _))| {
                quote! {
                  #ident_str
                }
            });

    let methods = method_idents
        .into_iter()
        .enumerate()
        .map(|(_, (ident, ident_str, args))| {
            quote! {
              #ident_str => {
                let result = self.#ident(
                  &serde_json::from_value::<#args>(params.clone())?,
                  invoker,
                ).await?;

                Ok(serde_json::to_value(result)?)
              }
            }
        });

    let module_impl = quote! {
        #[async_trait]
        impl PluginModule for #struct_ident {
          async fn _wrap_invoke(
            &mut self,
            method_name: &str,
            params: &serde_json::Value,
            invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
        ) -> Result<serde_json::Value, polywrap_plugin::error::PluginError> {
                let supported_methods = vec![#(#supported_methods),*];
                match method_name {
                    #(#methods)*
                    _ => panic!("Method '{}' not found. Supported methods: {:#?}", method_name, supported_methods),
                }
            }
        }
    };

    let s = quote! {
        #item_impl

        #module_impl
    };

    println!("{}", s);

    return s.into();
}

// #[proc_macro_attribute]
// pub fn my_macro(args: TokenStream, input: TokenStream) -> TokenStream {
//     let input_clone_a = input.clone();
//     let input_clone_b = input.clone();
//     let inputA = parse_macro_input!(input_clone_a as DeriveInput);
//     let inputB = parse_macro_input!(input_clone_b as ItemImpl);

//     let mut impl_block = match inputA.data {
//         syn::Data::Struct(data) => {
//             let struct_ident = data.struct_token;
//             let mut function_map = quote! {};
//             // Iterate through the trait implementation's functions
//             for item in inputB.items.iter() {
//                 if let ImplItem::Method(method) = item {
//                     let sig = match method.sig.clone().inputs.len() {
//                         2 => {
//                             let second_input = &method.sig.inputs[1];
//                             let function_ident = &method.sig.ident;
//                             function_map = quote! {
//                                 #function_map
//                                 #function_ident => {
//                                     let second_arg: #second_input = serde_json::from_value(json_value.clone()).unwrap();
//                                     self.#function_ident(second_arg);
//                                 },
//                             };
//                             quote!()
//                         }
//                         _ => {
//                             quote!(#item)
//                         }
//                     };
//                     // impl_block.extend(sig);
//                 } else {
//                     // impl_block.extend(quote!(#item));
//                 }
//             }
//             // Create a new trait implementation block
//             quote! {
//                 impl MyAdditionalTrait for #struct_ident {
//                     fn wrap_invoke(&self, method_name: &str, json_value: &Value) {
//                         match method_name {
//                             #function_map
//                             _ => println!("Method not found"),
//                         }
//                     }
//                 }
//             }
//         }
//         _ => panic!("This macro can only be applied to structs"),
//     };
//     // Return the modified impl block
//     impl_block.into()
// }

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
