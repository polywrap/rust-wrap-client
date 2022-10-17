struct UriResolutionContext {
  resolving_uri_map: HashMap<String, bool>,
  resolution_path: Vec<String>,
  history: Vec<UriResolutionStep>
}

impl UriResolutionContext {
  fn new() -> Self {
    UriResolutionContext {
      resolving_uri_map: HashMap::new(),
      resolution_path: Vec::new(),
      history: Vec::new()
    }
  }
}