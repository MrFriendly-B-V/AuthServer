use mysql::{Pool, params, Params, Error, Row};
use mysql::prelude::Queryable;

pub struct Database {
    pool: Option<Pool>

}

pub const DATABASE: Database = Database {
    pool: None
};

//Unsafe because static mutable
unsafe fn init_db(database: &mut Database) {
    use std::env;
    let mysql_username = env::var("MYSQL_USERNAME").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    let mysql_database = env::var("MYSQL_DATABASE").unwrap();

    let mysql_url = format!("mysql://{username}:{password}@{host}/{database}",
                            username = mysql_username,
                            password = mysql_password,
                            host = mysql_host,
                            database = mysql_database);

    let created_pool = Pool::new(mysql_url).unwrap();

    database.pool = Some(created_pool)
}

impl Database {

    pub fn new() -> Database {
        Database {
            pool: None
        }
    }

    /**
    Get the return_uri from the database
    */
    pub fn get_return_uri_blocking(&mut self, state: String) -> Result<Vec<Option<String>>, Error> {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let result = conn.exec::<Row, &str, Params>("SELECT from_uri FROM states WHERE state = :state ", params! {
                        "state" => state
                    });

        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let rows = result.unwrap();
        let mut from_uris: Vec<Option<String>> = Vec::new();

        for row in rows {
            let from_uri = row.get::<String, &str>("from_uri");
            from_uris.push(from_uri);
        }
        Ok(from_uris)
    }

    /**
    Set the return_uri in the database
    */
    pub fn set_return_uri_blocking(&mut self, state: String, return_uri: String) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("INSERT INTO states (state, from_uri) VALUES (:state, :from_uri)", params! {
                        "state" => state,
                        "from_uri" => return_uri,
                    }).expect("Database error");
    }

    pub fn get_expiring_tokens(&mut self, expiring_time: i64) -> Vec<(String, String)> {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let sql_result = conn.exec::<Row, &str, Params>("SELECT user_id,refresh_token FROM grants WHERE expires_in <= :expires_in", params! {
                            "expires_in" => expiring_time
                        });

        let rows = sql_result.unwrap();
        let mut result: Vec<(String, String)> = Vec::new();

        for row in rows {
            result.push((
                row.get::<String, &str>("user_id").unwrap(),
                row.get::<String, &str>("refresh_token").unwrap()
            ));
        }

        return result;
    }

    pub fn insert_refreshed_grant(&mut self, user_id: String, access_token: String, expires_in: i64) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("UPDATE grants SET access_token = :access_token, expires_in = :expires_in WHERE user_id = :user_id", params! {
                            "access_token" => access_token,
                            "expires_in" => expires_in,
                            "user_id" => user_id
                        }).expect("Error when updating database");
    }

    /**
    Insert a grant into the database. Only use this function if you have a refresh_token, do not set this empty!
    */
    pub fn insert_grant(&mut self, user_id: String, access_token: String, id_token: String, scopes: String, token_type: String, refresh_token: String, expires_in: i64) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("INSERT INTO grants (user_id, access_token, id_token, scopes, token_type, refresh_token, expires_in) VALUES (:user_id, :access_token, :id_token, :scopes, :token_type, :refresh_token, :expires_in)", params! {
                        "user_id" => user_id,
                        "access_token" => access_token,
                        "id_token" => id_token,
                        "scopes" => scopes,
                        "token_type" => token_type,
                        "refresh_token" => refresh_token,
                        "expires_in" => expires_in
                    }).expect("Database error");
    }

    pub fn get_session_id(&mut self, user_id: String) -> (Option<String>, bool) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let response = conn.exec::<Row, &str, Params>("SELECT session_id FROM sessions WHERE user_id = :user_id", params! {
                            "user_id" => user_id
                            });

        if response.is_err() {
            return (None, false);
        }

        let response_unwrapped = response.unwrap();
        if response_unwrapped.clone().len() == 0 {
            return (None, false);
        }

        let first = response_unwrapped.first().unwrap().get::<String, &str>("session_id");
        return (first, true);
    }

    pub fn get_token(&mut self, user_id: String) -> Option<String> {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let response = conn.exec::<Row, &str, Params>("SELECT access_token FROM grants WHERE user_id = :user_id", params! {
                            "user_id" => user_id
                            });

        if response.is_err() {
            return None;
        }

        let response_unwrapped = response.unwrap();

        if response_unwrapped.clone().len() == 0 {
            return None;
        }

        let first_row = response_unwrapped.first().unwrap().clone();

        return Some(first_row.get::<String, &str>("access_token").unwrap());
    }

    pub fn set_session(&mut self, user_id: String, session_id: String, expires_at: i64) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("INSERT INTO sessions (user_id, session_id, expires_at) VALUES (:user_id, :session_id, :expires_at)", params!{
                        "user_id" => user_id,
                        "session_id" => session_id,
                        "expires_at" => expires_at
                        }).expect("A database error occurred");
    }

    pub fn delete_session(&mut self, session_id: String) {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("DELETE FROM sessions WHERE session_id = :session_id", params!{
                        "session_id" => session_id
                        }).expect("A database error occurred");
    }

    pub fn get_session(&mut self, session_id: String) -> Vec<(String, i64)> {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let sql_response = conn.exec::<Row, &str, Params>("SELECT user_id,expires_at FROM sessions WHERE session_id = :session_id", params!{
                        "session_id" => session_id
                        }).expect("A database error occurred");

        let mut result: Vec<(String, i64)> = Vec::new();

        for row in sql_response {
            result.push((
                row.get::<String, &str>("user_id").unwrap(),
                row.get::<i64, &str>("expires_at").unwrap()
            ));
        };

        return result;
    }

    pub fn has_api_token(&mut self, api_token: String) -> bool {
        if self.pool.is_none() {
            unsafe {
                init_db(self);
            }
        }

        let pool = self.pool.as_ref().unwrap();
        let mut conn = pool.get_conn().unwrap();

        let sql_response = conn.exec::<Row, &str, Params>("SELECT api_token FROM api_tokens WHERE api_token = :api_token", params! {
                            "api_token" => api_token
                             });

        if sql_response.is_err() {
            return false;
        }

        let row = sql_response.unwrap();

        return row.len() > 0;
    }
}