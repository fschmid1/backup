use std::sync::{Arc, Mutex};

use diesel::{connection, ExpressionMethods};
use diesel::{query_dsl::methods::FilterDsl, QueryResult, RunQueryDsl, SqliteConnection};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    db::{models::BackupJob, schema::job},
    print_success,
    util::{
        dates::{get_dates, get_last_dates},
        shell::execute,
    },
};

pub async fn register_hourly_cron(
    connection: Arc<Mutex<SqliteConnection>>,
    sched: JobScheduler,
    args: Vec<String>,
) {
    let connection = connection.clone();
    sched
        .add(
            Job::new("1 18 * * * *", move |_, _| {
                let (_, _, _, hour) = get_dates();
                let (_, _, _, last_hour) = get_last_dates();
                let args = args.clone();
                if let Ok(mut conn) = connection.lock() {
                    let backup_jobs = job::table
                        .filter(job::hourly.eq(true))
                        .load::<BackupJob>(&mut *conn);
                    let backup_jobs: Vec<BackupJob> = match backup_jobs {
                        Ok(backup_jobs) => backup_jobs,
                        Err(err) => {
                            println!("Error: {}", err);
                            return;
                        }
                    };
                    backup_jobs.iter().for_each(|backup_job| {
                        let success = execute(
                            "rsync".to_string(),
                            [
                                args[0].clone(),
                                args[1].clone(),
                                backup_job.src.clone(),
                                format!(
                                    "--compare-dest='{}'",
                                    format!("{}/hourly/{}", backup_job.dst, last_hour)
                                ),
                                format!("{}/hourly/{}", backup_job.dst, hour),
                            ]
                            .to_vec(),
                        );
                        print_success(success, "Hourly".to_string());
                    });
                }
            })
            .unwrap(),
        )
        .await
        .unwrap();
}
