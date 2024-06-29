use serde::{Deserialize, Serialize};

pub type MessageId = u64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,
    pub title: String,
    pub text: String,
}
