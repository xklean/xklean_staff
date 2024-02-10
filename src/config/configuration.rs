use std::collections::HashMap;
use config::{Config, File};

pub struct Configuration {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_schema: String,

    pub service_host: String,
    pub service_port: String,
}

impl Configuration {
    pub fn init_config() -> Configuration {
        let setting = Config::builder().
            add_source(File::with_name("config.yaml")).build().unwrap();

        let map = setting.
            try_deserialize
                ::<HashMap<String, String>>().unwrap();

        return Configuration {
            db_host: map["database_host"].to_string(),
            db_user: map["database_user"].to_string(),
            db_password: map["database_password"].to_string(),
            db_name: map["database_name"].to_string(),
            db_schema: map["database_schema"].to_string(),
            service_host: map["service_host"].to_string(),
            service_port: map["service_port"].to_string(),
        };
    }

    pub fn get_connection_url(&self) -> String {
        return format!("postgres://{}:{}@{}/{}?currentSchema={}",
                       self.db_user,
                       self.db_password,
                       self.db_host,
                       self.db_name, self.db_schema);
    }

    pub fn get_service_address(&self) -> String {
        format!("{}:{}", self.service_host, self.service_port)
    }
}