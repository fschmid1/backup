use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::schema::backup_jobs;

#[derive(Serialize, Queryable, Debug, PartialEq, Eq, Identifiable)]
#[table_name = "backup_jobs"]
pub struct BackupJob {
    pub id: String,
    pub name: String,
    pub src: String,
    pub dst: String,
    pub hourly: bool,
    pub daily: bool,
    pub weekly: bool,
    pub monthly: bool,
}

#[derive(Deserialize, Serialize, Clone, Insertable, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[table_name = "backup_jobs"]
pub struct UpdateBackupJob {
    pub id: String,
    pub name: Option<String>,
    pub src: Option<String>,
    pub dst: Option<String>,
    pub hourly: Option<bool>,
    pub daily: Option<bool>,
    pub weekly: Option<bool>,
    pub monthly: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBackupJob {
    pub success: bool,
}
