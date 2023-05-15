use crate::db::{models::BackupJob};

pub trait Restoreable {
    fn restore(&self, dates: (i32, i32, i32, i32));
}

impl Restoreable for BackupJob {
    fn restore(&self, dates: (i32, i32, i32, i32)) {
        let (_month, _weel, _day, _hour) = dates;
        tokio::spawn(async move {});
    }
}
