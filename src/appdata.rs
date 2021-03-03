use tera::Tera;
use crate::environment::Environment;
use crate::threads::discovery_document::DiscoveryDocumentValues;
use std::sync::{Mutex, Arc};

pub struct AppData {
    pub tera:               Tera,
    pub environment:        Environment,
    pub discovery_document: Arc<Mutex<DiscoveryDocumentValues>>
}

impl AppData {
    pub fn new(tera: Tera, environment: Environment, discovery_document: Arc<Mutex<DiscoveryDocumentValues>>) -> AppData {
        AppData {
            tera,
            environment,
            discovery_document,
        }
    }
}