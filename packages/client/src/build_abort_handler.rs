use std::sync::Arc;

use polywrap_core::uri::Uri;

pub fn build_abort_handler(custom_abort_handler: Option<Arc<dyn Fn(Uri, String, String) + Send + Sync>>, uri: Uri, method: String) -> Box<dyn Fn(String) + Send + Sync> {
  match custom_abort_handler {
      Some(abort) => {
          Box::new(move |error_message: String| {
              abort(uri.clone(), method.clone(), error_message)
          })
      },
      None => {
          Box::new(move |error_message: String| {
              panic!(
                  r#"Wrapper aborted execution.
                  URI: {uri}  
                  Method: {method}
                  Message: {error_message}
              "#
              );
          })
      }
  }
}
