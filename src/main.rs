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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running preflight checks...");
    let environment = get_environment();

    println!("Preflight checks completed.");
    println!("Starting web server.");

    //Database instance
    let database = Arc::new(Mutex::new(Database::new(get_environment())));

    //Discovery Document thread
    let discovery_document_values = Arc::new(Mutex::new(DiscoveryDocumentValues::init_empty()));
    let discovery_document_values_for_dd_thread = discovery_document_values.clone();
    let _ = start_discovery_document_thread(discovery_document_values_for_dd_thread);

    HttpServer::new(move || {
        let mut tera = tera::Tera::new("templates/**/*").expect("Tera error!");
        tera.autoescape_on(vec![]);

        App::new()
            .data(AppData::new(tera, get_environment(), database.clone(), discovery_document_values.clone()))

            //Web endpoints
            .service(get_oauth_login::get_oauth_login)
            .service(get_oauth_grant::get_oauth_grant)
            .service(post_oauth_token::post_oauth_token)
        })
        .bind(format!("{}:{}", environment.get_bind_address(), environment.get_port()))?
        .run()
        .await
}

fn get_environment() -> Environment {
    use std::env;

    let port = env::var("PORT");
    if port.is_err() {
        eprintln!("Environmental variable 'PORT' not set.");
        exit(1);
    }

    let bind_address = env::var("BIND_ADDRESS");
    if bind_address.is_err() {
        eprintln!("Environmental variable 'BIND_ADDRESS' not set.");
        exit(1);
    }

    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET");
    if google_client_secret.is_err() {
        eprintln!("Environmental variable 'GOOGLE_CLIENT_SECRET' not set.");
        exit(1);
    }

    let google_client_id = env::var("GOOGLE_CLIENT_ID");
    if google_client_id.is_err() {
        eprintln!("Environmental variable 'GOOGLE_CLIENT_ID' not set.");
        exit(1);
    }

    let host = env::var("HOST");
    if host.is_err() {
        eprintln!("Environmental variable 'HOST' not set.");
        exit(1);
    }

    let mysql_host = env::var("MYSQL_HOST");
    if mysql_host.is_err() {
        eprintln!("Environmental variable 'MYSQL_HOST' not set.");
        exit(1);
    }

    let mysql_database = env::var("MYSQL_DATABASE");
    if mysql_database.is_err() {
        eprintln!("Environmental variable 'MYSQL_DATABASE' not set.");
        exit(1);
    }

    let mysql_username = env::var("MYSQL_USERNAME");
    if mysql_username.is_err() {
        eprintln!("Environmental variable 'MYSQL_USERNAME' not set.");
        exit(1);
    }

    let mysql_password = env::var("MYSQL_PASSWORD");
    if mysql_password.is_err() {
        eprintln!("Environmental variable 'MYSQL_PASSWORD' not set.");
        exit(1);
    }

    let api_tokens = env::var("API_TOKENS");
    if api_tokens.is_err() {
        eprintln!("Environmental variable 'API_TOKENS' not set.");
        exit(1);
    }

    //Build the API_TOKENS value
    let mut api_tokens_vec = Vec::new();
    {
        let api_tokens_string = api_tokens.unwrap();
        for item in api_tokens_string.split(",") {
            api_tokens_vec.push(item.to_string());
        }
    }

    Environment::new(
        port.unwrap(),
        bind_address.unwrap(),
        google_client_secret.unwrap(),
        google_client_id.unwrap(),
        host.unwrap(),
        mysql_host.unwrap(),
        mysql_database.unwrap(),
        mysql_username.unwrap(),
        mysql_password.unwrap(),
        api_tokens_vec
    )
}