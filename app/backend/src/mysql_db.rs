use mysql::*;
use mysql::prelude::*;
use common::*;
use crate::generic;

pub async fn query_customers() -> Result<Vec<Customer>> {//Result<(), Box<dyn std::error::Error>> {// Result<Vec<Customer>> {
    generic::logthis(format!("Entering query customers").as_str(), "INFO");

    let url = "mysql://root:mywonderfulpassword@localhost:3306/classicmodels";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let query = "SELECT customerNumber, customerName, contactLastName, contactFirstName, email
        FROM customers
        ORDER BY customerName;";
    let result:Vec<(i32, String, String, String, String)> = conn.query(query)?;

    let mut custs: Vec<Customer> = vec![];
    for r in result {
        //println!("{:?}", r);
        custs.push(Customer {
            customerNumber: r.0,
            customerName: r.1,
            contactLastName: r.2,
            contactFirstName: r.3,
            email: r.4,
        });
    }
    Ok(custs)
}

pub async fn query_js_jobs_per_customer(customer: String) -> Result<Vec<JS_Scheduled_Job>> {//
    generic::logthis(format!("Entering query_js_jobs_per_customer for {}", customer).as_str(), "INFO");

    let url = "mysql://root:mywonderfulpassword@localhost:3306/jasperserver";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    let mut query = "".to_string();
    let base_query = r#"WITH logs as
        (SELECT max(occurrence_date) AS occurrence_date, message,
        SUBSTRING_INDEX(SUBSTRING_INDEX(event_text , '\n', 1), '(', 1) as report, resource_uri
        FROM JILogEvent je GROUP BY 2,3,4)
        SELECT jb.id, label, qt.job_name,
        IFNULL(jb.description, '') AS description, trigger_state,
        trigger_type, DATE_FORMAT(FROM_UNIXTIME(next_fire_time/1000), '%Y-%m-%d %T') as next_fire, IFNULL(DATE_FORMAT(FROM_UNIXTIME(prev_fire_time/1000), '%Y-%m-%d %T'), '') as prev_fire,
        base_output_name, IFNULL(address, '') AS address,
        IFNULL(DATE_FORMAT(logs.occurrence_date, '%Y-%m-%d %T'), '') as occurrence_date,
        IFNULL(logs.message, '') as message
        FROM QRTZ_TRIGGERS  qt
        JOIN QRTZ_JOB_DETAILS qjd using (job_name)
        LEFT JOIN QRTZ_CRON_TRIGGERS qct ON (qt.trigger_name=qct.trigger_name and trigger_type = 'CRON')
        LEFT JOIN QRTZ_SIMPLE_TRIGGERS qst ON (qst.trigger_name=qt.trigger_name and trigger_type = 'SIMPLE')
        JOIN JIReportJob jb on id = substr(qt.job_name, 5,LENGTH(qt.job_name))
        LEFT JOIN JIReportJobMail jbm on (mail_notification = jbm.id)
        LEFT JOIN JIReportJobMailRecipient jbr ON (jbr.destination_id = jbm.id)
        LEFT JOIN logs ON (jb.report_unit_uri = logs.resource_uri AND logs.report = CONCAT('Job: ', label))"#;

     if customer != "" {
        query = format!("{} WHERE label LIKE CONCAT('[', '{}' ,']%') ORDER BY 1,3,4", base_query, customer);
     }
     else  {
        query = format!("{} ORDER BY 1,3,4", base_query);
    }
    let result:Vec<(i32, String, String,
        String, String, String,
        String, String, String,
        String, String, String,
    )> = conn.query(query)?;

    let mut jobs: Vec<JS_Scheduled_Job> = vec![];
    for r in result {
        //println!("{:?}", r);

        let mut f = true;
        if r.10 == "" { f = false; }
        jobs.push(JS_Scheduled_Job {
            id: r.0,
            label: r.1,
            //trigger_group: r.1,
            job_name: r.2,
            description: r.3,
            trigger_state: r.4,
            trigger_type: r.5,
            next_fire: r.6,
            prev_fire: r.7,
            base_output_name: r.8,
            address: r.9,
            err_message: r.10,
            occurrence_date: r.11,
            failed: f, //failed: f,
        });

    }
    Ok(jobs)


}
