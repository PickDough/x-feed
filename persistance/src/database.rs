use std::error::Error;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use entity::message::{self, ActiveModel, Entity as MessageEntity};
use futures::Stream;
use model::message::Message;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DatabaseConnection, QuerySelect};
use tokio::time;
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct MessageReadDatabase {
    conn: DatabaseConnection,
}

impl MessageReadDatabase {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl MessageReadDatabase {
    pub fn fetch_message_stream(
        &self,
        limit: u64,
    ) -> impl Stream<Item = Result<Vec<Message>, Box<dyn Error>>> + 'static {
        let stream = IntervalStream::new(time::interval(Duration::from_secs(1)));
        let atomic_id = AtomicUsize::new(0);

        let conn = self.conn.clone();

        let stream = stream.map(move |_| {
            let messages = futures::executor::block_on(
                MessageEntity::find()
                    .filter(
                        message::Column::Id
                            .gt(atomic_id.load(std::sync::atomic::Ordering::Relaxed) as i64),
                    )
                    .limit(limit)
                    .all(&conn),
            );

            let messages = messages.map(|messages| {
                if messages.is_empty() {
                    return vec![];
                }
                atomic_id.store(
                    messages.iter().map(|m| m.id as usize).max().unwrap(),
                    std::sync::atomic::Ordering::Relaxed,
                );

                messages
                    .into_iter()
                    .map(|m| Message {
                        id: m.id as u64,
                        title: m.title,
                        text: m.text,
                    })
                    .collect::<Vec<Message>>()
            });

            messages.map_err(|e| Box::new(e) as Box<dyn Error>)
        });

        stream
    }
}

#[derive(Clone)]
pub struct MessageStoreDatabase {
    conn: DatabaseConnection,
}

impl MessageStoreDatabase {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl MessageStoreDatabase {
    pub async fn post_messages(&self, messages: Vec<Message>) -> Result<(), Box<dyn Error>> {
        let messages = messages
            .into_iter()
            .map(|m| {
                let mut m = ActiveModel {
                    id: ActiveValue::NotSet,
                    title: ActiveValue::Set(m.title),
                    text: ActiveValue::Set(m.text),
                };
                m.id = ActiveValue::NotSet;

                m
            })
            .collect::<Vec<_>>();

        let message = MessageEntity::insert_many(messages)
            .on_empty_do_nothing()
            .exec(&self.conn)
            .await;

        message
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
