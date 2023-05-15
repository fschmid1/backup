use crate::db::{models::BackupJob, wrapper::update_job};

pub trait Restoreable {
    fn restore(&self, dates: (i32, i32, i32, i32));
}

impl Restoreable for BackupJob {
    fn restore(&self, dates: (i32, i32, i32, i32)) {
        let (month, weel, day, hour) = dates;
        tokio::spawn(async move {});
    }
}
