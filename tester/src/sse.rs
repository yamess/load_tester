use actix::Actor;
use futures::stream;
use actix_web::{HttpResponse, post, Responder, web};
use actix_web::http::StatusCode;
use crate::actor::LoadTestActor;
use crate::schemas::LoadTestPayload;
use actix_web::web::Bytes;

#[post("/sse")]
pub async fn sse(payload: web::Json<LoadTestPayload>) -> impl Responder {

        let payload = payload.into_inner();
    let arbiter = actix::Arbiter::new().handle();
    let actor = LoadTestActor::new("load_test".to_string());
    let addr = LoadTestActor::start_in_arbiter(&arbiter, |_| actor);

    let res = addr.send(payload).await.unwrap();

    let server_event = stream::unfold(Some(res), |state| async {
        match state {
            Some(data) => Some((Ok::<Bytes, actix_web::error::Error>(Bytes::from(data)), None)),
            None => {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
               None
            },
        }
    });
    HttpResponse::build(StatusCode::OK)
        .content_type("text/event-stream")
        .streaming(server_event)
}