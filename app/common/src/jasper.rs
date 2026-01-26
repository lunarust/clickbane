use serde::{Deserialize, Serialize};
use crate::Customer;

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
