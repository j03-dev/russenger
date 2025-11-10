pub mod handlers;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WebQuery {
    #[serde(rename = "hub.mode")]
    hub_mode: Option<String>,
    #[serde(rename = "hub.challenge")]
    hub_challenge: Option<String>,
    #[serde(rename = "hub.verify_token")]
    hub_verify_token: Option<String>,
}

impl WebQuery {
    pub fn get_hub_challenge(&self) -> HttpResponse {
        match (
            self.hub_mode.clone(),
            self.hub_challenge.clone(),
            self.hub_verify_token.clone(),
        ) {
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                if hub_mode.eq("subscribe") && std::env::var("VERIFY_TOKEN").unwrap().eq(&token) {
                    HttpResponse::Ok().body(hub_challenge)
                } else {
                    HttpResponse::Unauthorized().body("Token mismatch")
                }
            }
            _ => HttpResponse::Unauthorized().body("Insufficient arguments provided"),
        }
    }
}

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
