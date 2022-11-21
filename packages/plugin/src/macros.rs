#[macro_export]
macro_rules! base_impl_plugin_module {
  ($plugin_type:ty, $(($method_name:ident, $args_type:ty)),* $(,)?) => {
    impl $crate::module::PluginModule for $plugin_type {
      fn _wrap_invoke(
          &mut self,
          method_name: &str,
          params: &serde_json::Value,
          invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
      ) -> Result<serde_json::Value, polywrap_core::error::Error> {
          match method_name {
              $(stringify!($method_name) => {
                let result = self.$method_name(
                  &serde_json::from_value::<$args_type>(params.clone())
                      .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?,
                  invoker,
              )?;

              Ok(serde_json::to_value(result)
                  .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?)
              }),*
              _ => panic!("Method not found"),
          }
      }
  }
  };
}