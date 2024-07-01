use actix_web::{get, post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;

use model::message::Message;

use super::builder::AppState;

#[derive(Debug, Deserialize)]
struct FetchMessageQuery {
    #[serde(default = "default_limit")]
    limit: u64,
}

const fn default_limit() -> u64 {
    32
}

#[get("/message")]
async fn fetch_message(
    query: web::Query<FetchMessageQuery>,
    state: web::Data<AppState>,
) -> impl Responder {
    let message_stream = state.message_store.fetch_message_stream(query.limit);

    let message_stream = message_stream
        .map(|messages| {
            messages.map(|m| {
                if m.is_empty() {
                    return web::Bytes::new();
                }
                web::Bytes::copy_from_slice(
                    serde_json::to_string(&m)
                        .unwrap_or_else(|_| "".to_string())
                        .as_bytes(),
                )
            })
        })
        // TODO: Logging would be nice
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error fetching messages"));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(message_stream)
}

#[derive(Debug, Deserialize)]
struct PostMessageBody {
    title: String,
    text: String,
}

impl Into<Message> for PostMessageBody {
    fn into(self) -> Message {
        Message {
            id: 0,
            title: self.title,
            text: self.text,
        }
    }
}

#[post("/message")]
async fn post_message(
    message: web::Json<PostMessageBody>,
    state: web::Data<AppState>,
) -> impl Responder {
    let message: Message = message.into_inner().into();

    let message = state.producer.lock().unwrap().post_message(message);

    match message {
        Ok(_) => HttpResponse::Ok().json(()),
        // TODO: Logging would be nice
        Err(_) => HttpResponse::InternalServerError().json("Could not save message"),
    }
}
