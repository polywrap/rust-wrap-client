use polywrap_client::{builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};

use crate::utils::{instantiate_from_ptr, into_raw_ptr_and_forget};

#[no_mangle]
pub extern "C" fn create_client(builder_config_ptr: *mut BuilderConfig) -> *mut PolywrapClient {
  let builder = instantiate_from_ptr(builder_config_ptr);
  let config = builder.build();

  let client = PolywrapClient::new(config);
  into_raw_ptr_and_forget(client) as *mut PolywrapClient
}