use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use reqwest;
use crate::sqlite_db;
use common::{configuration::ConfigurationJs};
use common::jasper::{InputParam,JS_Report};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Resources {
    resourceLookup : Vec<ResourceLookup>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct ResourceLookup {
    creationDate: String,
    description: String,
    label: String,
    permissionMask: i32,
    updateDate: String,
    uri: String,
    version: i32,
    resourceType: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct InputControls {
    inputControl: Vec<InputControl>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct InputControl {
    dataType: Vec<DataType>,
    id: String,
    label: String,
    mandatory: bool,
    readOnly: bool,
    state: Vec<State>,
    r#type: String,
    uri: String,
    visible: bool,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct DataType {
    strictMax: bool,
    strictMin: bool,
    r#type: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct State {
    id: String,
    uri: String,
    r#value: String,
}


pub async fn sync_resources() -> Vec<JS_Report> {

    // Get Jasper Server Configuration
    let jsconf: ConfigurationJs = sqlite_db::query_js_configuration().await.unwrap();

    let url = format!("{}/resources?limit=0&type=reportUnit&recursive=true", jsconf.js_url);

    let rep = call_api(url, &jsconf.js_secret).await;

    let res: Resources = from_str(&rep).unwrap();
    let mut output: Vec<JS_Report> = vec![];
    for r in res.resourceLookup {
        let param_url = format!("{}/reports{}/inputControls", jsconf.js_url, r.uri);
        let pr: Vec<InputParam> = parse_param(
            call_api(param_url, &jsconf.js_secret)
            .await
        ).await;

        let _strpr = serde_json::to_string(&pr).unwrap();

        let tmp_report = JS_Report{
                label: r.label,
                description: r.description,
                uri: r.uri,
                param: pr,
                default: false,
                frequency: [0,0,0].to_vec(),
            };


        let my_report: JS_Report = sqlite_db::get_report_scheduled(tmp_report).await.unwrap();
        //println!(">> {:?}", my_report);
        let _ = sqlite_db::insert_report(my_report.clone()).await;
        output.push(my_report)
    }
    output
}

async fn call_api(url: String, sec: &str) -> String {
    let client = reqwest::Client::new();
    let doge = client
        .get(url)
        .header("Content-Type", "application/xml")
        .header("Authorization", "Basic ".to_owned() + sec)
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");
    doge
}
async fn parse_param(call_res: String, ) -> Vec<InputParam> {
    let mut output: Vec<InputParam> = vec![];

    if call_res != "" {
    let res: InputControls = from_str(&call_res).unwrap();
    for r in res.inputControl {
        let mapped_id = sqlite_db::check_report_resource_mapped(r.id.clone()).await.unwrap();

        output.push(
            InputParam {
                id: r.id,
                label: r.label,
                uri: r.uri,
                dataType: r.r#type,
                mapped: Some(mapped_id),
            }
        )
    }
    }
    output
}
