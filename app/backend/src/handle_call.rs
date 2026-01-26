use std::time::Duration;
use std::fs;
use std::path::Path;
use warp::{http::StatusCode, reply::json, Reply};
use reqwest;
use crate::mysql_db;
use crate::sqlite_db;
use crate::generic;
use crate::jasper_import;

use common::*;
use common::{configuration::ConfigurationJs, configuration::ConfigurationJsRequest,configuration::ConfigurationBusiness};
use common::jasper::{JS_Report,JS_Scheduled_Job,CustomerJobSchedule,CustomerJobRequest,InputMapping};


// BUSINESS DB CONFIGURATION HANDLERS
pub async fn get_configuration_call() -> Result<impl Reply, warp::Rejection> {
    let cfg = sqlite_db::query_configuration()
        .await
        .map_err(|_e| warp::reject::not_found())?;

    Ok(warp::reply::json(&cfg))
}
pub async fn set_configuration_call(body: ConfigurationBusiness) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::insert_configuration(body)
    .await;

    Ok(StatusCode::OK)
}
pub async fn udp_configuration_call(body: ConfigurationBusiness) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::update_configuration(body)
    .await;

    Ok(StatusCode::OK)
}
pub async fn delete_configuration_call(i: i32) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::delete_configuration(i)
    .await;

    Ok(StatusCode::OK)
}

// JASPER SERVER CONFIGURATION HANDLERS
pub async fn get_js_configuration_call() -> Result<impl Reply, warp::Rejection> {
    let cfg_js = sqlite_db::query_js_configuration()
        .await
        .map_err(|_e| warp::reject::not_found())?;

    Ok(warp::reply::json(&cfg_js))
}
pub async fn set_js_configuration_call(body: ConfigurationJsRequest) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::insert_js_configuration(body)
    .await;

    Ok(StatusCode::OK)
}
pub async fn udp_js_configuration_call(body: ConfigurationJs) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::update_js_configuration(body)
    .await;

    Ok(StatusCode::OK)
}
pub async fn delete_js_configuration_call(i: i32) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::delete_js_configuration(i)
    .await;

    Ok(StatusCode::OK)
}

