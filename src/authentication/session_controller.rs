/**
Get a user_id for a session_id. Will not check the expires_at of this session!
*/
#[allow(dead_code)]
pub fn get_user_for_session(session_id: String) -> Result<String, bool> {
    let users: Vec<(String, i64)> = unsafe {
        crate::DATABASE.get_session(session_id)
    };

    //We take the first
    if users.len() == 0 {
        return Err(false);
    }

    let first_result = users.first().unwrap().clone();
    Ok(first_result.0)
}

/**
Generate a session_id for a user_id, and insert it into the database
*/
pub fn generate_session(user_id: String) -> String {
    use rand::Rng;
    let session_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(255).map(char::from).collect();

    //Calculate epoch date at which the session expires, 30 days
    let expires_at = chrono::Utc::now() + chrono::Duration::days(30);

    //Insert the data into the database
    unsafe {
        crate::DATABASE.set_session(user_id, session_id.clone(), expires_at.timestamp());
    }

    //Return the session id
    session_id
}

pub fn validate_session(session_id: String) -> Result<bool, bool> {
    //Get the expires_at for this session_id
    let sql_result = unsafe {
        crate::DATABASE.get_session(session_id.clone())
    };

    if sql_result.len() == 0 {
        return Err(false);
    }

    //Take the first result
    let result = sql_result.first().unwrap().clone();
    let expires_at = result.1;

    //Check if the current epoch is more than the expires_at,
    //if this is the case then the session is no longer value,
    //and we should remove it from the database
    if chrono::Utc::now().timestamp() >= expires_at {
        unsafe {
            crate::DATABASE.delete_session(session_id);
        }

        return Ok(false);
    }

    Ok(true)
}