use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use colored::Colorize;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::restore::restore_handler;
use std::env;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use util::dates::{get_dates, get_last_dates};
use util::folders::create_folders;

use crate::jobs::register_hourly_cron;

mod db;
mod jobs;
mod routes;
mod util;

fn print_success(success: bool, label: String) {
    let res;
    if success {
        res = format!(
            "{:?} {} {}\n",
            chrono::Local::now(),
            label.green(),
            "Success".green(),
        );
    } else {
        res = format!(
            "{:?} {} {}\n",
            chrono::Local::now(),
            label.red(),
            "Failed".red(),
        );
    }
    print!("{}", res);
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(std::env::var("LOG_FILE").unwrap().to_string())
        .unwrap();
    if let Err(e) = writeln!(file, "{}", res) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

async fn home_route() -> impl Responder {
    HttpResponse::Ok().body("Backup Service")
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();
    let args = [
        "-av --delete".to_string(),
        "--exclude={'docker-volumes/mariadb/ib_logfile0','docker-volumes/mariadb/ibtmp1'}"
            .to_string(),
    ]
    .to_vec();

    // create_folders(target_folder.clone(), user.clone(), server.clone());

    let sched = JobScheduler::new().await.unwrap();
    register_hourly_cron(
        Arc::new(Mutex::new(connection)),
        sched.clone(),
        args.clone(),
    )
    .await;

    // //daily
    // {
    //     let args = args.clone();
    //     let target_folder = target_folder.clone();
    //     let src_folder = src_folder.clone();
    //     sched
    //         .add(
    //             Job::new("1 16 1 * * *", move |_, _| {
    //                 let (_, _, day, _) = get_dates();
    //                 let (_, _, last_day, _) = get_last_dates();
    //                 let args = args.clone();
    //                 let success = execute(
    //                     "rsync".to_string(),
    //                     [
    //                         args[0].clone(),
    //                         args[1].clone(),
    //                         src_folder.clone(),
    //                         format!(
    //                             "--compare-dest='{}'",
    //                             format!("{}/daily/{}", target_folder, last_day)
    //                         ),
    //                         format!("{}/daily/{}", target_folder, day),
    //                     ]
    //                     .to_vec(),
    //                 );
    //                 print_success(success, "Daily".to_string());
    //             })
    //             .unwrap(),
    //         )
    //         .await
    //         .unwrap();
    // }

    // // //weekly
    // {
    //     let args = args.clone();
    //     let target_folder = target_folder.clone();
    //     let src_folder = src_folder.clone();
    //     sched
    //         .add(
    //             Job::new("1 14 1 * * 1", move |_, _| {
    //                 let (_, week, _, _) = get_dates();
    //                 let (_, last_week, _, _) = get_last_dates();
    //                 let success = execute(
    //                     "rsync".to_string(),
    //                     [
    //                         args[0].clone(),
    //                         args[1].clone(),
    //                         src_folder.clone(),
    //                         format!(
    //                             "--compare-dest='{}'",
    //                             format!("{}/weekly/{}", target_folder, last_week)
    //                         ),
    //                         format!("{}/weekly/{}", target_folder, week),
    //                     ]
    //                     .to_vec(),
    //                 );
    //                 print_success(success, "Weekly".to_string());
    //             })
    //             .unwrap(),
    //         )
    //         .await
    //         .unwrap();
    // }

    // // //monthly
    // {
    //     let args = args.clone();
    //     let target_folder = target_folder.clone();
    //     let src_folder = src_folder.clone();
    //     sched
    //         .add(
    //             Job::new("1 12 1 1 * *", move |_, _| {
    //                 let (month, _, _, _) = get_dates();
    //                 let success = execute(
    //                     "rsync".to_string(),
    //                     [
    //                         args[0].clone(),
    //                         args[1].clone(),
    //                         src_folder.clone(),
    //                         format!("{}/monthly/{}", target_folder, month),
    //                     ]
    //                     .to_vec(),
    //                 );
    //                 print_success(success, "Monthly".to_string());
    //             })
    //             .unwrap(),
    //         )
    //         .await
    //         .unwrap();
    // }

    sched.start().await.unwrap();
    println!("Started backup scheduler");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_route))
            .service(restore_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
