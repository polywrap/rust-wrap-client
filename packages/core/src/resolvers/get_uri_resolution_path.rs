use super::uri_resolution_context::{UriResolutionStep, UriPackageOrWrapper};

pub fn get_uri_resolution_path(history: Vec<UriResolutionStep>) -> Vec<UriResolutionStep> {
  history.into_iter()
  .filter(|x| {
    if let Ok(uri_package_or_wrapper) = &x.result {
      match uri_package_or_wrapper {
          UriPackageOrWrapper::Uri(uri) => {
            uri.to_string() != x.source_uri.to_string()
          },
          UriPackageOrWrapper::Package(_, _) => {
            true
          },
          UriPackageOrWrapper::Wrapper(_, _) => {
            true
          }
      }
    } else {
      true
    }
  }).map(|mut x| {
    if let Some(subhistory) = &x.sub_history {
      if subhistory.len() > 0 {
        x.sub_history = Some(get_uri_resolution_path(subhistory.clone()));
        x
      } else {
        x
      }
    } else {
      x
    }
  }).collect()
}