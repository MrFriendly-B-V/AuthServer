use actix_web::{get, web, HttpResponse, HttpRequest};
use tera::Context;
use std::collections::HashMap;
use rand::Rng;

const GOOGLE_OAUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

#[get("/oauth/login")]
pub async fn get_oauth_login(appdata: web::Data<crate::appdata::AppData>, request: HttpRequest) -> HttpResponse {
    //Get the query string
    let qstring = qstring::QString::from(request.query_string());

    //Get the 'scopes' parameter and check if it is present
    let scopes_param = qstring.get("scopes");
    if scopes_param.is_none() {
        return HttpResponse::BadRequest().json("Missing parameter 'scopes'");
    }

    //Get the 'from' parameter and check if it is present
    let from_param = qstring.get("from");
    if from_param.is_none() {
        return HttpResponse::BadRequest().json("Missing parameter 'from'");
    }
    let from = from_param.unwrap();

    //Split the 'scopes' param on ',' and merge again with space delimitation (Google API requirement)
    let split_scopes_vec: Vec<&str> = scopes_param.unwrap().split(",").collect();
    let scopes_space_delim: String = split_scopes_vec.join(" ");

    //Generate a state token
    let state: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect();

    &appdata.database.lock().unwrap().set_return_uri_blocking(state.clone(), from.to_string());

    let env = &appdata.environment;
    let google_client_id = env.get_google_client_id().clone();
    let host = env.get_host().clone();

    let google_redirect_uri = format!("https://{}/oauth/grant", host);

    //Build the redirect query into a HashMap first
    let parameters: HashMap<&str, &str> = [
        ("client_id", google_client_id.as_str()),
        ("response_type", "code"),
        ("scope", scopes_space_delim.as_str()),
        ("access_type", "offline"),
        ("state", state.as_str()),
        ("include_granted_scopes", "true"),
        ("prompt", "select_account"),
        ("redirect_uri", google_redirect_uri.as_str())
    ].iter().cloned().collect();

    //Turn the HashMap into a valid URL
    let url_query_params = crate::utils::hashmap_to_url(parameters);
    let final_url = format!("{}{}", GOOGLE_OAUTH_URL, url_query_params);

    let mut ctx = Context::new();
    ctx.insert("redirect_url", final_url.as_str());

    let rendered = appdata.tera.render("login.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}