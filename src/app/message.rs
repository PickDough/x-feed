use std::{sync::atomic::AtomicUsize, time::Duration};

use actix_web::{get, post, web, Error, HttpResponse, Responder};
use async_stream::try_stream;
use entity::message::{self, Entity as MessageEntity};
use futures::{future::ok, stream::once};
use futures::{FutureExt, StreamExt, TryStreamExt};
use sea_orm::entity::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use tokio::{stream, time};
use tokio_stream::wrappers::IntervalStream;

use crate::model::message::Message;

use super::builder::AppState;

pub trait MessageStore {
    fn fetch_message(&self) -> Vec<Message>;
    fn post_message(&self, message: Message) -> Message;
}

#[derive(Debug, Deserialize)]
struct FetchMessageQuery {
    #[serde(default = "default_limit")]
    limit: u64,
}

const fn default_limit() -> u64 {
    32
}

#[get("/streams")]
async fn streams() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(
        b"{\"test\": \"test\"}",
    )));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[get("/message")]
async fn fetch_message(
    query: web::Query<FetchMessageQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut stream = IntervalStream::new(time::interval(Duration::from_secs(1)));
    let atomic_id = AtomicUsize::new(0);

    let conn = state.conn.clone();

    let stream = stream.map(move |_| {
        let messages = futures::executor::block_on(
            MessageEntity::find()
                .filter(
                    message::Column::Id
                        .gt(atomic_id.load(std::sync::atomic::Ordering::Relaxed) as i32),
                )
                .all(&conn),
        )
        .unwrap();

        if let Some(message) = messages.last() {
            atomic_id.store(message.id as usize, std::sync::atomic::Ordering::Relaxed);
        }

        if messages.is_empty() {
            return Ok::<actix_web::web::Bytes, Error>(web::Bytes::new());
        }

        println!("{:?}", messages);
        let messages = messages
            .into_iter()
            .map(|message| Message {
                id: u64::try_from(message.id).unwrap(),
                title: message.title,
                text: message.text,
            })
            .collect::<Vec<_>>();

        Ok::<actix_web::web::Bytes, Error>(web::Bytes::copy_from_slice(
            serde_json::to_string(&messages).unwrap().as_bytes(),
        ))
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::pin(stream))
}

#[post("/message")]
async fn post_message(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
