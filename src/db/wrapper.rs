use std::sync::{Arc, Mutex};

use super::models::BackupJob;

use super::schema::backup_jobs;
use diesel::{ExpressionMethods, QueryDsl};
use diesel::{RunQueryDsl, SqliteConnection};

#[derive(Debug)]
pub struct BackupJobError {
    pub message: String,
}

pub async fn get_job(
    conn: actix_web::web::Data<Arc<Mutex<SqliteConnection>>>,
    id: String,
) -> Result<BackupJob, BackupJobError> {
    if let Ok(mut conn) = conn.lock() {
        let backup_job: Result<BackupJob, diesel::result::Error> = backup_jobs::table
            .filter(backup_jobs::id.eq(id.clone()))
            .first::<BackupJob>(&mut *conn);
        return backup_job.map_err(|_| BackupJobError {
            message: format!("BackupJob #{} not found", id),
        });
    }
    return Err(BackupJobError {
        message: format!("BackupJob #{} not found", id),
    });
}
