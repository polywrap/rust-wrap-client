use crate::uri::Uri;
use std::collections::HashMap;

/// Defines which wraps implement a certain interface.
pub type InterfaceImplementations = HashMap<Uri, Vec<Uri>>;
