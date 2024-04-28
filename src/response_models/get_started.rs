use rocket::serde::Serialize;

use super::{payload::Payload, ResponseModel};

#[derive(Debug, Clone, Serialize)]
pub struct GetStartedModel {
    get_started: String,
}

impl GetStartedModel {
    pub fn new(payload: Payload) -> Self {
        Self {
            get_started: payload.to_string(),
        }
    }
}

impl ResponseModel for GetStartedModel {
    const END_POINT: &'static str = "messenger_profile";
}
