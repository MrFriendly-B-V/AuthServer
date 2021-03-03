#[allow(dead_code)]
pub struct Environment {
    port:                   String,
    bind_address:           String,
    google_client_secret:   String,
    google_client_id:       String,
    host:                   String,
    mysql_host:             String,
    mysql_database:         String,
    mysql_username:         String,
    mysql_password:         String,
    api_tokens:             Vec<String>
}

impl Environment {

    pub fn new(port: String,
               bind_address: String,
               google_client_secret: String,
               google_client_id: String,
               host: String,
               mysql_host: String,
               mysql_database: String,
               mysql_username: String,
               mysql_password: String,
               api_tokens: Vec<String>) -> Environment {

        Environment {
            port,
            bind_address,
            google_client_secret,
            google_client_id,
            host,
            mysql_host,
            mysql_database,
            mysql_username,
            mysql_password,
            api_tokens
        }
    }

    pub fn get_port(&self) -> &String {
        &self.port
    }

    pub fn get_bind_address(&self) -> &String {
        &self.bind_address
    }

    pub fn get_google_client_secret(&self) -> &String {
        &self.google_client_secret
    }

    pub fn get_google_client_id(&self) -> &String {
        &self.google_client_id
    }

    pub fn get_host(&self) -> &String {
        &self.host
    }

    #[allow(dead_code)]
    pub fn get_mysql_host(&self) -> &String {
        &self.mysql_host
    }

    #[allow(dead_code)]
    pub fn get_mysql_database(&self) -> &String {
        &self.mysql_database
    }

    #[allow(dead_code)]
    pub fn get_mysql_username(&self) -> &String {
        &self.mysql_username
    }

    #[allow(dead_code)]
    pub fn get_mysql_password(&self) -> &String {
        &self.mysql_password
    }

    #[allow(dead_code)]
    //TODO this has no uses, yet
    pub fn get_api_tokens(&self) -> &Vec<String> {
        &self.api_tokens
    }
}