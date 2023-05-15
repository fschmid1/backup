use crate::{
    db::{
        models::{DeleteBackupJob, NewBackupJob},
        wrapper::{establish_connection, get_job},
    },
    util::folders::create_folders,
};

use super::super::db::{models::BackupJob, schema::backup_jobs};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_jobs))
            .route(web::post().to(create_job))
            .route(web::patch().to(update_job)),
    );
    cfg.service(web::resource("/{id}").route(web::delete().to(delete_job)));
}

async fn get_jobs() -> impl Responder {
    let backup_jobs = backup_jobs::table.load::<BackupJob>(&mut establish_connection());
    let backup_jobs: Vec<crate::db::models::BackupJob> = match backup_jobs {
        Ok(backup_jobs) => backup_jobs,
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::InternalServerError().body("Something went wrong");
        }
    };
    return HttpResponse::Ok().json(backup_jobs);
}

async fn create_job(mut job: web::Json<NewBackupJob>) -> impl Responder {
    job.id = Some(uuid::Uuid::new_v4().to_string());
    let mut conn = establish_connection();
    match diesel::insert_into(backup_jobs::table)
        .values(job.clone())
        .execute(&mut conn)
    {
        Ok(_) => {
            drop(conn);
            let backup_job = get_job(job.id.clone().unwrap()).await.unwrap();
            create_folders(backup_job.clone());
            return HttpResponse::Ok().json(backup_job);
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn update_job(job: web::Json<BackupJob>) -> impl Responder {
    match diesel::update(backup_jobs::table)
        .filter(backup_jobs::id.eq(job.id.clone()))
        .set(job.clone())
        .execute(&mut establish_connection())
    {
        Ok(_) => return HttpResponse::Ok().json(job),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn delete_job(id: web::Path<String>) -> impl Responder {
    match diesel::delete(backup_jobs::table.filter(backup_jobs::id.eq(id.clone())))
        .execute(&mut establish_connection())
    {
        Ok(_) => return HttpResponse::Ok().json(DeleteBackupJob { success: true }),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }
}
