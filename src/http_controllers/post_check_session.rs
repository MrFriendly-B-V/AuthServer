use actix_web::{post, HttpResponse, HttpRequest};
use serde::Serialize;

#[derive(Serialize)]
struct SessionCheckResponse {
    pub status:     i16,
    pub reason:     String
}

#[post("/session/check")]
pub async fn post_check_session(request: HttpRequest) -> HttpResponse {
    //Get the query string
    let qstring = qstring::QString::from(request.query_string());

    //Check the session_id parameter
    let session_id_param = qstring.get("session_id");
    if session_id_param.is_none() {
        return HttpResponse::BadRequest().json("Missing session_id parameter");
    }

    //Validate the session_id
    let session_validation = crate::authentication::session_controller::validate_session(session_id_param.unwrap().to_string());
    if session_validation.is_err() {
        let response = SessionCheckResponse { status: 401, reason: "Unknown session_id".to_string() };
        return HttpResponse::Ok().json(&response);
    }
    
    let session_validation_state = session_validation.unwrap();
    let response =
        if session_validation_state {
            SessionCheckResponse { status: 200, reason: "OK".to_string()}
        } else {
            SessionCheckResponse { status: 401, reason: "Session expired".to_string() }
        };

    return HttpResponse::Ok().json(&response);
}