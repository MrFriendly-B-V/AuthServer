use actix_web::{get, web, HttpResponse, HttpRequest};
use tera::Context;
use crate::environment::Environment;

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
    let return_uri_vec = unsafe {
        crate::DATABASE.get_return_uri_blocking(state_param.unwrap().to_string()).unwrap()
    };

    //Likely database error
    if return_uri_vec.first().is_none() {
        return HttpResponse::InternalServerError().json("Unable to fetch information for supplied parameter 'state'. Database error.");
    }

    //Get the value of the return_uri, if its 'none', 404
    let return_uri = return_uri_vec.first().unwrap();
    if return_uri.is_none() {
        return HttpResponse::NotFound().json("Found no reference to provided 'state' parameter.");
    }

    //Access code parameter, we can trust Google that this is never missing
    let access_code = qstring.get("code").unwrap();

    //Spin up a thread to exchange the code for an access and ID token
    let env = Environment::get_environment().unwrap();

    let client_id = env.google_client_id;
    let client_secret = env.google_client_secret;
    let host = env.host;
    let google_endpoint = discovery_document.token_endpoint.clone();
    let jwks_keys = discovery_document.jwks_keys.clone();

    let user_id = crate::authentication::exchange_code_for_token::exchange(access_code.to_string(), client_id, client_secret, host, google_endpoint, jwks_keys);

    if user_id.is_err() {
        return HttpResponse::BadRequest().json(user_id.err());
    }

    //Get a session_id
    let has_session = unsafe {
        crate::DATABASE.get_session_id(user_id.clone().unwrap())
    };

    let session_id =
        if has_session.1 {
            has_session.0.unwrap()
        } else {
            crate::authentication::session_controller::generate_session(user_id.unwrap())
    };

    let return_uri_cloned = return_uri.as_ref().unwrap();
    let final_uri = if return_uri_cloned.clone().contains("?") {
        //Return URI already has query parameters, we can add to this
        format!("{}&session_id={}", return_uri.clone().unwrap(), session_id.clone())
    } else {
        //Return URI contains no query parameters, we need to add our own
        format!("{}?session_id={}", return_uri.clone().unwrap(), session_id.clone())
    };

    //Apply the return_uri to the template
    let mut ctx = Context::new();
    ctx.insert("redirect_url", &final_uri);
    let rendered = appdata.tera.render("login.html", &ctx).unwrap();

    //Return a 200 status code
    HttpResponse::Ok().body(rendered)
}