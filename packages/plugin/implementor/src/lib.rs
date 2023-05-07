use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

use syn::{parse, parse_macro_input, ItemImpl };


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
pub fn plugin_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(input as ItemImpl);
    let _ = parse_macro_input!(args as parse::Nothing);

    let struct_ident = item_impl.clone().self_ty;

    let mut method_idents: Vec<(Ident, String, bool, Option<bool>)> = vec![];

    for item in item_impl.clone().items {
        match item {
            syn::ImplItem::Method(method) => {
              let function_ident = &method.sig.ident;
              let env_is_option = if &method.sig.inputs.len() > &3 {
                let env = &method.sig.inputs[3];
                let env_str = quote! { #env }.to_string();
                
                Some(env_str.contains("Option <"))
              } else {
                None
              };
              
              let output_type = match &method.sig.output {
                syn::ReturnType::Default => quote! { () },
                syn::ReturnType::Type(_, ty) => quote! { #ty },
              };
              let output_type = quote! { #output_type }.to_string();
              let function_ident_str =
                  snake_case_to_camel_case(&function_ident.to_string());
              let output_is_option = output_type.contains("Option <");

              method_idents.push((
                  function_ident.clone(),
                  function_ident_str.clone(),
                  output_is_option,
                  env_is_option
              ));
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
        .map(|(_, (ident, ident_str, output_is_option, env_is_option))| {
            let args = if let Some(env_is_option) = env_is_option {
              let env = if env_is_option { 
                quote! {
                  if let Some(e) = env {
                    _decoded_env = JSON::from_value(e.clone()).unwrap();
                    Some(&_decoded_env)
                  } else {
                    None
                  }
                }
              } else {
                quote! {
                  if let Some(e) = env {
                    _decoded_env = JSON::from_value(e.clone()).unwrap();
                    &_decoded_env
                  } else {
                    panic!("Env must be defined for method '{}'", #ident_str)
                  }
                }
              };

              quote! {
                &polywrap_msgpack::decode(&params).unwrap(),
                invoker,
                #env
              }
            } else {
              quote! {
                &polywrap_msgpack::decode(&params).unwrap(),
                invoker
              }
            };

            let output = if output_is_option {
              quote! {
                if let Some(r) = result {
                  Ok(polywrap_msgpack::serialize(&r)?)
                } else {
                  Ok(vec![])
                }
              }
            } else {
              quote! {
                Ok(polywrap_msgpack::serialize(&result)?)
              }
            };
          
            quote! {
                #ident_str => {
                  let mut _decoded_env = JSON::Value::Null;
                  let result = self.#ident(
                    #args
                  )?;

                  #output
                }
              }
        });

    let module_impl = quote! {
        impl polywrap_plugin::module::PluginModule for #struct_ident {
          fn _wrap_invoke(
            &mut self,
            method_name: &str,
            params: &[u8],
            env: Option<&JSON::Value>,
            invoker: Arc<dyn polywrap_core::invoker::Invoker>,
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
