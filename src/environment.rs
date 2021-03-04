pub struct Environment {
    pub port:                   String,
    pub bind_address:           String,
    pub google_client_secret:   String,
    pub google_client_id:       String,
    pub host:                   String,
    pub mysql_host:             String,
    pub mysql_database:         String,
    pub mysql_username:         String,
    pub mysql_password:         String,
}

impl Environment {

    /**
    Check whether all required environmental variables are set

    @returns Returns true if all environmental variables are present. False if any are missing
    */
    pub fn check_environment() -> bool {
        let env = Environment::get_environment();

        return !env.is_none();
    }

    /**
    Get an instance of Environment. This will be None if any environmental variables are missing.
    */
    pub fn get_environment() -> Option<Environment> {
        use std::env;

        let port = env::var("PORT");
        if port.is_err() {
            eprintln!("Environmental variable 'PORT' not set.");
            return None;
        }

        let bind_address = env::var("BIND_ADDRESS");
        if bind_address.is_err() {
            eprintln!("Environmental variable 'BIND_ADDRESS' not set.");
            return None;
        }

        let google_client_secret = env::var("GOOGLE_CLIENT_SECRET");
        if google_client_secret.is_err() {
            eprintln!("Environmental variable 'GOOGLE_CLIENT_SECRET' not set.");
            return None;
        }

        let google_client_id = env::var("GOOGLE_CLIENT_ID");
        if google_client_id.is_err() {
            eprintln!("Environmental variable 'GOOGLE_CLIENT_ID' not set.");
            return None;
        }

        let host = env::var("HOST");
        if host.is_err() {
            eprintln!("Environmental variable 'HOST' not set.");
            return None;
        }

        let mysql_host = env::var("MYSQL_HOST");
        if mysql_host.is_err() {
            eprintln!("Environmental variable 'MYSQL_HOST' not set.");
            return None;
        }

        let mysql_database = env::var("MYSQL_DATABASE");
        if mysql_database.is_err() {
            eprintln!("Environmental variable 'MYSQL_DATABASE' not set.");
            return None;
        }

        let mysql_username = env::var("MYSQL_USERNAME");
        if mysql_username.is_err() {
            eprintln!("Environmental variable 'MYSQL_USERNAME' not set.");
            return None;
        }

        let mysql_password = env::var("MYSQL_PASSWORD");
        if mysql_password.is_err() {
            eprintln!("Environmental variable 'MYSQL_PASSWORD' not set.");
            return None;
        }

        Some(Environment {
            port: port.unwrap(),
            bind_address: bind_address.unwrap(),
            google_client_secret: google_client_secret.unwrap(),
            google_client_id: google_client_id.unwrap(),
            host: host.unwrap(),
            mysql_host: mysql_host.unwrap(),
            mysql_database: mysql_database.unwrap(),
            mysql_username: mysql_username.unwrap(),
            mysql_password: mysql_password.unwrap(),
        })
    }
}