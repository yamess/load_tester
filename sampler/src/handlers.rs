use actix_web::{get, HttpResponse, post, Responder, web};
use crate::schemas::{RequestPayload, ResponsePayload};

#[get("")]
pub async fn healthcheck() -> impl Responder {
    log::info!("Healthcheck request received");
    HttpResponse::Ok().finish()
}

#[post("/predict")]
pub async fn make_prediction(payload: web::Json<RequestPayload>) -> impl Responder {
    let payload = payload.into_inner();
    log::info!("Prediction request received: {:?}", payload);
    let y = payload.x1 * 2.0 - payload.x2 + 1.0 + 0.27;
    HttpResponse::Ok().json(ResponsePayload { y })
}