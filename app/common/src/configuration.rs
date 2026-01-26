use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConfigurationJsRequest {
    pub js_url: String,
    pub js_secret: String,
    pub js_db_host: String,
    pub js_db_port: i32,
    pub js_db_user: String,
    pub js_db_password: String,
}
impl ConfigurationJsRequest {
    pub fn of (c: ConfigurationJsRequest) -> ConfigurationJsRequest {
        ConfigurationJsRequest {
            js_url: c.js_url,
            js_secret: c.js_secret,
            js_db_host: c.js_db_host,
            js_db_port: c.js_db_port,
            js_db_user: c.js_db_user,
            js_db_password: c.js_db_password,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigurationJs {
    pub js_id: i64,
    pub js_url: String,
    pub js_secret: String,
    pub js_db_host: String,
    pub js_db_port: i32,
    pub js_db_user: String,
    pub js_db_password: String,
}
impl ConfigurationJs {
    pub fn default() -> ConfigurationJs {
        ConfigurationJs {
            js_id: 0,
            js_url: "".to_string(),
            js_secret: "".to_string(),
            js_db_host: "".to_string(),
            js_db_port: 0,
            js_db_user: "".to_string(),
            js_db_password: "".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigurationBusiness {
    pub business_id: i64,
    pub business_name: String,
    pub business_db_host: String,
    pub business_db_port: i32,
    pub business_db_user: String,
    pub business_db_password: String,
}
impl ConfigurationBusiness {
    pub fn default() -> ConfigurationBusiness {
        ConfigurationBusiness {
            business_id: 0,
            business_name: "".to_string(),
            business_db_host: "".to_string(),
            business_db_port: 0,
            business_db_user: "".to_string(),
            business_db_password: "".to_string(),
        }
    }
}
