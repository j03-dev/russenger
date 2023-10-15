use rocket::serde::Serialize;
use std::env;

pub mod generic;
pub mod media;
pub mod payload;
pub mod quick_replies;
pub mod text;

#[derive(Serialize)]
pub struct Recipient<'r> {
    pub id: &'r str,
}

#[rocket::async_trait]
pub trait SendResponse: Serialize {
    async fn send(&self) -> Result<reqwest::Response, reqwest::Error> {
        let api = env::var("API").expect("API not found on .env file");
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("PAGE_ACCESS_TOKEN not found on .env file");
        let facebook_api = format!("{}{}", api, page_access_token);
        reqwest::Client::new()
            .post(&facebook_api)
            .json(&self)
            .send()
            .await
    }
}
