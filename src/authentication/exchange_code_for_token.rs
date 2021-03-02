use std::collections::HashMap;

#[allow(dead_code)]
pub struct ExchangeCodeForToken {
    retries: usize
}

impl ExchangeCodeForToken {
    pub fn new() -> ExchangeCodeForToken {
        ExchangeCodeForToken {
            retries: 0
        }
    }

    pub fn exchange(&mut self, code: String, client_id: String, client_secret: String, host: String, token_endpoint: String) {
        std::thread::spawn(move || {

            println!("Exchanging!");

            let redirect_uri = format!("https://{}/oauth/token", host.clone().as_str());

            println!("{}", redirect_uri);

            let parameters: HashMap<&str, &str> = [
                ("code", code.as_str()),
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("redirect_uri", redirect_uri.as_str()),
                ("grant_type", "authorization_code")
            ].iter().cloned().collect();

            let url_query_parameters = crate::utils::hashmap_to_url(parameters);
            let final_url = format!("{}{}", token_endpoint, url_query_parameters);

            if final_url.eq("") {
                eprintln!("Unable to determine full URL for exchanging access code for Token");
                return;
            }


            println!("FInal url: {}", final_url.clone());

            //Send the request
            let req = reqwest::blocking::Client::new().post(&final_url).send();
            if req.is_err() {
                eprintln!("{:?}", req.err());

                /*if self.retries > 5 {
                    eprintln!("POST request to Google to exchange a code for a token failed. Exceeded maximum amount of retries. Stopping here.");
                    return;
                } else {
                    eprintln!("POST request to Google to exchange a code for a token failed. Retrying in 30 seconds!");
                    self.retries +=1;

                    std::thread::sleep(Duration::from_secs(30));

                    self.exchange(environment, code, client_id, client_secret, host, token_endpoint);
                }*/
            }
        });
    }
}