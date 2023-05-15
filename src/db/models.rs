use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(
    Serialize, Queryable, Debug, Clone, PartialEq, Eq, Identifiable, Deserialize, AsChangeset,
)]
#[table_name = "backup_jobs"]
pub struct BackupJob {
    pub id: String,
    pub name: String,
    pub src: String,
    pub dst: String,
    pub is_ready: bool,
    pub hourly: bool,
    pub daily: bool,
    pub weekly: bool,
    pub monthly: bool,
}

#[derive(Deserialize, Serialize, Insertable, PartialEq, Eq, AsChangeset, Debug, Clone)]
#[table_name = "backup_jobs"]
pub struct NewBackupJob {
    pub id: Option<String>,
    pub name: String,
    pub src: String,
    pub dst: String,
    pub hourly: bool,
    pub daily: bool,
    pub weekly: bool,
    pub monthly: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBackupJob {
    pub success: bool,
}

#[derive(Serialize, Queryable, Debug, PartialEq, Eq, Identifiable)]
#[table_name = "logs"]
pub struct Log {
    pub id: Option<i32>,
    pub job_id: Option<String>,
    pub level: i32,
    pub message: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
