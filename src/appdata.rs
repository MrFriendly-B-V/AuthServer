use tera::Tera;
use crate::environment::Environment;
use crate::threads::discovery_document::DiscoveryDocumentValues;
use crate::database::database::Database;
use std::sync::{Mutex, Arc};


pub struct AppData {
    pub tera:               Tera,
    pub environment:        Environment,
    pub database:           Arc<Mutex<Database>>,
    pub discovery_document: Arc<Mutex<DiscoveryDocumentValues>>
}

impl AppData {
    pub fn new(tera: Tera, environment: Environment, database: Arc<Mutex<Database>>, discovery_document: Arc<Mutex<DiscoveryDocumentValues>>) -> AppData {
        AppData {
            tera,
            environment,
            database,
            discovery_document,
        }
    }
}