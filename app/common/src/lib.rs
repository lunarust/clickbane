use serde::{Deserialize, Serialize};
//use serde_json;
pub mod configuration;
pub mod jasper;

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
