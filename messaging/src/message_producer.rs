use std::error::Error;

use kafka::producer::{Producer, Record};

use model::message::Message;

pub struct MessageProducer {
    producer: Producer,
    topic: String,
}

impl MessageProducer {
    pub fn new(producer: Producer, topic: String) -> Self {
        Self { producer, topic }
    }
}

impl MessageProducer {
    pub fn post_message(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_string(&message)?;
        let record = Record::from_value(&self.topic, message.as_bytes());

        self.producer.send(&record)?;

        Ok(())
    }
}
