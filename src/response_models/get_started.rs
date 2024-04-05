use rocket::serde::Serialize;

use crate::{payload::Payload, ResponseModel};

#[derive(Debug, Clone, Serialize)]
pub struct GetStarted {
    get_started: String,
}

impl GetStarted {
    pub fn new(payload: Payload) -> Self {
        Self {
            get_started: payload.to_string(),
        }
    }
}

impl ResponseModel for GetStarted {
    const END_POINT: &'static str = "messenger_profile";
}
