use std::env;

use rocket::serde::Serialize;

#[derive(Debug)]
pub enum SendResult {
    Okey(reqwest::Response),
    Error(reqwest::Error),
}

pub struct Res;

impl Res {
    pub async fn send<T: Serialize>(&self, value: T) -> SendResult {
        let version = env::var("FACEBOOK_API_VERSION").unwrap_or("v15.0".into());
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("env variable `PAGE_ACCESS_TOKEN` should be set");
        let facebook_api = format!(
            "https://graph.facebook.com/{version}/me/messages?access_token={page_access_token}"
        );
        match reqwest::Client::new()
            .post(&facebook_api)
            .json(&value)
            .send()
            .await
        {
            Ok(response) => SendResult::Okey(response),
            Err(error) => SendResult::Error(error),
        }
    }
}
