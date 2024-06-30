use dotenv::dotenv;
use kafka::{client::FetchOffset, consumer::Consumer};
use sea_orm::{ConnectOptions, Database};
use std::str;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connect_options = ConnectOptions::new(std::env::var("DATABASE_URL").unwrap()).to_owned();

    let db = Database::connect(connect_options).await.unwrap();

    let store = persistance::database::MessageStoreDatabase::new(db.clone());

    let mut consumer = Consumer::from_hosts(vec![std::env::var("KAFKA_URL").unwrap()])
        .with_topic(std::env::var("KAFKA_TOPIC").unwrap())
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                // If the consumer receives an event, this block is executed
                println!("{:?}", str::from_utf8(m.value).unwrap());
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}
