use polywrap_client::{client::PolywrapClient, core::{uri::Uri, invoker::Invoker}};

pub fn invoke_client(uri: &str, method: &str, args: &[u8], client: &PolywrapClient) -> Vec<u8> {
  let result = client.invoke_raw(
      &Uri::try_from(uri).unwrap(),
      method, 
      Some(args),
      None,
      None,
  ).unwrap();

  result
}