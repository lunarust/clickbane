# Clickbane

Clickbane is a small automation tool designed to eliminate repetitive, soul-crushing clicks when managing BI and reporting setups across environments.

It helps you:

Move Metabase dashboards, questions, and collections safely between environments (sandbox → demo → prod)

Automate and wire JasperServer scheduled reports to your own business logic

Onboard new clients faster, with fewer mistakes and far less clicking

In short:
Clickbane handles the boring glue work so humans don’t have to.

Built with:

A Rust backend for speed, safety, and reliability

Support for multiple databases (PostgreSQL, MySQL, reporting & business DBs)

A lightweight web UI to keep things visible and controllable

If your workflow involves dashboards, reports, environments, and too much manual configuration — Clickbane is here to slay that pain.

```
├── bi_management
│   ├── backend
│   │   ├── Cargo.toml
│   │   ├── configuration.db
│   │   ├── jobtemplate
│   │   ├── makefile
│   │   ├── src
│   │   └── target
│   ├── common
│   │   ├── Cargo.toml
│   │   └── src
│   └── frontend
│       ├── Cargo.toml
│       ├── dist
│       ├── GitHub_Invertocat_Black_Clearspace.png
│       ├── index.html
│       ├── src
│       ├── styles.scss
│       ├── target
│       └── Trunk.toml
├── docker
│   ├── docker-compose.yml
│   ├── docker-entrypoint-initdb.d
│   │   └── 001_mysqlsampledatabase.sql
│   ├── dockerfile_backend
│   ├── jasper.env
│   └── jasperserver-import
├── JasperServer.md
└── README.md

```



# TODO & Issues:
- [x] Filter on scheduled jobs page
- [ ] page reports.rs, problem with the checkboxes & saving process
- [x] need to refresh the list of report scheduled for a customer
- [x] button delete all for a customer is not implemented
- [ ] need to create a form for the parameters mapping between jasper report & business db
- [ ] create the entire module for metabase import & export
- [ ] Rust-Sqlite3 - trouble with simple row result, need to review later.
- [ ] Option to select another DB, like PostgreSQL instead of MySQL
- [ ] Set Backend URL in Frontend as environments variable


# Configuration

- Url access to Jasper
- Jasper credentials
- Jasper DB [Host, port, type, Username, password]
- Business DB [Host, port, type, Username, password]
- Metabase instances [URL, Username, password]


# Apps

## Jasper  

### Backend entrypoints:

| METHOD | Entrypoint | Params & Body |
| ------ | ---------- | --------------|
| GET  | localhost:9000/jasper/failed_jobs | / |
| POST | localhost:9000/jasper/replay/job_1 | Body XML Job ID|


Examples:
```bash
curl --location 'localhost:9000/jasper/failed_jobs'
```

### Installation, requirements and so on.
#### JasperServer

Using a container to avoid a full install
https://hub.docker.com/r/judahpaul/jasperserver


Local access: http://localhost:8080/login.html
Default Username & password: jasperadmin/jasperadmin

Get Jasper Studio trial version or community edition.
Created a couple of reports.

JasperServer used REST/API

- Replay a Job
```bash
curl -X POST http://jasperadmin:MYAWESOMEPASSWORD@localhost:8080/jasperserver/rest_v2/jobs/restart/ \
   -H "Content-Type: application/xml" \
   -H "Accept: application/xml" \
   -d "<jobIdList><jobId>49993</jobId><jobId>219307</jobId></jobIdList>"
```

## Scheduled Jobs

### List all failed scheduled job

```sql
-- PostgreSQL
SELECT
label, occurrence_date, trigger_group, job_name, job_group, jb.description, trigger_state, trigger_type, misfire_instr
FROM QRTZ_TRIGGERS  qt
JOIN JIREPORTJOB jb on id::text = substr(qt.job_name, 5,LENGTH(qt.job_name))
, JILogEvent jl where (event_text LIKE '%ReportJobs.' ||  job_name ||'%')
and occurrence_date > now() - interval '7 hours' order by occurrence_date desc

--MySQL

SELECT
  label, occurrence_date, trigger_group, job_name, job_group, jb.description, trigger_state, trigger_type, misfire_instr, jl.event_text, jl.event_data
FROM QRTZ_TRIGGERS  qt
JOIN JIReportJob jb ON CONCAT('job_', id) = qt.job_name
, JILogEvent jl
WHERE (event_text LIKE '%ReportJobs.' ||  job_name ||'%')
AND jl.occurrence_date >= CURDATE()
ORDER BY occurrence_date DESC

```

# Mysql DB:

> [!Note]
> Create a .mysql-root file containing your instance root password.


[MySQL - docker hub](https://hub.docker.com/_/mysql)

```bash
mysql  -h 127.0.0.1 --protocol tcp -u root -p
mysql  -h 127.0.0.1 --protocol tcp -u root -p classicmodels
mysql  -h 127.0.0.1 --protocol tcp -u root -p < /home/rust/git/rust_web/reporting_mgmt/docker/docker-entrypoint-initdb.d/001_mysqlsampledatabase.sql
```


```sql
mysql> SHOW DATABASES;
+--------------------+
| Database           |
+--------------------+
| classicmodels      |
| information_schema |
| mysql              |
| performance_schema |
| sys                |
+--------------------+
5 rows in set (0.00 sec)


ALTER TABLE customers
ADD email varchar(120);
```


```sql
-- Creating a fake email address linked to my Clients for the purpose of this exercice
UPDATE customers SET email = CONCAT(TRIM(contactLastName), '_', TRIM(contactFirstName), '@jo.com');

-- Check
SELECT contactLastName, contactFirstName, email, customerName FROM customers order by customerName;
```

Using dummy data from:
[MySQL Sample database](https://www.mysqltutorial.org/getting-started-with-mysql/mysql-sample-database/)



 [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
