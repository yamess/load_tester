use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use crate::routes;

pub async fn run() -> std::io::Result<()> {
    let addr = "0.0.0.0:8082";
    log::info!("Starting web server at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::routes)
    })
        .bind(addr)?
        .run()
        .await
}