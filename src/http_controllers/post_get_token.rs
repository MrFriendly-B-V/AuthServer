use actix_web::{post, HttpResponse, HttpRequest};
use serde::Serialize;

#[derive(Serialize)]
struct TokenResponse {
    status:     i16,
    reason:     String,
    token:      Option<String>,
}

#[post("/token")]
pub async fn post_get_token(request: HttpRequest) -> HttpResponse {
    //Query string
    let qstring = qstring::QString::from(request.query_string());

    //Get ad verify that the user_id parameter is present
    let user_id_parameter = qstring.get("user_id");
    if user_id_parameter.is_none() {
        return HttpResponse::Ok().json(TokenResponse { status: 400, reason: "Missing parameter user_id".to_string(), token: None});
    }

    //Get and verify that the api_token parameter is present
    let api_token_parameter = qstring.get("api_token");
    if api_token_parameter.is_none() {
        return HttpResponse::Ok().json(TokenResponse { status: 400, reason: "Missing parameter api_token".to_string(), token: None});
    }

    //Validate the api_token
    let api_token_valid = crate::authentication::api_tokens::is_api_token_valid(api_token_parameter.unwrap().to_string());
    if !api_token_valid {
        return HttpResponse::Unauthorized().json("Invalid api_token");
    }

    //Get the token for this user_id
    let token = unsafe {
        crate::DATABASE.get_token(user_id_parameter.unwrap().to_string())
    };

    if token.is_none() {
        return HttpResponse::NotFound().json("Provided user_id was not found");
    }

    return HttpResponse::Ok().json(TokenResponse { status: 200, reason: "OK".to_string(), token: Some(token.unwrap())});
}