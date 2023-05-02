use super::uri_resolution_context::{UriResolutionStep, UriPackageOrWrapper};

pub fn get_uri_resolution_path(history: &[UriResolutionStep]) -> Vec<UriResolutionStep> {
  history.into_iter()
  .filter(|uri_resolution_step| {
    if let Ok(uri_package_or_wrapper) = &uri_resolution_step.result {
      match uri_package_or_wrapper {
          UriPackageOrWrapper::Uri(uri) => {
            uri.to_string() != uri_resolution_step.source_uri.to_string()
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
  })
  .cloned()
  .map(|mut uri_resolution_step| {
    if let Some(subhistory) = &uri_resolution_step.sub_history {
      if !subhistory.is_empty() {
        uri_resolution_step.sub_history = Some(get_uri_resolution_path(&subhistory));
        uri_resolution_step
      } else {
        uri_resolution_step
      }
    } else {
      uri_resolution_step
    }
  }).collect()
}