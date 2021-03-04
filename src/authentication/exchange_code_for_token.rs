use std::collections::HashMap;
use serde::Deserialize;
use crate::authentication::token_exchange_response::Response;
use alcoholic_jwt::{JWKS, validate, Validation, token_kid};
use crate::threads::discovery_document::JwksKeys;


#[derive(Deserialize)]
struct IdToken {
    pub email:  String,
    pub sub:    String
}

pub fn exchange(code: String, client_id: String, client_secret: String, host: String, token_endpoint: String, jwks_keys: JwksKeys) -> Result<String, String> {
    //Redirect uri is the same as where the user was originally redirected to after logging in, for some reason.
    let redirect_uri = format!("https://{}/oauth/grant", host.clone().as_str());

    //Set the parameters, in a hashmap for easier understanding
    let client_id_clone = client_id.clone();
    let parameters: HashMap<&str, &str> = [
        ("code", code.as_str()),
        ("client_id", client_id_clone.as_str()),
        ("client_secret", client_secret.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("grant_type", "authorization_code")
    ].iter().cloned().collect();

    //Build the URL
    let url_query_parameters = crate::utils::hashmap_to_url(parameters);
    let final_url = format!("{}{}", token_endpoint, url_query_parameters);

    //Send the request
    let req = reqwest::blocking::Client::new().post(&final_url).header("Content-Length", "0").send();
    if req.is_err() {
        return Err("Something went wrong getting information from Google".to_string());
    }

    //Deserialize the response into a Response object
    let req_unwrapped = req.unwrap();
    let response: Response = req_unwrapped.json().unwrap();

    //Validate the id_token
    // 1. Validate signature
    // 2. Validate the ISS claim (needs to be https://accounts.google.com or accounts.google.com)
    // 3. Validate that the `aud` field is equal to our client id
    // 4. Validate that the `exp` field has not yet passed (time)
    // See: https://developers.google.com/identity/protocols/oauth2/openid-connect#validatinganidtoken

    //Quick decoding.
    //JWT is built up like: <base64 header>.<base64 payload>.<signature>
    //We're interested in the headers
    let id_token = response.id_token.clone();
    let id_parts: Vec<&str> = id_token.split(".").collect();

    //JKWS can only be created by deserialization, for some reason. So we serialize our data, and serialize it back again /s
    let jwks_serialized = serde_json::to_string(&jwks_keys).unwrap();
    let jwks: JWKS = serde_json::from_str(&jwks_serialized).unwrap();

    //Set all the things we want to validate,
    // - Issuer (step 2)
    // - Audience (step 3)
    // - Expiration (step 4)
    let validations = vec![
        Validation::Issuer("https://accounts.google.com".into()),
        Validation::Audience(client_id.clone()),
        Validation::NotExpired
    ];

    //Get the kid we are going to use, since there are multiple keys
    let kid = token_kid(&id_token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token!");

    //Get the JWK for the selected kid
    let jwk = jwks.find(&kid).expect("Specified key not found in set!");

    //Validate the JWT
    let jwt_validation_result = validate(&id_token, &jwk, validations);
    if jwt_validation_result.is_err() {
        eprintln!("JWT is not valid!");
        return Err("JWT is not valid.".to_string());
    }

    //Get the user ID from the JWT
    let jwt_payload_base64: &str = id_parts[1];
    let jwt_payload = String::from_utf8(base64::decode(jwt_payload_base64.as_bytes()).unwrap()).unwrap();

    let id_token: IdToken = serde_json::from_str(&jwt_payload).unwrap();
    //Send our data to the database
    //the refresh_token is not always present, if it is not we do not want to override what is already in the database
    let epoch_expires_in = chrono::Utc::now() + chrono::Duration::seconds(response.expires_in as i64);

    unsafe {
        if response.refresh_token.is_empty() {
            crate::DATABASE.insert_refreshed_grant(id_token.sub.clone(), response.access_token, epoch_expires_in.timestamp());
        } else {
            crate::DATABASE.insert_grant(id_token.sub.clone(), response.access_token, response.id_token, response.scope, response.token_type, response.refresh_token, epoch_expires_in.timestamp());
        }
    }

    Ok(id_token.sub)
}
