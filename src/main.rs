use chrono::prelude::*;
use colored::Colorize;
use dotenv::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

fn get_dates() -> (u32, u32, u32, u32) {
    let now = chrono::Local::now();
    let month = now.month();
    let week = (now.day() - 1) / 7 + 1;
    let day = now.weekday().num_days_from_monday() + 1;
    let hour = now.hour();
    (month, week, day, hour)
}

fn get_last_dates() -> (u32, u32, u32, u32) {
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

fn execute(command: String, args: Vec<String>) -> bool {
    println!("{} {}", command.clone(), args.clone().join(" "));
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);
    let output = cmd.output().expect("failed to execute process");
    return output.status.success();
}

fn create_ifnot_exitsts(folder: String) {
    let mut cmd = std::process::Command::new("mkdir");
    cmd.arg("-p");
    cmd.arg(folder);
    cmd.output().expect("failed to execute process");
}

fn create_folders(target_folder: String) {
    create_ifnot_exitsts(format!("{}/hourly", target_folder));
    create_ifnot_exitsts(format!("{}/weekly", target_folder));
    create_ifnot_exitsts(format!("{}/monthly", target_folder));

    for i in 1..25 {
        create_ifnot_exitsts(format!("{}/hourly/{}", target_folder, i));
    }
    for i in 1..6 {
        create_ifnot_exitsts(format!("{}/weekly/{}", target_folder, i));
    }
    for i in 1..13 {
        create_ifnot_exitsts(format!("{}/monthly/{}", target_folder, i));
    }
}

fn print_success(success: bool, label: String) {
    if success {
        println!(
            "{} {} {:?}",
            label.green(),
            "Success".green(),
            chrono::Local::now()
        );
    } else {
        println!(
            "{} {} {:?}",
            label.red(),
            "Failed".red(),
            chrono::Local::now()
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    dotenv().ok();
    let server = std::env::var("SERVER").unwrap().to_string();
    let user = std::env::var("USER").unwrap().to_string();
    let src_folder = std::env::var("SRC_FOLDER").unwrap().to_string();
    let target_folder = std::env::var("TARGET_FOLDER").unwrap().to_string();
    let args = [
        "-av".to_string(),
        "--exclude={'docker-volumes/mariadb/ib_logfile0','docker-volumes/mariadb/ibtmp1'}"
            .to_string(),
    ]
    .to_vec();

    create_folders(target_folder.clone());

    let sched = JobScheduler::new().await.unwrap();

    //hourly
    {
        let args = args.clone();
        let server = server.clone();
        let user = user.clone();
        let target_folder = target_folder.clone();
        let src_folder = src_folder.clone();
        sched
            .add(Job::new("1 18 * * * *", move |_, _| {
                let (month, week, day, hour) = get_dates();
                let (_, _, _, last_hour) = get_last_dates();
                let success = execute(
                    "rsync".to_string(),
                    [
                        args[0].clone(),
                        args[1].clone(),
                        src_folder.clone(),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/monthly/{}", target_folder, month)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/weekly/{}", target_folder, week)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/daily/{}", target_folder, day)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/hourly/{}", target_folder, last_hour)
                        ),
                        format!("{}@{}:{}/hourly/{}", user, server, target_folder, hour),
                    ]
                    .to_vec(),
                );
                print_success(success, "Hourly".to_string());
            })?)
            .await?;
    }

    //daily
    {
        let args = args.clone();
        let server = server.clone();
        let user = user.clone();
        let target_folder = target_folder.clone();
        let src_folder = src_folder.clone();
        sched
            .add(Job::new("1 16 1 * * *", move |_, _| {
                let (month, week, day, _) = get_dates();
                let (_, _, last_day, _) = get_last_dates();
                let args = args.clone();
                let success = execute(
                    "rsync".to_string(),
                    [
                        args[0].clone(),
                        args[1].clone(),
                        src_folder.clone(),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/monthly/{}", target_folder, month)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/weekly/{}", target_folder, week)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/daily/{}", target_folder, last_day)
                        ),
                        format!("{}@{}:{}/daily/{}", user, server, target_folder, day),
                    ]
                    .to_vec(),
                );
                print_success(success, "Daily".to_string());
            })?)
            .await?;
    }

    // //weekly
    {
        let args = args.clone();
        let server = server.clone();
        let user = user.clone();
        let target_folder = target_folder.clone();
        let src_folder = src_folder.clone();
        sched
            .add(Job::new("1 14 1 * * 1", move |_, _| {
                let (month, week, _, _) = get_dates();
                let (_, last_week, _, _) = get_last_dates();
                let success = execute(
                    "rsync".to_string(),
                    [
                        args[0].clone(),
                        args[1].clone(),
                        src_folder.clone(),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/monthly/{}", target_folder, month)
                        ),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/weekly/{}", target_folder, last_week)
                        ),
                        format!("{}@{}:{}/weekly/{}", user, server, target_folder, week),
                    ]
                    .to_vec(),
                );
                print_success(success, "Weekly".to_string());
            })?)
            .await?;
    }

    // //monthly
    {
        let args = args.clone();
        let server = server.clone();
        let user = user.clone();
        let target_folder = target_folder.clone();
        let src_folder = src_folder.clone();
        sched
            .add(Job::new("1 12 1 1 * *", move |_, _| {
                let (month, _, _, _) = get_dates();
                let (last_month, _, _, _) = get_last_dates();
                let success = execute(
                    "rsync".to_string(),
                    [
                        args[0].clone(),
                        args[1].clone(),
                        src_folder.clone(),
                        format!(
                            "--compare-dest='{}'",
                            format!("{}/monthly/{}", target_folder, last_month)
                        ),
                        format!("{}@{}:{}/monthly/{}", user, server, target_folder, month),
                    ]
                    .to_vec(),
                );
                print_success(success, "Monthly".to_string());
            })?)
            .await?;
    }

    sched.start().await?;

    println!("Started backup scheduler");
    loop {}
    Ok(())
}
