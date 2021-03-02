use actix_web::{get, web, HttpResponse, HttpRequest};
use tera::Context;
use crate::authentication::exchange_code_for_token::ExchangeCodeForToken;

#[get("/oauth/grant")]
pub async fn get_oauth_grant(appdata: web::Data<crate::appdata::AppData>, request: HttpRequest) -> HttpResponse {
    let discovery_document = &appdata.discovery_document.lock().unwrap();
    let qstring = qstring::QString::from(request.query_string());

    //Get the 'state' parameter, if it doesn't exist throw a 400 error
    let state_param = qstring.get("state");
    if state_param.is_none() {
        return HttpResponse::BadRequest().json("Missing parameter 'state'");
    }

    //Get the return_uri for the value of the 'state' parameter
    let return_uri_vec = &appdata.database.lock().unwrap().get_return_uri_blocking(state_param.unwrap().to_string());

    //Likely database error
    if return_uri_vec.is_err() {
        eprintln!("{:?}", return_uri_vec.as_ref().err());
        return HttpResponse::InternalServerError().json("Unable to fetch information for supplied parameter 'state'. Database error.");
    }

    //Get the value of the return_uri, if its 'none', 404
    let return_uri = return_uri_vec.as_ref().unwrap().first();
    if return_uri.is_none() {
        return HttpResponse::NotFound().json("Found no reference to provided 'state' parameter.");
    }

    //Apply the return_uri to the template
    let mut ctx = Context::new();
    ctx.insert("redirect_url", return_uri.unwrap());
    let rendered = appdata.tera.render("login.html", &ctx).unwrap();

    //Access code parameter, we can trust Google that this is never missing
    let access_code = qstring.get("code").unwrap();

    //Spin up a thread to exchange the code for an access and ID token
    let client_id = appdata.environment.get_google_client_id().clone();
    let client_secret = appdata.environment.get_google_client_secret().clone();
    let host = appdata.environment.get_host().clone();
    let google_endpoint = discovery_document.token_endpoint.clone();

    let mut exchange_code_for_token = ExchangeCodeForToken::new();
    exchange_code_for_token.exchange(access_code.to_string(), client_id, client_secret, host, google_endpoint);

    println!("Exchanging token!");

    HttpResponse::Ok().body(rendered)
}