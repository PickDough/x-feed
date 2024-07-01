#![warn(unused_extern_crates)]
mod app;

use actix_web::middleware;
use app::builder::AppState;
use dotenv::dotenv;
use env_logger::Env;
use std::sync::Arc;
use std::sync::Mutex;

use kafka::producer::Producer;
use sea_orm::{ConnectOptions, Database};

use actix_web::HttpServer;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let connect_options = ConnectOptions::new(std::env::var("DATABASE_URL").unwrap()).to_owned();

    let db = Database::connect(connect_options).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    let store = persistance::database::MessageReadDatabase::new(db.clone());

    let producer = Producer::from_hosts(vec![std::env::var("KAFKA_URL").unwrap()])
        .with_ack_timeout(std::time::Duration::from_secs(1))
        .with_required_acks(kafka::producer::RequiredAcks::One)
        .create()
        .unwrap();

    let state = AppState {
        message_store: store,
        producer: Arc::new(Mutex::new(
            messaging::message_producer::MessageProducer::new(
                producer,
                std::env::var("KAFKA_TOPIC").unwrap(),
            ),
        )),
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        app::builder::build_app(state.clone()).wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", std::env::var("PORT").unwrap().parse().unwrap()))?
    .run()
    .await
}
