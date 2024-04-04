use std::env;

use rocket::serde::Serialize;
use serde_json::json;

use crate::{payload::Payload, persistent_menu::PersistentMenu};

#[derive(Debug)]
pub enum SendResult {
    Okey(reqwest::Response),
    Error(reqwest::Error),
}

async fn send<T: Serialize>(data: T, endpoint: &str) -> SendResult {
    let version = env::var("FACEBOOK_API_VERSION").unwrap_or("v15.0".into());
    let page_access_token =
        env::var("PAGE_ACCESS_TOKEN").expect("env variable `PAGE_ACCESS_TOKEN` should be set");
    let url_api = format!(
        "https://graph.facebook.com/{version}/me/{endpoint}?access_token={page_access_token}"
    );
    match reqwest::Client::new()
        .post(url_api)
        .json(&data)
        .send()
        .await
    {
        Ok(response) => SendResult::Okey(response),
        Err(error) => SendResult::Error(error),
    }
}

pub struct Res;

impl Res {
    pub async fn send<T: Serialize>(&self, response_model: T) -> SendResult {
        send(response_model, "messages").await
    }
    pub async fn send_user_setting(&self, persistent_menu: PersistentMenu<'_>) -> SendResult {
        send(persistent_menu, "custom_user_settings").await
    }

    pub async fn send_get_started(&self, payload: Payload) -> SendResult {
        send(
            json!({"get_started": {"payload": payload.to_string()}}),
            "messenger_profile",
        )
        .await
    }
}
