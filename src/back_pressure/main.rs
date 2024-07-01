use dotenv::dotenv;
use kafka::{client::FetchOffset, consumer::Consumer};
use sea_orm::{ConnectOptions, Database};

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

    let sleep_time = std::env::var("SLEEP_TIME_MS")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            let messages = ms
                .messages()
                .iter()
                .map(|m| serde_json::from_slice(m.value).unwrap())
                .collect();

            store
                .post_messages(messages)
                .await
                .expect("Failed to save messages");

            consumer.consume_messageset(ms).unwrap();
        }

        let _ = consumer.commit_consumed();
        tokio::time::sleep(tokio::time::Duration::from_millis(sleep_time)).await;
    }
}
