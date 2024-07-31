use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::handlers::{healthcheck, submit_task};
use crate::sse::sse;

pub fn routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(healthcheck)
            .service(submit_task)
            .service(sse)
    );
}