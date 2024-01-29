use std::env;

use rocket::serde::Serialize;

pub struct Res;

impl Res {
    pub async fn send<T: Serialize>(&self, value: T) -> Result<reqwest::Response, reqwest::Error> {
        let api = env::var("API").expect("API not found on .env file");
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("PAGE_ACCESS_TOKEN not found on .env file");
        let facebook_api = format!("{api}{page_access_token}");
        reqwest::Client::new()
            .post(&facebook_api)
            .json(&value)
            .send()
            .await
    }
}
