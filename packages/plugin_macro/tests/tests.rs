#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use wrap_manifest_schemas::versions::WrapManifest;
    use polywrap_plugin_macro::{plugin_impl};
    use polywrap_plugin::{error::PluginError};
    
    use serde::{Serialize, Deserialize};
    use polywrap_core::{invoke::Invoker, env::Env};
    

    #[derive(Serialize, Deserialize)]
    pub struct Args {
      boo: String
    }

    pub trait Module {
      fn foo_method(&mut self, args: &Args, e: Option<Env>, a: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError>;
    }

    fn get_manifest() -> WrapManifest {
      todo!()
    }

    #[test]
    fn add_env_field() {
        
        #[derive(Debug)]
        struct Foo {
            a: String
        }

        let _bar = Foo {
            a: "sss".to_string()
        };

        impl Foo {
            pub fn new(_a: String) -> Self {
                Self {
                    a: "sss".to_string()
                }
            }

            pub fn methoda(&self, b: i32) -> u32 {
                b.try_into().unwrap()
            }
        }

        #[plugin_impl]
        impl Module for Foo {
          fn foo_method(&mut self, _arg: &Args, _: Option<Env>, _s: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError> {
            Ok(Some(vec![0]))
          }
        }
    }
}