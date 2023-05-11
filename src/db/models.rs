use diesel::{Identifiable, Insertable, Queryable};
use serde::Serialize;

use super::schema::job;

#[derive(Serialize, Queryable, Insertable, Debug, PartialEq, Eq, Identifiable)]
#[table_name = "job"]
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
