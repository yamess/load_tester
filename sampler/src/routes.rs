use actix_web::web;
use crate::handlers::{healthcheck, make_prediction};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(healthcheck)
            .service(make_prediction),
    );
}