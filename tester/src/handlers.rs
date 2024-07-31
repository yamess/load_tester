use std::fmt::format;
use actix_web::{get, HttpResponse, post, Responder, web};
use async_tempfile::TempFile;
use uuid::Uuid;
use crate::schemas::LoadTestPayload;
use crate::service::run_load_test;

#[get("")]
pub async fn healthcheck() -> impl Responder {
    log::info!("Healthcheck request received");
    HttpResponse::Ok().finish()
}

#[post("/task")]
pub async fn submit_task(payload: web::Json<LoadTestPayload>) -> impl Responder {
    log::info!("Task submission request received");
    let payload= payload.into_inner();

    let file_path = format!("/tmp/{}.html", Uuid::new_v4().to_string());

    run_load_test(payload, file_path.clone()).await.unwrap();
    let file = tokio::fs::read(file_path).await.unwrap();
    HttpResponse::Ok().body(file)
}