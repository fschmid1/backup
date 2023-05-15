use super::models::BackupJob;

use super::schema::backup_jobs;

use diesel::{Connection, ExpressionMethods, QueryDsl};
use diesel::{RunQueryDsl, SqliteConnection};
use std::env;


#[derive(Debug)]
pub struct BackupJobError {
    pub message: String,
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_job(id: String) -> Result<BackupJob, BackupJobError> {
    let backup_job: Result<BackupJob, diesel::result::Error> = backup_jobs::table
        .filter(backup_jobs::id.eq(id.clone()))
        .first::<BackupJob>(&mut establish_connection());
    return backup_job.map_err(|_| BackupJobError {
        message: format!("BackupJob #{} not found", id),
    });
}

pub async fn update_job(job: BackupJob) -> Result<usize, BackupJobError> {
    let mut conn = establish_connection();
    diesel::update(backup_jobs::table)
        .set(job.clone())
        .execute(&mut conn)
        .map_err(|_| BackupJobError {
            message: format!("Error while updating Job#{}", job.id),
        })
}
