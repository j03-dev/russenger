use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sender {
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickReplyPayload {
    pub payload: String,
}

impl QuickReplyPayload {
    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    pub text: Option<String>,
    pub quick_reply: Option<QuickReplyPayload>,
}

impl Message {
    pub fn get_text(&self) -> String {
        self.text.clone().unwrap_or_default()
    }

    pub fn get_quick_reply(&self) -> Option<QuickReplyPayload> {
        self.quick_reply.clone()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Postback {
    pub payload: String,
}

impl Postback {
    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Messaging {
    pub sender: Sender,
    pub postback: Option<Postback>,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub messaging: Vec<Messaging>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InComingData {
    pub entry: Vec<Entry>,
}

impl InComingData {
    pub fn get_sender(&self) -> &String {
        &self.entry[0].messaging[0].sender.id
    }

    pub fn get_message(&self) -> Option<Message> {
        self.entry[0].messaging[0].message.clone()
    }

    pub fn get_postback(&self) -> Option<Postback> {
        self.entry[0].messaging[0].postback.clone()
    }
}
