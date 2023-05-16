use crate::{package::WrapPackage, wrapper::Wrapper, error::Error};
use std::{collections::HashMap, sync::Arc, vec};

use crate::{uri::Uri};

#[derive(Clone)]
pub enum UriPackageOrWrapper {
  Uri(Uri),
  Wrapper(Uri, Arc<dyn Wrapper>),
  Package(Uri, Arc<dyn WrapPackage>),
}

impl UriPackageOrWrapper {
  pub fn uri(&self) -> Uri {
    match self {
        UriPackageOrWrapper::Uri(uri) => uri.clone(),
        UriPackageOrWrapper::Wrapper(uri, _) => uri.clone(),
        UriPackageOrWrapper::Package(uri, _) => uri.clone(),
    }
  }
}

#[derive(Clone)]
pub struct UriResolutionStep {
    pub source_uri: Uri,
    pub result: Result<UriPackageOrWrapper, Error>,
    pub description: Option<String>,
    pub sub_history: Option<Vec<UriResolutionStep>>,
}

#[derive(Default)]
pub struct UriResolutionContext {
    resolving_uri_map: HashMap<String, bool>,
    resolution_path: Vec<String>,
    history: Vec<UriResolutionStep>,
}

impl UriResolutionContext {
    pub fn new() -> Self {
        UriResolutionContext::default()
    }

    pub fn resolution_path(&mut self, resolution_path: Vec<String>) -> &Self {
        self.resolution_path = resolution_path;
        self
    }

    pub fn history(&mut self, history: Vec<UriResolutionStep>) -> &Self {
        self.history = history;
        self
    }

    pub fn resolving_uri_map(&mut self, resolving_uri_map: HashMap<String, bool>) -> &Self {
        self.resolving_uri_map = resolving_uri_map;
        self
    }

    pub fn is_resolving(&self, uri: &Uri) -> bool {
        self.resolving_uri_map.contains_key(&uri.to_string())
    }

    pub fn start_resolving(&mut self, uri: &Uri) {
        self.resolving_uri_map.insert(uri.to_string(), true);
        self.resolution_path.push(uri.to_string());
    }

    pub fn stop_resolving(&mut self, uri: &Uri) {
        self.resolving_uri_map.remove(&uri.to_string());
    }

    pub fn track_step(&mut self, step: UriResolutionStep) {
        self.history.push(step);
    }

    pub fn get_history(&self) -> &Vec<UriResolutionStep> {
        &self.history
    }

    pub fn get_resolution_path(&self) -> Vec<Uri> {
        self.resolution_path
            .iter()
            .map(|uri| Uri::new(uri))
            .collect()
    }

    pub fn create_sub_history_context(&self) -> UriResolutionContext {
        UriResolutionContext {
            resolving_uri_map: self.resolving_uri_map.clone(),
            resolution_path: self.resolution_path.clone(),
            history: vec![]
        }
    }

    pub fn create_sub_context(&self) -> UriResolutionContext {
        UriResolutionContext {
            resolving_uri_map: self.resolving_uri_map.clone(),
            resolution_path: vec![],
            history: vec![]
        }
    }
}
