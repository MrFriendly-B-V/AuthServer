use crate::environment::Environment;

use mysql::{Pool, params, Params, Error};
use mysql::prelude::Queryable;

pub struct Database {
    pool: Pool

}

impl Database {
    pub fn new(environment: Environment) -> Database {
        let mysql_username = environment.get_mysql_username().clone();
        let mysql_password = environment.get_mysql_password().clone();
        let mysql_database = environment.get_mysql_database().clone();
        let mysql_host = environment.get_mysql_host().clone();

        let mysql_url = format!("mysql://{username}:{password}@{host}/{database}",
                                username = mysql_username,
                                password = mysql_password,
                                host = mysql_host,
                                database = mysql_database);

        let pool = Pool::new(mysql_url).unwrap();

        Database {
            pool
        }
    }

    pub fn get_return_uri_blocking(&self, state: String) -> Result<Vec<String>, Error> {
        let mut conn = self.pool.get_conn().unwrap();

        let result = conn.exec::<String, &str, Params>("SELECT from_uri FROM states WHERE state = :state ", params! {
                        "state" => state
                    });

        if result.is_err() {
            return Err(result.err().unwrap());
        }

        Ok(result.unwrap())
    }

    pub fn set_return_uri_blocking(&self, state: String, return_uri: String) {
        let mut conn = self.pool.get_conn().unwrap();

        conn.exec::<usize, &str, Params>("INSERT INTO states (state, from_uri) VALUES (:state, :from_uri)", params! {
                        "state" => state,
                        "from_uri" => return_uri,
                    }).expect("Database error");
    }
}