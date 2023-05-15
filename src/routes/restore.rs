use crate::db::{traits::Restoreable, wrapper::get_job};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Deserialize, Validate)]
pub struct RestoreRequest {
    #[validate(minimum = 1)]
    #[validate(maximum = 7)]
    pub day: i32,
    #[validate(minimum = 1)]
    #[validate(maximum = 4)]
    pub week: i32,
    #[validate(minimum = 1)]
    #[validate(maximum = 12)]
    pub month: i32,
    #[validate(minimum = 1)]
    #[validate(maximum = 24)]
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

pub async fn restore_handler(request: web::Json<RestoreRequest>) -> impl Responder {
    if let Err(err) = request.validate() {
        return HttpResponse::BadRequest().body(err.to_string());
    }
    let backup_job = match get_job(request.id.clone()).await {
        Ok(backup_job) => backup_job,
        Err(err) => {
            return HttpResponse::BadRequest().body(err.message);
        }
    };
    backup_job.restore((request.month, request.week, request.day, request.hour));
    return HttpResponse::Ok().json(RestoreResponse {
        day: request.day.clone(),
        week: request.week.clone(),
        month: request.month.clone(),
        hour: request.hour.clone(),
        dst: backup_job.dst,
    });
}
