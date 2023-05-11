use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};

use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::db::wrapper::get_job;

#[derive(Deserialize)]
pub struct RestoreRequest {
    pub day: i32,
    pub week: i32,
    pub month: i32,
    pub hour: i32,
    pub id: String,
}

#[derive(Serialize)]
pub struct RestoreResponse {
    day: i32,
    week: i32,
    month: i32,
    hour: i32,
    dst: String,
}

pub async fn restore_handler(
    request: web::Json<RestoreRequest>,
    conn: web::Data<Arc<Mutex<SqliteConnection>>>,
) -> impl Responder {
    if request.month > 12 || request.month < 1 {
        return HttpResponse::BadRequest().body("Invalid month");
    }
    if request.week > 4 || request.week < 1 {
        return HttpResponse::BadRequest().body("Invalid week");
    }
    if request.day > 7 || request.day < 1 {
        return HttpResponse::BadRequest().body("Invalid day");
    }
    if request.hour > 24 || request.hour < 1 {
        return HttpResponse::BadRequest().body("Invalid hour");
    }
    let backup_job = get_job(conn, request.id.clone()).await;

    HttpResponse::Ok().json(RestoreResponse {
        day: request.day.clone(),
        week: request.week.clone(),
        month: request.month.clone(),
        hour: request.hour.clone(),
        dst: backup_job.unwrap().dst,
    })
}
