use tera::Tera;
use crate::threads::discovery_document::DiscoveryDocumentValues;
use std::sync::{Mutex, Arc};

pub struct AppData {
    pub tera:               Tera,
    pub discovery_document: Arc<Mutex<DiscoveryDocumentValues>>
}