use std::collections::HashMap;
use crate::client::Client;

use crate::{uri::uri::Uri, error::CoreError};
use crate::wrapper::{Wrapper};

pub struct UriWrapper {
  // uri: Uri,
  // wrapper: Box<dyn Wrapper>,
}

pub enum UriPackageOrWrapper {
    Uri(Uri),
    Wrapper(Uri, UriWrapper),
}

pub struct UriResolutionStep {
  source_uri: Uri,
  result: Result<UriPackageOrWrapper, CoreError>,
  description: Option<String>,
  sub_history: Option<Vec<UriResolutionStep>>,
}

pub struct UriResolutionContext {
  resolving_uri_map: HashMap<String, bool>,
  resolution_path: Vec<String>,
  history: Vec<UriResolutionStep>
}

impl UriResolutionContext {
  pub fn new() -> Self {
    UriResolutionContext {
      resolving_uri_map: HashMap::new(),
      resolution_path: Vec::new(),
      history: Vec::new()
    }
  }

  pub fn resolution_path<'a>(&'a mut self, resolution_path: Vec<String>) -> &'a Self {
    self.resolution_path = resolution_path;
    self
  }

  pub fn history<'a>(&'a mut self, history: Vec<UriResolutionStep>) -> &'a Self {
    self.history = history;
    self
  }

  pub fn resolving_uri_map<'a>(&'a mut self, resolving_uri_map: HashMap<String, bool>) -> &'a Self {
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
    self.resolution_path.iter().map(|uri| Uri::new(uri)).collect()
  }
}