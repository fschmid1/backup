use std::sync::{Arc, Mutex};

use diesel::ExpressionMethods;
use diesel::{query_dsl::methods::FilterDsl, RunQueryDsl, SqliteConnection};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    db::{models::BackupJob, schema::backup_jobs},
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
                    let backup_jobs = backup_jobs::table
                        .filter(backup_jobs::hourly.eq(true))
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

pub async fn register_daily_cron(
    connection: Arc<Mutex<SqliteConnection>>,
    sched: JobScheduler,
    args: Vec<String>,
) {
    let connection = connection.clone();
    sched
        .add(
            Job::new("1 14 1 * * 1", move |_, _| {
                let (_, _, day, _) = get_dates();
                let (_, _, last_day, _) = get_last_dates();
                let args = args.clone();
                if let Ok(mut conn) = connection.lock() {
                    let backup_jobs = backup_jobs::table
                        .filter(backup_jobs::daily.eq(true))
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
                                    format!("{}/daily/{}", backup_job.dst, last_day)
                                ),
                                format!("{}/daily/{}", backup_job.dst, day),
                            ]
                            .to_vec(),
                        );
                        print_success(success, "Daily".to_string());
                    });
                }
            })
            .unwrap(),
        )
        .await
        .unwrap();
}

pub async fn register_weekly_cron(
    connection: Arc<Mutex<SqliteConnection>>,
    sched: JobScheduler,
    args: Vec<String>,
) {
    let connection = connection.clone();
    sched
        .add(
            Job::new("1 16 1 * * **", move |_, _| {
                let (_, week, _, _) = get_dates();
                let (_, last_week, _, _) = get_last_dates();
                let args = args.clone();
                if let Ok(mut conn) = connection.lock() {
                    let backup_jobs = backup_jobs::table
                        .filter(backup_jobs::daily.eq(true))
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
                                    format!("{}/weekly/{}", backup_job.dst, last_week)
                                ),
                                format!("{}/weekly/{}", backup_job.dst, week),
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

pub async fn register_monthly_cron(
    connection: Arc<Mutex<SqliteConnection>>,
    sched: JobScheduler,
    args: Vec<String>,
) {
    let connection = connection.clone();
    sched
        .add(
            Job::new("1 12 1 1 * *", move |_, _| {
                let (month, _, _, _) = get_dates();
                let args = args.clone();
                if let Ok(mut conn) = connection.lock() {
                    let backup_jobs = backup_jobs::table
                        .filter(backup_jobs::daily.eq(true))
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
                                format!("{}/monthly/{}", backup_job.dst, month),
                            ]
                            .to_vec(),
                        );
                        print_success(success, "Monthly".to_string());
                    });
                }
            })
            .unwrap(),
        )
        .await
        .unwrap();
}
