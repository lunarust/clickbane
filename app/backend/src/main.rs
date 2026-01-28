extern crate lazy_static;
use warp::{http::Method, Filter};
use std::fs;

mod handle_call;
mod mysql_db;
mod sqlite_db;
mod generic;
mod jasper_import;

#[tokio::main]
async fn main() {

    println!("Good day ▼(´ᴥ`)▼ ");

    // Check if we have a SQLite DB
    if exists(format!("{}/configuration.db", generic::get_current_working_dir())) {
        // todo load conf
        println!("Loading configuration to implement");
    } else {
        sqlite_db::create_db;
    }


    let all_clients = warp::path("clients");
    let all_clients_routes = all_clients
        .and(warp::get())
        .and_then(handle_call::get_all_clients_call);


    let jasper_jobs_per_customer = warp::path!("jasper");
    let jasper_schedule_all_for_customer = warp::path!("jasper" / "all");
    let jasper_replay_job = warp::path!("jasper" / "replay" / String);
    let jasper_remove_job = warp::path!("jasper" / i32);
    let jasper_fetch_all_report = warp::path!("jasper" / "fetch");
    let jasper_sync_report = warp::path!("jasper" / "sync");
    let jasper_set_report_default = warp::path!("jasper" / "default");
    let jasper_change_report_frequency = warp::path!("jasper" / "frequency");
    let jasper_delete_all_reports = warp::path!("jasper" / "remove");

    let jasper_routes = jasper_replay_job
        .and(warp::get())
        .and_then(handle_call::replay_jasper_job)
        .or(jasper_jobs_per_customer
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::get_jasper_jobs_per_customer))
        .or(jasper_schedule_all_for_customer
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::schedule_jasper_jobs_for_customer))
        .or(jasper_remove_job
            .and(warp::delete())
            .and_then(handle_call::delete_jasper_job))
        .or(jasper_delete_all_reports
            .and(warp::delete())
            .and(warp::body::json())
            .and_then(handle_call::delete_jasper_job_all))
        .or(jasper_fetch_all_report
            .and(warp::get())
            .and_then(handle_call::fetch_all_jasper_reports))
        .or(jasper_sync_report
            .and(warp::get())
            .and_then(handle_call::sync_jasper_reports))
        .or(jasper_set_report_default
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::set_report_default))
        .or(jasper_change_report_frequency
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::set_report_freq)
        );


    let conf_js = warp::path!("configuration" / "js");
    let conf_js_del = warp::path!("configuration" / "js" / i32);
    let conf_js_upd = warp::path!("configuration" / "js" / "update");

    let config_js_routes = conf_js
        .and(warp::get())
        .and_then(handle_call::get_js_configuration_call)
        .or(conf_js
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::set_js_configuration_call))
        .or(conf_js_upd
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::udp_js_configuration_call))
        .or(conf_js_del
            .and(warp::delete())
            .and_then(handle_call::delete_js_configuration_call));

    let conf = warp::path!("configuration");
    let conf_del = warp::path!("configuration" / i32);
    let conf_upd = warp::path!("configuration" / "update");

    let config_routes = conf
        .and(warp::get())
        .and_then(handle_call::get_configuration_call)
        .or(conf
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::set_configuration_call))
        .or(conf_upd
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handle_call::udp_configuration_call))
        .or(conf_del
            .and(warp::delete())
            .and_then(handle_call::delete_configuration_call));



    let routes = all_clients_routes
        .or(jasper_routes)
        .or(config_js_routes)
        .or(config_routes)
        .with(
            warp::cors()
            .allow_origin("http://localhost")
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::HEAD,
            ])
            .allow_headers(vec!["allow_origin", "allow_any_origin", "Access-Control-Allow-Origin",
                "Referer", "Control-Request-Headers", "Content-Type"])
            .max_age(300)
            .allow_any_origin(),
    );
    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
pub fn exists(p: String) -> bool {
    fs::metadata(p).is_ok()
}
