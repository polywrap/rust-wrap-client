#[macro_export]
macro_rules! impl_plugin_traits {
  ($plugin_type:ty, $(($method_name:ident, $args_type:ty)),* $(,)?) => {
    #[$crate::async_trait]
    impl $crate::module::PluginModule for $plugin_type {
      async fn _wrap_invoke(
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
              ).await?;

              Ok(serde_json::to_value(result)
                  .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?)
              }),*
              _ => panic!("Method not found"),
          }
      }
  }

  impl Into<PluginPackage> for $plugin_type {
    fn into(self) -> PluginPackage {
        let manifest = get_manifest();
        let plugin_module = Arc::new(Mutex::new(Box::new(self) as Box<dyn PluginModule>));
        PluginPackage::new(plugin_module, manifest)
    }
  }

  impl Into<PluginWrapper> for $plugin_type {
      fn into(self) -> PluginWrapper {
        let plugin_module = Arc::new(Mutex::new(Box::new(self) as Box<dyn PluginModule>));
        PluginWrapper::new(plugin_module)
      }
  }
  };
}
