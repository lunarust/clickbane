use rusqlite::{Connection, Result, params};

use common::configuration::{ConfigurationJs, ConfigurationJsRequest,ConfigurationBusiness};
use common::jasper::{JS_Report,InputMapping};

use crate::generic;


#[derive(Clone, Debug)]
struct Mapped {
    id: i32,
}

pub async fn create_db() -> Result<()> {
    let conn = Connection::open(format!("{}/configuration.db", generic::get_current_working_dir()))?;
    // THIS IS NOT WORKING / ALSO NEED TO ADD BUSINESS TABLE
    conn.execute(
        "create table if not exists js_configuration (
             id integer primary key,
             js_url text not null,
             js_secret text not null,
             js_db_host text not null,
             js_db_port integer,
             js_db_user text,
             js_db_password text
         )",
        (),
    )?;

    Ok(())
}

pub async fn insert_report(jsr: JS_Report) -> Result<()> {
    generic::logthis(format!("SQLITE: insert_report: {:?}", jsr).as_str(), "INFO");

    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    //let mut ch = true;
    //if jsr.default { ch = false; }
    let frequency: u32 = jsr.frequency[0]*100 + jsr.frequency[1]*10 + jsr.frequency[2];

    //println!(">>>INSERT REPORT {:?} FOR {:?}", frequency, jsr);

    conn.execute(
        "INSERT INTO jasper_jobs (label, description, uri, param, schedule, frequency)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(uri) DO UPDATE SET
        label=excluded.label,
        schedule=excluded.schedule,
        description=excluded.description,
        param=excluded.param,
        frequency=excluded.frequency",
        params![
            jsr.label,
            jsr.description,
            jsr.uri,
            serde_json::to_string(&jsr.param).unwrap(),
            jsr.default.to_string(),
            frequency
        ],
    )?;

    Ok(())
}
pub async fn query_default_job() -> Result<Vec<JS_Report>> {

    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    let mut stmt = conn.prepare("
        SELECT label, description, uri,
            param,
            ifnull(schedule, 'false') as 'default', ifnull(frequency , 0) as frequency
        FROM jasper_jobs WHERE schedule = 'true'
        ")?;

    let stmt_res = stmt.query_map([], |row| {
        let param_json: String = row.get(3)?;
        let f: i32 = row.get(5)?;
        let freq_vec = format!("{:0>3}", f).chars().flat_map(|ch| ch.to_digit(10)).collect();
        Ok(
            JS_Report {
            label: row.get(0)?,
            description: row.get(1)?,
            uri: row.get(2)?,
            param: serde_json::from_str(&param_json).expect("DRAMA"),
            default: true,
            frequency: freq_vec,
        })
    })?;
    let mut reps: Vec<JS_Report> = vec![];
    for res in stmt_res {
        reps.push(res?);
    }
    Ok(reps.clone())
}
pub async fn get_reports_scheduled() -> Result<Vec<JS_Report>> {
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    let mut stmt = conn.prepare("SELECT label, description, uri,
        param,
        ifnull(schedule, 'false') as 'default', ifnull(frequency , 0) as frequency
        FROM jasper_jobs")?;

    let rw = stmt.query_map([], |row| {
       let param_json: String = row.get(3)?;
       let f: i32 = row.get(5)?;
       let freq: String =  format!("{:03}", f);
       let s: String = row.get(4)?;
       let sc: bool =
            match s.as_str() {
                "true" => true,
                _ => false,
            };

       Ok(
            JS_Report {
                label: row.get(0)?,
                description: row.get(1)?,
                uri: row.get(2)?,
                param: serde_json::from_str(&param_json).expect("DRAMA"),
                default: sc,
                frequency: freq.chars().flat_map(|ch| ch.to_digit(10)).collect(),
        })
    })?;
    let mut reps: Vec<JS_Report> = vec![];

    for a in rw {
        reps.push(a?);
    }
    Ok(reps)
}
pub async fn get_report_scheduled(jsf: JS_Report) -> Result<JS_Report> {
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    let mut stmt = conn.prepare(format!("SELECT label, description, uri,
        param,
        ifnull(schedule, 'false') as 'default', ifnull(frequency , 0) as frequency
        FROM jasper_jobs WHERE uri = '{}'", jsf.uri).as_str())?;

    let rw = stmt.query_map([], |row| {
       let param_json: String = row.get(3)?;
       let f: i32 = row.get(5)?;
       let freq: String =  format!("{:03}", f);
       let s: String = row.get(4)?;
       let sc: bool =
            match s.as_str() {
                "true" => true,
                _ => false,
            };

       Ok(
            JS_Report {
                label: row.get(0)?,
                description: row.get(1)?,
                uri: row.get(2)?,
                param: serde_json::from_str(&param_json).expect("DRAMA"),
                default: sc,
                frequency: freq.chars().flat_map(|ch| ch.to_digit(10)).collect(),
        })
    })?;
    let mut reps: Vec<JS_Report> = vec![];

    for a in rw {
        reps.push(a?);
    }
    if reps.len() > 0 {
        Ok(reps[0].clone())
    }
    else { Ok(jsf) }
}
pub async fn check_report_resource_mapped(label: String) -> Result<i32> {
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    let mut stmt = conn.prepare(format!("SELECT id FROM input_mapping WHERE input_id = '{}'", label).as_str())?;

    let rw = stmt.query_map([], |row| {
        Ok(Mapped {
            id: row.get(0)?,
        })
    })?;
    let mut output: i32 = 0;
    for a in rw {
        output = a.unwrap().id;
    }

    Ok(output)
}
pub async fn get_report_resource_mapping(id: i32) -> Result<Vec<InputMapping>> {
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    let mut stmt = conn.prepare(format!("SELECT id, input_id, configuration_id, configuration_table, configuration_column
        FROM input_mapping WHERE id = {}", id).as_str())?;

    let rw = stmt.query_map([], |row| {
        Ok(InputMapping {
            id: row.get(0)?,
            input_id: row.get(1)?,
            configuration_id: row.get(2)?,
            configuration_table: row.get(3)?,
            configuration_column: row.get(4)?,
        })
    })?;
    let mut ims: Vec<InputMapping> = vec![];
    for res in rw {
        ims.push(res?);
    }
    Ok(ims.clone())
}

pub async fn query_js_configuration() -> Result<ConfigurationJs> {
    generic::logthis(format!("Entering query js Configuration???").as_str(), "INFO");

    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    let mut stmt = conn.prepare("
        SELECT id, js_url, js_secret, js_db_host, js_db_port, js_db_user, js_db_password
        FROM js_configuration;")?;

    let cfg_js = stmt.query_map([], |row| {
        Ok(ConfigurationJs {
            js_id: row.get(0)?,
            js_url: row.get(1)?,
            js_secret: row.get(2)?,
            js_db_host: row.get(3)?,
            js_db_port: row.get(4)?,
            js_db_user: row.get(5)?,
            js_db_password: row.get(6)?,
        })
    })?;
    let mut cfg: Vec<ConfigurationJs> = vec![];
    for res in cfg_js {
        cfg.push(res?);
    }

    Ok(cfg[0].clone())
}
pub async fn insert_js_configuration(conf: ConfigurationJsRequest) -> Result<ConfigurationJs> {
    generic::logthis(format!("Entering insert js Configuration {:?}", conf).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    conn.execute(
        "INSERT INTO js_configuration (js_url, js_secret, js_db_host, js_db_port, js_db_user, js_db_password)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        [conf.clone().js_url, conf.clone().js_secret, conf.clone().js_db_host, conf.clone().js_db_port.to_string(), conf.clone().js_db_user, conf.clone().js_db_password],
        )?;
    let last_id = conn.last_insert_rowid();

    Ok(ConfigurationJs {
        js_id: last_id,
        js_url: conf.js_url,
        js_secret: conf.js_secret,
        js_db_host: conf.js_db_host,
        js_db_port: conf.js_db_port,
        js_db_user: conf.js_db_user,
        js_db_password: conf.js_db_password,
    })
}
pub async fn update_js_configuration(conf: ConfigurationJs) -> Result<ConfigurationJs> {
    generic::logthis(format!("Entering update js Configuration {:?}", conf).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    conn.execute(
        "UPDATE js_configuration
        set js_url = ?1,
        js_secret = ?2,
        js_db_host =?3,
        js_db_port = ?4,
        js_db_user = ?5,
        js_db_password = ?6
        WHERE id = ?7",
        [conf.clone().js_url, conf.clone().js_secret, conf.clone().js_db_host, conf.clone().js_db_port.to_string(),
            conf.clone().js_db_user, conf.clone().js_db_password, conf.clone().js_id.to_string()],
        )?;
    Ok(conf)
}
pub async fn delete_js_configuration(id: i32) -> Result<()> {
    generic::logthis(format!("Entering delete js Configuration {:?}", id).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    conn.execute(
        "DELETE from js_configuration WHERE id = ?1", [id]
    )?;
    Ok(())
}

pub async fn query_configuration() -> Result<ConfigurationBusiness> {
    generic::logthis(format!("Entering query Configuration???").as_str(), "INFO");

    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    let mut stmt = conn.prepare("
        SELECT id, name, db_host, db_port, db_user, db_password
        FROM configuration;")?;

    let cfg_business = stmt.query_map([], |row| {
        Ok(ConfigurationBusiness {
            business_id: row.get(0)?,
            business_name: row.get(1)?,
            business_db_host: row.get(2)?,
            business_db_port: row.get(3)?,
            business_db_user: row.get(4)?,
            business_db_password: row.get(5)?,
        })
    })?;
    let mut cfg: Vec<ConfigurationBusiness> = vec![];
    for res in cfg_business {
       generic::logthis(format!("looping {:?}", res).as_str(), "INFO");

        cfg.push(res?);
    }

    Ok(cfg[0].clone())
}
pub async fn insert_configuration(conf: ConfigurationBusiness) -> Result<ConfigurationBusiness> {
    generic::logthis(format!("Entering insert Configuration {:?}", conf).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    conn.execute(
        "INSERT INTO configuration (name, db_host, db_port, db_user, db_password)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        [conf.clone().business_name, conf.clone().business_db_host,
            conf.clone().business_db_port.to_string(), conf.clone().business_db_user,
            conf.clone().business_db_password],
        )?;
    let last_id = conn.last_insert_rowid();

    Ok(ConfigurationBusiness {
        business_id: last_id,
        business_name: conf.business_name,
        business_db_host: conf.business_db_host,
        business_db_port: conf.business_db_port,
        business_db_user: conf.business_db_user,
        business_db_password: conf.business_db_password,
    })
}

pub async fn update_configuration(conf: ConfigurationBusiness) -> Result<ConfigurationBusiness> {
    generic::logthis(format!("Entering update Configuration {:?}", conf).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;

    conn.execute(
        "UPDATE configuration
        set name = ?1,
        business_db_host =?2,
        business_db_port = ?3,
        business_db_user = ?4,
        business_db_password = ?5
        WHERE id = ?6",
        [conf.clone().business_name, conf.clone().business_db_host, conf.clone().business_db_port.to_string(),
            conf.clone().business_db_user, conf.clone().business_db_password, conf.clone().business_id.to_string()],
        )?;
    Ok(conf)
}
pub async fn delete_configuration(id: i32) -> Result<()> {
    generic::logthis(format!("Entering delete Configuration {:?}", id).as_str(), "INFO");
    let url = format!("{}/configuration.db", generic::get_current_working_dir());
    let conn = Connection::open(url)?;
    conn.execute(
        "DELETE from configuration WHERE id = ?1", [id]
    )?;
    Ok(())
}
