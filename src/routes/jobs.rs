use std::sync::{Arc, Mutex};

use crate::db::models::{DeleteBackupJob, NewBackupJob, UpdateBackupJob};

use super::super::db::{models::BackupJob, schema::backup_jobs};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use diesel::{ExpressionMethods, QueryDsl};
use diesel::{RunQueryDsl, SqliteConnection};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_jobs))
            .route(web::post().to(create_job))
            .route(web::patch().to(update_job)),
    );
    cfg.service(web::resource("/{id}").route(web::delete().to(delete_job)));
}

async fn get_jobs(conn: web::Data<Arc<Mutex<SqliteConnection>>>) -> impl Responder {
    if let Ok(mut conn) = conn.lock() {
        let backup_jobs = backup_jobs::table.load::<BackupJob>(&mut *conn);
        let backup_jobs: Vec<crate::db::models::BackupJob> = match backup_jobs {
            Ok(backup_jobs) => backup_jobs,
            Err(err) => {
                println!("Error: {}", err);
                return HttpResponse::InternalServerError().body("Something went wrong");
            }
        };
        return HttpResponse::Ok().json(backup_jobs);
    }
    return HttpResponse::InternalServerError().body("Something went wrong");
}

async fn create_job(
    conn: web::Data<Arc<Mutex<SqliteConnection>>>,
    mut job: web::Json<NewBackupJob>,
) -> impl Responder {
    if let Ok(mut conn) = conn.lock() {
        job.id = Some(uuid::Uuid::new_v4().to_string());
        match diesel::insert_into(backup_jobs::table)
            .values(job.clone())
            .execute(&mut *conn)
        {
            Ok(_) => return HttpResponse::Ok().json(job),
            Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
        }
    }
    return HttpResponse::InternalServerError().body("Something went wrong");
}

async fn update_job(
    conn: web::Data<Arc<Mutex<SqliteConnection>>>,
    job: web::Json<UpdateBackupJob>,
) -> impl Responder {
    if let Ok(mut conn) = conn.lock() {
        match diesel::update(backup_jobs::table)
            .filter(backup_jobs::id.eq(job.id.clone()))
            .set(job.clone())
            .execute(&mut *conn)
        {
            Ok(_) => return HttpResponse::Ok().json(job),
            Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
        }
    }
    return HttpResponse::InternalServerError().body("Something went wrong");
}

async fn delete_job(
    conn: web::Data<Arc<Mutex<SqliteConnection>>>,
    id: web::Path<String>,
) -> impl Responder {
    if let Ok(mut conn) = conn.lock() {
        match diesel::delete(backup_jobs::table.filter(backup_jobs::id.eq(id.clone())))
            .execute(&mut *conn)
        {
            Ok(_) => return HttpResponse::Ok().json(DeleteBackupJob { success: true }),
            Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
        }
    }
    return HttpResponse::InternalServerError().body("Something went wrong");
}