pub async fn get_all_clients_call() ->  Result<impl Reply, warp::Rejection> {
    generic::logthis(format!("Entering handle call > get all clients").as_str(), "INFO");

    let custs = mysql_db::query_customers()
        .await;

    Ok(json::<Vec<_>>(
        &custs.unwrap(),
    ))
}
pub async fn schedule_jasper_jobs_for_customer(body: CustomerJobSchedule) -> Result<impl Reply, warp::Rejection> {
    // Get path with template for jobs.
    let fq: [&str; 3] = [
    "<simpleTrigger><misfireInstruction>0</misfireInstruction><startType>1</startType><timezone>UTC</timezone><version>0</version><occurrenceCount>-1</occurrenceCount><recurrenceInterval>1</recurrenceInterval><recurrenceIntervalUnit>DAY</recurrenceIntervalUnit></simpleTrigger>",
    "<simpleTrigger><misfireInstruction>0</misfireInstruction><startType>1</startType><timezone>UTC</timezone><version>0</version><occurrenceCount>-1</occurrenceCount><recurrenceInterval>7</recurrenceInterval><recurrenceIntervalUnit>DAY</recurrenceIntervalUnit></simpleTrigger>",
    "<calendarTrigger><misfireInstruction>0</misfireInstruction><startType>1</startType><timezone>UTC</timezone><version>0</version><daysType>MONTH</daysType><hours>10</hours><minutes>0</minutes><monthDays>1</monthDays><months><month>1</month><month>10</month><month>11</month><month>12</month><month>2</month><month>3</month><month>4</month><month>5</month><month>6</month><month>7</month><month>8</month><month>9</month></months></calendarTrigger>"
    ];
    let fq_label: [&str; 3] = [
        "Daily", "Weekly", "Monthly"
    ];
    // Get Jasper Server Configuration
    let jsconf: ConfigurationJs = sqlite_db::query_js_configuration().await.unwrap();

    let url = format!("{}/jobs/", jsconf.js_url);

    // Get Default Jobs
    let jsreports = sqlite_db::query_default_job().await;
    // Schedule themT
    let mypath = format!("{}/jobtemplate/", generic::get_current_working_dir());
    for js in jsreports.unwrap() {
        let path = Path::new(mypath.as_str());

        for model_file in fs::read_dir(path).expect("Unable to list directory") {
            let model_file = model_file.expect("Unable to get file");
            let mut contents = fs::read_to_string(model_file.path()).expect("Something went wrong while reading template file.");

            contents = contents.replace("[BASEOUTPUTFILENAME]", &js.label);
            contents = contents.replace("[REPORTURI]", &js.uri);
            contents = contents.replace("CUSTOMERNAME", &body.customer.customerName.replace("&", "&#38;"));
            contents = contents.replace("[RECIPIENTS]", &body.customer.email);
            contents = contents.replace("[SFTPUSERNAME]", &body.ftpUser);
            contents = contents.replace("[SFTPPASSWORD]", &body.ftpPassword);
            contents = contents.replace("[SFTPHOST]", &body.ftpHost);

            for p in &js.param {
                let map: Vec<InputMapping> = sqlite_db::get_report_resource_mapping(p.mapped.unwrap()).await.unwrap();
                for m in map {
                    contents = contents.replace(format!("[{}]", m.input_id).as_str(), &body.customer.customerNumber.to_string());
            }}

            for i in 0..3 {
                if js.frequency[i] == 1 {
                    let mut contents_sch = contents.replace("[REPORTNAME]", format!("{} - {}", fq_label[i], &js.label).as_str());
                    contents_sch = contents_sch.replace("[SCHEDULEDTRIGGER]", fq[i]);
                    let _result = put_jasper_rest_api(url.clone(), jsconf.js_secret.clone(), contents_sch.clone()).await;

                    //println!("{:?}", contents_sch);
                    //println!("{:?}", contents_sch);
                }
            }
        }
    }
    Ok(StatusCode::OK)
}
pub async fn get_jasper_jobs_per_customer(body: CustomerJobRequest) -> Result<impl Reply, warp::Rejection> {
    let jobs = mysql_db::query_js_jobs_per_customer(body.customer_name)
        .await;
    Ok(json::<Vec<_>>(
        &jobs.unwrap(),
    ))
}
pub async fn delete_jasper_job(job_id: i32) -> Result<impl Reply, warp::Rejection> {
    generic::logthis(format!("JASPER, Delete {}", job_id).as_str(), "INFO");

    // Get Jasper Server Configuration
    let jsconf: ConfigurationJs = sqlite_db::query_js_configuration().await.unwrap();

    let url = format!("{}/jobs/{}", jsconf.js_url, job_id.to_string());
    let _ = delete_jasper_rest_api(url, jsconf.js_secret).await;

    Ok(StatusCode::OK)
}
pub async fn delete_jasper_job_all(customer: Customer) -> Result<impl Reply, warp::Rejection> {
    generic::logthis(format!("JASPER, Delete all {}", customer.customerName).as_str(), "INFO");

    let js: Vec<JS_Scheduled_Job> = mysql_db::query_js_jobs_per_customer(customer.customerName).await.unwrap();
    for j in js {
        let _ = delete_jasper_job(j.id).await;
    }

    Ok(StatusCode::OK)
}
pub async fn replay_jasper_job(job_name: String) -> Result<impl Reply, warp::Rejection> {
    generic::logthis(format!("JASPER, Replay {}", job_name).as_str(), "INFO");

    // Get Jasper Server Configuration
    let jsconf: ConfigurationJs = sqlite_db::query_js_configuration().await.unwrap();

    let url = format!("{}/jobs/restart", jsconf.js_url);
    let body_xml = format!("<jobIdList><jobId>{}</jobId></jobIdList>", job_name.replace("job_", ""));
    //println!("{:?} {:?}", url, body_xml);

    let _ = post_jasper_rest_api(url, jsconf.js_secret, body_xml).await;

    Ok(StatusCode::OK)
}
pub async fn fetch_all_jasper_reports() -> Result<impl Reply, warp::Rejection> {
    generic::logthis(format!("JASPER, Fetch all reports").as_str(), "INFO");
    let rep: Vec<JS_Report> = jasper_import::fetch_resources().await;
    //Ok(StatusCode::OK)
    Ok(json::<Vec<_>>(
        &rep,
    ))
}
pub async fn set_report_default(jsr: JS_Report) -> Result<impl Reply, warp::Rejection> {
    let _ = sqlite_db::insert_report(jsr)
    .await;

    Ok(StatusCode::OK)
}


async fn delete_jasper_rest_api(url: String, secret: String) ->  Result<impl Reply, warp::Rejection> {
    let client = reqwest::Client::new();
    let _ = client
        .delete(&url)
        .header("Authorization", "Basic ".to_owned() + &secret)
        .header("Content-Type", "application/xml")
        .timeout(Duration::from_secs(15))
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

   Ok(StatusCode::OK)
}

async fn post_jasper_rest_api(url: String, secret: String, contents: String) ->  Result<impl Reply, warp::Rejection> {
    let client = reqwest::Client::new();
    let _ = client
        .post(&url)
        .header("Authorization", "Basic ".to_owned() + &secret)
        .header("Content-Type", "application/xml")
        .timeout(Duration::from_secs(15))
        .body(contents)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

   Ok(StatusCode::OK)
}
async fn put_jasper_rest_api(url: String, secret: String, contents: String) ->  Result<impl Reply, warp::Rejection> {
    let client = reqwest::Client::new();
    let _doge = client
        .put(&url)
        .header("Authorization", "Basic ".to_owned() + &secret)
        .header("Content-Type", "application/xml")
        .timeout(Duration::from_secs(15))
        .body(contents)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

   Ok(StatusCode::OK)
}
