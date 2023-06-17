use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use polywrap_client::core::resolution::uri_resolution_context::{
    UriResolutionContext, UriResolutionStep,
};

use crate::uri::FFIUri;

use super::uri_package_or_wrapper::FFIUriPackageOrWrapper;

pub struct FFIUriResolutionStep {
    pub source_uri: Arc<FFIUri>,
    pub result: Box<dyn FFIUriPackageOrWrapper>,
    pub description: Option<String>,
    pub sub_history: Option<Vec<FFIUriResolutionStep>>,
}

pub struct FFIUriResolutionContext(pub Arc<Mutex<UriResolutionContext>>);

impl FFIUriResolutionContext {
    pub fn new() -> Self {
        FFIUriResolutionContext(Arc::new(Mutex::new(UriResolutionContext::new())))
    }

    pub fn set_resolution_path(&self, resolution_path: Vec<String>) {
        self.0.lock().unwrap().resolution_path(resolution_path);
    }

    pub fn set_history(&self, history: Vec<FFIUriResolutionStep>) {
        self.0
            .lock()
            .unwrap()
            .history(history.into_iter().map(|u| u.into()).collect());
    }

    pub fn set_resolving_uri_map(&self, resolving_uri_map: HashMap<String, bool>) {
        self.0.lock().unwrap().resolving_uri_map(resolving_uri_map);
    }

    pub fn set_start_resolving(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().start_resolving(&uri.0);
    }

    pub fn set_stop_resolving(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().stop_resolving(&uri.0)
    }

    pub fn track_step(&self, step: FFIUriResolutionStep) {
        self.0.lock().unwrap().track_step(step.into());
    }

    pub fn get_history(&self) -> Vec<FFIUriResolutionStep> {
        self.0
            .lock()
            .unwrap()
            .get_history()
            .clone()
            .into_iter()
            .map(|u| u.into())
            .collect()
    }

    pub fn get_resolution_path(&self) -> Vec<Arc<FFIUri>> {
        self.0
            .lock()
            .unwrap()
            .get_resolution_path()
            .into_iter()
            .map(|u| Arc::new(FFIUri(u)))
            .collect()
    }

    pub fn create_sub_history_context(&self) -> Arc<FFIUriResolutionContext> {
        let res_context = self.0.lock().unwrap().create_sub_history_context();

        Arc::new(FFIUriResolutionContext(Arc::new(Mutex::new(res_context))))
    }

    pub fn create_sub_context(&self) -> Arc<FFIUriResolutionContext> {
        let res_context = self.0.lock().unwrap().create_sub_history_context();

        Arc::new(FFIUriResolutionContext(Arc::new(Mutex::new(res_context))))
    }
}

impl From<UriResolutionStep> for FFIUriResolutionStep {
    fn from(value: UriResolutionStep) -> Self {
        FFIUriResolutionStep {
            source_uri: Arc::new(FFIUri(value.source_uri)),
            result: Box::new(value.result.unwrap()),
            description: value.description,
            sub_history: value
                .sub_history
                .map(|sub_history| sub_history.into_iter().map(|step| step.into()).collect()),
        }
    }
}

impl From<FFIUriResolutionStep> for UriResolutionStep {
    fn from(value: FFIUriResolutionStep) -> Self {
        UriResolutionStep {
            source_uri: value.source_uri.0.clone(),
            result: Ok(value.result.into()),
            description: value.description,
            sub_history: value
                .sub_history
                .map(|sub_history| sub_history.into_iter().map(|step| step.into()).collect()),
        }
    }
}
