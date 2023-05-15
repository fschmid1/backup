use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use colored::Colorize;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::restore::restore_handler;
use std::env;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::JobScheduler;

use crate::db::wrapper::establish_connection;
use crate::jobs::{
    register_daily_cron, register_hourly_cron, register_monthly_cron, register_weekly_cron,
};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = [
        "-av --delete".to_string(),
        "--exclude={'docker-volumes/mariadb/ib_logfile0','docker-volumes/mariadb/ibtmp1'}"
            .to_string(),
    ]
    .to_vec();

    // create_folders(target_folder.clone(), user.clone(), server.clone());
    let sched = JobScheduler::new().await.unwrap();
    register_hourly_cron(sched.clone(), args.clone()).await;
    register_daily_cron(sched.clone(), args.clone()).await;
    register_weekly_cron(sched.clone(), args.clone()).await;
    register_monthly_cron(sched.clone(), args.clone()).await;

    sched.start().await.unwrap();
    println!("Started backup scheduler, running on port 8080");
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(home_route))
            .wrap(Logger::default())
            .service(web::scope("/api/jobs").configure(routes::jobs::init))
            .service(web::resource("/api/restore").route(web::post().to(restore_handler)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
