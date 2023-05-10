use chrono::prelude::*;

pub fn get_dates() -> (u32, u32, u32, u32) {
    let now = chrono::Local::now();
    let month = now.month();
    let week = (now.day() - 1) / 7 + 1;
    let day = now.weekday().num_days_from_monday() + 1;
    let hour = now.hour();
    (month, week, day, hour)
}

pub fn get_last_dates() -> (u32, u32, u32, u32) {
    let now = chrono::Local::now();
    let month = if now.month() - 1 == 0 {
        12
    } else {
        now.month() - 1
    };
    let week = if now.day() - 7 <= 0 {
        1
    } else {
        (now.day() - 7) / 7 + 1
    };
    let day = if now.weekday().num_days_from_monday() - 1 <= 0 {
        7
    } else {
        now.weekday().num_days_from_monday()
    };
    let hour = if now.hour() - 1 <= 0 {
        24
    } else {
        now.hour() - 1
    };

    (month, week, day, hour)
}
