mod http_controllers;
mod environment;
mod database;
mod appdata;
mod threads;
mod utils;
mod authentication;

use http_controllers::*;
use actix_web::{HttpServer, App};
use environment::Environment;
use std::process::exit;
use crate::appdata::AppData;
use crate::database::database::Database;
use std::sync::{Arc, Mutex};
use crate::threads::discovery_document::{DiscoveryDocumentValues, start_discovery_document_thread};

//TODO this really needs to be fixed
pub static mut DATABASE: Database = crate::database::database::DATABASE;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Perform preflight checks:
    // - Check if all environmental variables exist
    println!("Running preflight checks...");
    if !Environment::check_environment() {
        eprintln!("Preflight checks failed: Some environmental variables are not set. Exiting!");
        exit(1);
    }
    println!("Preflight checks completed.");

    //Spin up a thread for the discovery document
    println!("Starting Discovery Document thread.");
    let discovery_document_values = Arc::new(Mutex::new(DiscoveryDocumentValues::init_empty()));
    let discovery_document_values_for_dd_thread = discovery_document_values.clone();
    start_discovery_document_thread(discovery_document_values_for_dd_thread);

    //Start the refresh token thread
    println!("Starting RefreshToken thread.");
    threads::refresh_token::refresh_token_thread();

    //Start the Actix HTTP web server
    println!("Starting HTTP server.");
    let environment = Environment::get_environment().unwrap();
    HttpServer::new(move || {
        //Create a Tera instance (Template engine)
        let mut tera = tera::Tera::new("templates/**/*").expect("Tera error!");
        tera.autoescape_on(vec![]);

        App::new()
            .data(AppData { tera, discovery_document: discovery_document_values.clone() })

            //GET endpoints
            .service(get_oauth_login::get_oauth_login)
            .service(get_oauth_grant::get_oauth_grant)

            //POST endpoints
            .service(post_check_session::post_check_session)
            .service(post_get_token::post_get_token)
        })

        //Bind address is based off of environmental variables
        .bind(format!("{bind_address}:{port}", bind_address = environment.bind_address, port = environment.port))?
        .run()
        .await
}