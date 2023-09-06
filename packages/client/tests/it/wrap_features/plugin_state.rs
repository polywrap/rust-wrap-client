#[cfg(test)]
mod plugin_state_tests {
    use std::sync::{Arc, Mutex};

    use polywrap_client_builder::{ClientConfig, ClientConfigBuilder};
    use polywrap_core::wrapper::Wrapper;
    use polywrap_plugin::{module::PluginModule, error::PluginError, wrapper::PluginWrapper};

    #[derive(Debug)]
    struct CounterPlugin {
      pub counter: u32
    }

    impl PluginModule for CounterPlugin {
      fn _wrap_invoke(
        &mut self,
        _: &str,
        _: &[u8],
        _: Option<&[u8]>,
        _: std::sync::Arc<dyn polywrap_core::invoker::Invoker>,
      ) -> Result<Vec<u8>, PluginError> {
          self.counter = self.counter + 1;

          Ok(vec![])
      }
    }

  #[test]
  pub fn it_adds_concrete_plugin_without_losing_state_access() {
    let mut config = ClientConfig::new();
    let plugin = Arc::new(Mutex::new(CounterPlugin { counter: 7 }));
    let plugin_wrapper = Arc::new(PluginWrapper::new(plugin.clone()));
    config.add_wrapper("foo/bar".try_into().unwrap(), plugin_wrapper.clone() as Arc<dyn Wrapper>);

    assert_eq!(plugin.lock().unwrap().counter, 7);
  }
}