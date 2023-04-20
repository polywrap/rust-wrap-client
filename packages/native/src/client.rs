use polywrap_client::{client::PolywrapClient as InnerPolywrapClient, core::invoke::Invoker, builder::types::ClientConfigHandler};
use serde_json::Value;
use std::sync::Arc;
use crate::builder::BuilderConfigContainer;

struct PolywrapClient {
  pub inner_client: InnerPolywrapClient
}

impl PolywrapClient {
  pub fn new(config_builder: Arc<BuilderConfigContainer>) -> PolywrapClient {
    let config = config_builder.inner_builder.lock().unwrap().build();
    PolywrapClient { 
      inner_client: InnerPolywrapClient::new(config)
    }
  }

  pub fn invoke_raw(&self, uri: &str, method: &str, args: Option<Vec<u8>>, env: Option<&str>) -> Vec<u8> {
    let args = if let Some(args) = args {
      Some(args.as_slice())
    } else {
      None
    };

    let env = if let Some(env) = env {
      Some(serde_json::from_str::<Value>(env).unwrap())
    } else {
      None
    };

    self.inner_client.invoke_raw(
      &uri.to_string().try_into().unwrap(),
      method,
      args,
      env,
      None
    ).unwrap()
  }
}