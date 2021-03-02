use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DISCOVERY_DOCUMENT_URI: &str = "https://accounts.google.com/.well-known/openid-configuration";

#[derive(Deserialize)]
pub struct DiscoveryDocumentValues {
    pub issuer:                  String,
    pub authorization_endpoint:  String,
    pub token_endpoint:          String,
    pub jwks_uri:                String,
    pub userinfo_endpoint:       String,
    pub revocation_endpoint:     String,

    #[serde(default)]
    pub jwks_keys:               JwksKeys
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct JwksKeys {
    keys: Vec<SignatureKey>
}

impl Default for JwksKeys {
    fn default() -> Self {
        JwksKeys {
            keys: Vec::new()
        }
    }
}

impl JwksKeys {
    pub fn new() -> JwksKeys {
        JwksKeys {
            keys: Vec::new()
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SignatureKey {
    kid:    String,
    kty:    String,
    alg:    String,
    n:      String,

    #[serde(rename(deserialize = "use"))]
    key_uses:    String,
    e:      String,
}

impl DiscoveryDocumentValues {
    pub fn init_empty() -> DiscoveryDocumentValues {
        DiscoveryDocumentValues {
            issuer: String::new(),
            authorization_endpoint: String::new(),
            token_endpoint: String::new(),
            jwks_uri: String::new(),
            userinfo_endpoint: String::new(),
            revocation_endpoint: String::new(),
            jwks_keys: JwksKeys::new()
        }
    }
}

/**
Get the keys specified at the jwks_uri from Google
*/
fn get_jwks_keys(jwks_uri: String) -> Result<JwksKeys, reqwest::Error>{
    println!("Getting JWKS Keys...");

    let req = reqwest::blocking::get(&jwks_uri);
    if req.is_err() {
        eprintln!("Something went wrong fetching JWKS Keys!");
        return Err(req.err().unwrap());
    }
    let response: JwksKeys = req.unwrap().json().unwrap();

    println!("Successfully fetched JWKS Keys.");
    Ok(response)
}

/**
Spawns a new std::thread which will fetch the Discovery Document from Google periodically
*/
pub fn start_discovery_document_thread(discovery_document_values: Arc<Mutex<DiscoveryDocumentValues>>) {
    std::thread::spawn(move || {
       loop {
           println!("Getting Discovery Document from Google...");

           //Make a request to Google for the Discovery Document, check if it failed
           let dd_req = reqwest::blocking::get(DISCOVERY_DOCUMENT_URI);
           if dd_req.is_err() {
               eprintln!("Something went wrong fetching the Discovery Document from Google! Retrying in 30 seconds.");
               std::thread::sleep(Duration::from_secs(30));
               continue;
           }

           //Deserialize the response into DiscoveryDocumentValues
           let dd_response: DiscoveryDocumentValues = dd_req.unwrap().json().unwrap();

           //We have to wrap it in brackets here. If we don't, we can never read the DiscoveryDocumentValues in another thread,
           //because it's still in scope here, because we are sleeping later. By wrapping it in {}
           //it goes out of scope before the sleep, and we can read it in other threads
           {
               let mut dd_values = discovery_document_values.lock().unwrap();
               dd_values.issuer = dd_response.issuer;
               dd_values.authorization_endpoint = dd_response.authorization_endpoint;
               dd_values.token_endpoint = dd_response.token_endpoint;
               dd_values.jwks_uri = dd_response.jwks_uri.clone();
               dd_values.userinfo_endpoint = dd_response.userinfo_endpoint;
               dd_values.revocation_endpoint = dd_response.revocation_endpoint;
           }

           println!("Discovery Document fetched.");

           //Next we should fetch the jwks keys found at the jwks_uri
           let jwks_keys = get_jwks_keys(dd_response.jwks_uri.clone());
           if jwks_keys.is_err() {
               eprintln!("{:?}", jwks_keys.err().unwrap());
           } else {
               let mut dd_values = discovery_document_values.lock().unwrap();
               dd_values.jwks_keys = jwks_keys.unwrap();
           }

           //Sleep for 1 hour, then we fetch the document again
           std::thread::sleep(Duration::from_secs(3600));
       }
    });
}

