use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput, ItemImpl, ItemStruct };

fn snake_case_to_camel_case(s: &str) -> String {
    s.split('_')
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                s.chars().next().unwrap().to_uppercase().collect::<String>() + &s[1..]
            }
        })
        .collect()
}

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

    quote! {
        #[derive(polywrap_plugin_macro::Plugin)]
        #item_struct
    }
    .into()
}

#[proc_macro_attribute]
pub fn plugin_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(input as ItemImpl);
    let _ = parse_macro_input!(args as parse::Nothing);

    let struct_ident = item_impl.clone().self_ty;

    let mut method_idents: Vec<(Ident, String, Ident, bool)> = vec![];

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
                            _ => panic!("Wrong argument type"),
                        };
                        let function_ident = &method.sig.ident;
                        let function_ident_str =
                            snake_case_to_camel_case(&function_ident.to_string());

                        let output_is_option = quote!{
                            #method.sig.output
                        }.to_string().contains("Option <");


                        method_idents.push((
                            function_ident.clone(),
                            function_ident_str.clone(),
                            function_input.unwrap().clone(),
                            output_is_option,
                        ))
                    }
                    _ => panic!("Wrong number of arguments"),
                };
            }
            _ => panic!("Wrong function signature"),
        }
    }

    let supported_methods =
        method_idents
            .clone()
            .into_iter()
            .enumerate()
            .map(|(_, (_, ident_str, _, _))| {
                quote! {
                  #ident_str
                }
            });

    let methods = method_idents
        .into_iter()
        .enumerate()
        .map(|(_, (ident, ident_str, args, output_is_option))| {
            if output_is_option {
                quote! {
                    #ident_str => {
                      let result = self.#ident(
                        &polywrap_msgpack::decode::<#args>(params.clone())?,
                        invoker,
                      )?;

                      if let Some(r) = result {
                        Ok(polywrap_msgpack::serialize(r)?)
                      } else {
                        Ok(vec![])
                      }
                    }
                  }
            } else {
                quote! {
                  #ident_str => {
                    let result = self.#ident(
                      &polywrap_msgpack::decode::<#args>(params.clone())?,
                      invoker,
                    )?;
    
                    Ok(polywrap_msgpack::serialize(result)?)
                  }
                }
            }
        });

    let module_impl = quote! {
        impl polywrap_plugin::module::PluginModule for #struct_ident {
          fn _wrap_invoke(
            &mut self,
            method_name: &str,
            params: &[u8],
            invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
        ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
                let supported_methods = vec![#(#supported_methods),*];
                match method_name {
                    #(#methods)*
                    _ => panic!("Method '{}' not found. Supported methods: {:#?}", method_name, supported_methods),
                }
            }
        }
    };

    let from_impls = quote! {
      impl From<#struct_ident> for polywrap_plugin::package::PluginPackage {
        fn from(plugin: #struct_ident) -> polywrap_plugin::package::PluginPackage {
            let plugin_module = Arc::new(std::sync::Mutex::new(Box::new(plugin) as Box<dyn polywrap_plugin::module::PluginModule>));
            polywrap_plugin::package::PluginPackage::new(plugin_module, get_manifest())
        }
      }

      impl From<#struct_ident> for polywrap_plugin::wrapper::PluginWrapper {
        fn from(plugin: #struct_ident) -> polywrap_plugin::wrapper::PluginWrapper {
            let plugin_module = Arc::new(std::sync::Mutex::new(Box::new(plugin) as Box<dyn polywrap_plugin::module::PluginModule>));
            polywrap_plugin::wrapper::PluginWrapper::new(plugin_module)
        }
      }
    };

    quote! {
        #item_impl

        #module_impl

        #from_impls
    }
    .into()
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
