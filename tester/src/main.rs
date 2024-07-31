mod handlers;
mod routes;
mod schemas;
mod web;
mod service;
mod actor;
mod sse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    web::run().await
}
