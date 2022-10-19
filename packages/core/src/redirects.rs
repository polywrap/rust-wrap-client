use std::{collections::HashMap};

use crate::{client::UriRedirect, error::CoreError, uri::uri::Uri};

pub fn apply_redirects(uri: &Uri, redirects: &Vec<UriRedirect>) -> Result<Uri, CoreError> {
    let mut redirect_from_to_map = HashMap::new();

    for redirect in redirects {
        if redirect.from.to_string() == uri.to_string() {
            return Err(CoreError::RedirectsError(
                format!(
                    "Redirect missing the from property.\nEncountered while resolving {}",
                    uri.to_string()
                ),
                redirect_from_to_map,
            ));
        }

        if redirect_from_to_map.contains_key(&redirect.from.to_string()) {
            continue;
        }

        redirect_from_to_map.insert(redirect.from.to_string(), redirect.to.to_string());
    }

    let mut final_uri = uri.to_string();
    let mut visited_uris: HashMap<String, bool> = HashMap::new();

    while redirect_from_to_map.contains_key(&final_uri) {
        visited_uris.insert(final_uri.to_string(), true);

        final_uri = redirect_from_to_map.get(&final_uri).unwrap().to_string();

        if visited_uris.contains_key(&final_uri) {
            return Err(CoreError::RedirectsError(
                format!(
                    "Redirect loop detected while resolving {}",
                    uri.to_string()
                ),
                redirect_from_to_map,
            ));
        }
    }

    Ok(Uri::from(final_uri))
}