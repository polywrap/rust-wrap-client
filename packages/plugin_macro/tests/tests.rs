#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use polywrap_plugin_macro::{plugin_struct, plugin_impl};
    use polywrap_plugin::{module::PluginModule, error::PluginError};
    use async_trait::async_trait;
    use serde::{Serialize, Deserialize};
    use polywrap_core::invoke::Invoker;
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct Args {
      boo: String
    }

    #[async_trait]
    pub trait Module {
      async fn foo_method(&mut self, args: &Args, a: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError>;
    }

    #[test]
    fn add_env_field() {
        

        #[plugin_struct]
        struct Foo {
            a: String
        }

        let bar = Foo {
            env: json!({}),
            a: "sss".to_string()
        };

        impl Foo {
            pub fn new(a: String) -> Self {
                Self {
                    env: json!({}),
                    a: "sss".to_string()
                }
            }

            pub fn methoda(&self, b: i32) -> u32 {
                b.try_into().unwrap()
            }

            pub fn methodb(&mut self, b: i32) {
                self.env = json!({})
            }
        }

        #[async_trait]
        impl Module for Foo {
          async fn foo_method(&mut self, arg: &Args, s: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError> {
            Ok(Some(vec![0]))
          }
        }

        assert_eq!(bar.env, json!({}));
    }
}