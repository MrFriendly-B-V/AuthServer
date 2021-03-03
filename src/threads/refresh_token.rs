use std::collections::HashMap;
use serde::Deserialize;

const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

#[derive(Deserialize)]
struct TokenResponse {
    access_token:   String,
    expires_in:     i16
}

pub fn refresh_token_thread() {

    std::thread::spawn(|| {
       loop {
           //Calculate the epoch time 10 minutes into the future
           let checking_expiry_dt = chrono::Utc::now() + chrono::Duration::minutes(10);
           let checking_expiry = checking_expiry_dt.timestamp();

           let to_refresh_tokens = unsafe {
               crate::DATABASE.get_expiring_tokens(checking_expiry)
           };

           for item in to_refresh_tokens {
               let user_id = item.0;
               let refresh_token = item.1;

               let environment = crate::get_environment();

               let parameters: HashMap<&str, &str> = [
                   ("client_id", environment.get_google_client_id().as_str()),
                   ("client_secret", environment.get_google_client_secret().as_str()),
                   ("refresh_token", refresh_token.as_str()),
                   ("grant_type", "refresh_token")
               ].iter().cloned().collect();

               let url_params_built = crate::utils::hashmap_to_url(parameters);
               let final_url = format!("{}{}", TOKEN_ENDPOINT, url_params_built);

               let google_response = reqwest::blocking::Client::new().post(&final_url).header("Content-Length", "0").send();
               if google_response.is_err() { continue; }
               let token_response: TokenResponse = google_response.unwrap().json().unwrap();

               let epoch_expires_in = chrono::Utc::now() + chrono::Duration::seconds(token_response.expires_in as i64);
               unsafe {
                   crate::DATABASE.insert_refreshed_grant(user_id, token_response.access_token, epoch_expires_in.timestamp());
               }
           }

           std::thread::sleep(std::time::Duration::from_secs(30));
       }
    });
}