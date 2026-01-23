# JasperServer CE

Running JasperServer Community Edition in a container for this sandbox (see docker-compose.yml).

## How to Deploy the container:

- Mount only the 2 first volumes.
- Specify the path for the keystore in your environment file, note I also changed it in keystore.init.properties.
- Start the container a first time to init the database and deploy all the files.
- Add the last volume pointing to an empty file.
- Download MariaDB jar connection driver and paste it under /opt/sandbox_jasper/jasper_webapp/ROOT/WEB-INF/lib
- Restart the container.

[Manually add maria db driver](https://repo1.maven.org/maven2/org/mariadb/jdbc/mariadb-java-client/2.5.3/)
Note, newer drivers won't work.

## Rest API

- list reports
http://localhost:8080/rest_v2/resources?limit=0&type=reportUnit&recursive=true
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<resources>
    <resourceLookup>
        <creationDate>2026-01-16T09:11:08</creationDate>
        <description></description>
        <label>Orders</label>
        <permissionMask>1</permissionMask>
        <updateDate>2026-01-16T09:11:08</updateDate>
        <uri>/Reports/Orders</uri>
        <version>0</version>
        <resourceType>reportUnit</resourceType>
    </resourceLookup>
    <resourceLookup>
        <creationDate>2026-01-16T09:10:43</creationDate>
        <description></description>
        <label>Payments</label>
        <permissionMask>1</permissionMask>
        <updateDate>2026-01-16T09:10:43</updateDate>
        <uri>/Reports/Payments</uri>
        <version>0</version>
        <resourceType>reportUnit</resourceType>
    </resourceLookup>
</resources>
```
- parameters
http://localhost:8080/rest_v2/reports/Reports/Payments/inputControls/

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<inputControls>
    <inputControl>
        <dataType>
            <strictMax>false</strictMax>
            <strictMin>false</strictMin>
            <type>number</type>
        </dataType>
        <id>customerNumber</id>
        <label>customerNumber</label>
        <mandatory>false</mandatory>
        <masterDependencies/>
        <readOnly>false</readOnly>
        <slaveDependencies/>
        <state>
            <id>customerNumber</id>
            <uri>/Reports/Payments_files/customerNumber</uri>
            <value></value>
        </state>
        <type>singleValueNumber</type>
        <uri>repo:/Reports/Payments_files/customerNumber</uri>
        <visible>true</visible>
    </inputControl>
</inputControls>
```
## Useful queries:


### List of scheduled reports

```sql
-- PostgreSQL
SELECT
to_timestamp(cast(next_fire_time/1000 as bigint)) as next_fire,
to_timestamp(cast(prev_fire_time/1000 as bigint)) as prev_fire,
label,
cron_expression, trigger_state,
report_unit_uri, string_agg(address, ', ')
FROM QRTZ_TRIGGERS  qt
join QRTZ_JOB_DETAILS qjd using (job_name)
LEFT join QRTZ_CRON_TRIGGERS qct on (qt.trigger_name=qct.trigger_name and trigger_type = 'CRON')
LEFT Join QRTZ_SIMPLE_TRIGGERS qst on (qst.trigger_name=qt.trigger_name and trigger_type = 'SIMPLE')
JOIN JIReportJob jb on id::text = substr(qt.job_name, 5,LENGTH(qt.job_name))

LEFT JOIN JIReportJobMail jbm on (mail_notification = jbm.id)
LEFT JOIN JIReportJobMailRecipient jbr ON (jbr.destination_id = jbm.id)
where trigger_state != 'PAUSED'
GROUP BY 1,2,3,4,5,6
order by 1,3

-- MySQL

SELECT label,
DATE_FORMAT(FROM_UNIXTIME(next_fire_time/1000), '%Y-%m-%d %T') as next_fire,
DATE_FORMAT(FROM_UNIXTIME(prev_fire_time/1000), '%Y-%m-%d %T') as prev_fire,
base_output_name,
cron_expression, trigger_state,
report_unit_uri,
address
FROM QRTZ_TRIGGERS  qt
JOIN QRTZ_JOB_DETAILS qjd using (job_name)
LEFT JOIN QRTZ_CRON_TRIGGERS qct ON (qt.trigger_name=qct.trigger_name and trigger_type = 'CRON')
LEFT JOIN QRTZ_SIMPLE_TRIGGERS qst ON (qst.trigger_name=qt.trigger_name and trigger_type = 'SIMPLE')
JOIN JIReportJob jb on id = substr(qt.job_name, 5,LENGTH(qt.job_name))
LEFT JOIN JIReportJobMail jbm on (mail_notification = jbm.id)
LEFT JOIN JIReportJobMailRecipient jbr ON (jbr.destination_id = jbm.id)
WHERE trigger_state != 'PAUSED'
ORDER BY 1,3,4;


+---------------------+-----------+------------------+-------------------------+-----------------+---------------+-----------------+------------+
| next_fire           | prev_fire | base_output_name | label                   | cron_expression | trigger_state | report_unit_uri | address    |
+---------------------+-----------+------------------+-------------------------+-----------------+---------------+-----------------+------------+
| 2026-01-12 05:19:00 | NULL      | Orders           | [COMPANY] Weekly report | NULL            | WAITING       | /Reports/Orders | jo@jo.plop |
+---------------------+-----------+------------------+-------------------------+-----------------+---------------+-----------------+------------+


```
### List of today's failed scheduled job

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

+-------------------------+---------------------+---------------+----------+------------+-------------+---------------+--------------+---------------+------------------------+
| label                   | occurrence_date     | trigger_group | job_name | job_group  | description | trigger_state | trigger_type | misfire_instr | event_data             |
+-------------------------+---------------------+---------------+----------+------------+-------------+---------------+--------------+---------------+------------------------+
| [COMPANY] Weekly report | 2026-01-09 08:54:44 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 11:19:49 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 11:20:32 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 11:21:52 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:32:26 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:33:09 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:34:58 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:34:58 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:40:04 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
| [COMPANY] Weekly report | 2026-01-09 15:40:04 | ReportJobs    | job_1    | ReportJobs |             | WAITING       | SIMPLE       |             0 | NULL                   |
+-------------------------+---------------------+---------------+----------+------------+-------------+---------------+--------------+---------------+------------------------+

```


### Scheduled reports & failed jobs

```sql
SELECT label,
DATE_FORMAT(FROM_UNIXTIME(next_fire_time/1000), '%Y-%m-%d %T') as next_fire,
DATE_FORMAT(FROM_UNIXTIME(prev_fire_time/1000), '%Y-%m-%d %T') as prev_fire,
base_output_name,
cron_expression, trigger_state,
report_unit_uri,
address, max(je.occurrence_date) as occurence_date, max(je.message) as err_message
FROM QRTZ_TRIGGERS  qt
JOIN QRTZ_JOB_DETAILS qjd using (job_name)
LEFT JOIN QRTZ_CRON_TRIGGERS qct ON (qt.trigger_name=qct.trigger_name and trigger_type = 'CRON')
LEFT JOIN QRTZ_SIMPLE_TRIGGERS qst ON (qst.trigger_name=qt.trigger_name and trigger_type = 'SIMPLE')
JOIN JIReportJob jb on id = substr(qt.job_name, 5,LENGTH(qt.job_name))
LEFT JOIN JIReportJobMail jbm on (mail_notification = jbm.id)
LEFT JOIN JIReportJobMailRecipient jbr ON (jbr.destination_id = jbm.id)
LEFT JOIN JILogEvent je ON (resource_uri=report_unit_uri) AND event_text LIKE CONCAT('%Job: ', label, '%')
WHERE trigger_state != 'PAUSED'
GROUP BY 1,2,3,4,5,6,7,8
ORDER BY 1,3,4;


```
