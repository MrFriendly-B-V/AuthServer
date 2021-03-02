use actix_web::{post, web, HttpResponse};
use actix_web::web::Bytes;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response {
    access_token:   String,
    expires_in:     String,
    id_token:       String,
    scope:          String,
    token_type:     String,
    refresh_token:  String
}

#[post("/oauth/token")]
#[allow(unused_variables)]
pub async fn post_oauth_token(appdata: web::Data<crate::appdata::AppData>, bytes: Bytes) -> HttpResponse {
    let body = String::from_utf8(bytes.to_vec()).map_err(|_| HttpResponse::BadRequest().finish()).unwrap();

    println!("Received body: {}", body);

    //let response: Response = serde_json::from_str(&body).unwrap();
    //println!("Access token: {}", response.access_token);

    HttpResponse::Ok().finish()
}