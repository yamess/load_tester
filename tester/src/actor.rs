use actix::{Actor, ActorFutureExt, ContextFutureSpawner, Handler, ResponseActFuture, Running, WrapFuture};
use uuid::Uuid;
use crate::schemas::LoadTestPayload;
use crate::service::run_load_test;

pub struct LoadTestActor {
    pub name: String
}

impl LoadTestActor {
    pub fn new(name: String) -> LoadTestActor {
        LoadTestActor {
            name
        }
    }
}

impl Actor for LoadTestActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Actor {} started", self.name);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        log::info!("Actor {} stopping", self.name);
        Running::Stop
    }
}

impl Handler<LoadTestPayload> for LoadTestActor {
    type Result = ResponseActFuture<Self, Vec<u8>>;

    fn handle(&mut self, msg: LoadTestPayload, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Actor {} received message: {:?}", self.name, msg);

        async move {
            let file_path = format!("/tmp/{}.html", Uuid::new_v4().to_string());
            run_load_test(msg, file_path.clone()).await.unwrap();
            tokio::fs::read(file_path).await.unwrap()
        }
            .into_actor(self)
            .map(|res, _act, _ctx| res)
            .boxed_local()

    }
}