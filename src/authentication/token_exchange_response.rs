use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub access_token:   String,
    pub expires_in:     i16,
    pub id_token:       String,
    pub scope:          String,
    pub token_type:     String,

    #[serde(default)]
    pub refresh_token:  String
}

impl Default for Response {
     fn default() -> Response {
        Response {
            access_token: String::new(),
            expires_in: 0,
            id_token: String::new(),
            scope: String::new(),
            token_type: String::new(),
            refresh_token: String::new()
        }
    }
}

#[derive(Deserialize)]
pub struct JwtHeader {
    pub iss: String,
    pub aud: String,
    pub exp: String
}