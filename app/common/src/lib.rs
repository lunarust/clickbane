use serde::{Deserialize, Serialize};
//use serde_json;
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
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Customer {
    pub customerNumber: i32,
    pub customerName: String,
    pub contactLastName: String,
    pub contactFirstName: String,
    pub email: String,
}
impl Customer {
    pub fn default() -> Customer {
        Customer {
            customerNumber: 0,
            customerName: "".to_string(),
            contactLastName: "".to_string(),
            contactFirstName: "".to_string(),
            email: "".to_string(),
        }
    }
    pub fn of (c: Customer) -> Customer {
        Customer {
            customerNumber: c.customerNumber,
            customerName: c.customerName,
            contactLastName: c.contactLastName,
            contactFirstName: c.contactFirstName,
            email: c.email,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct JS_Scheduled_Job {
    pub id: i32,
    pub label: String,
    pub job_name: String,
    pub description: String,
    pub trigger_state: String,
    pub trigger_type: String,
    pub next_fire: String,
    pub prev_fire: String,
    pub base_output_name: String,
    pub address: String,
    pub occurrence_date: String,
    pub err_message: String,
    pub failed: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomerJobRequest {
    pub customer_name: String,
}
impl CustomerJobRequest {

    pub fn of (cjr: CustomerJobRequest) -> CustomerJobRequest {
        CustomerJobRequest {
            customer_name: cjr.customer_name,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CustomerJobSchedule {
    pub customer: Customer,
    pub ftpHost: String,
    pub ftpPassword: String,
    pub ftpUser: String,

}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JS_Report {
    pub label: String,
    pub description: String,
    pub uri: String,
    pub param: Vec<InputParam>,
    pub default: bool,
    pub frequency: Vec<u32>,
}

impl JS_Report {
    pub fn default() -> JS_Report {
        JS_Report {
            label: "".to_string(),
            description: "".to_string(),
            uri: "".to_string(),
            param: vec![],
            default: false,
            frequency: [0,0,0].to_vec(),
        }
    }
}

#[warn(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InputParam {
    pub id: String,
    pub label: String,
    pub uri: String,
    pub dataType: String,
    pub mapped: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InputMapping {
    pub id: i32,
    pub input_id: String,
    pub configuration_id: i32,
    pub configuration_table: String,
    pub configuration_column: String,
}
