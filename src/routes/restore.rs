use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RestoreRequest {
    pub day: i32,
    pub week: i32,
    pub month: i32,
    pub hour: i32,
}

#[derive(Serialize)]
struct RestoreResponse {
    day: i32,
    week: i32,
    month: i32,
    hour: i32,
}
#[post("/restore")]
pub async fn restore_handler(request: web::Json<RestoreRequest>) -> impl Responder {
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

    HttpResponse::Ok().json(RestoreResponse {
        day: request.day.clone(),
        week: request.week.clone(),
        month: request.month.clone(),
        hour: request.hour.clone(),
    })
}
