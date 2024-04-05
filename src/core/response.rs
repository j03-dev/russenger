use std::env;

use crate::ResponseModel;

#[derive(Debug)]
pub enum SendResult {
    Okey(reqwest::Response),
    Error(reqwest::Error),
}

pub struct Res;

impl Res {
    pub async fn send<T: ResponseModel>(&self, response_model: T) -> SendResult {
        let version = env::var("FACEBOOK_API_VERSION").unwrap_or("v15.0".into());
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("env variable `PAGE_ACCESS_TOKEN` should be set");
        let url_api = format!(
            "https://graph.facebook.com/{version}/me/{endpoint}?access_token={page_access_token}",
            endpoint = response_model.get_endpoint()
        );
        match reqwest::Client::new()
            .post(url_api)
            .json(&response_model)
            .send()
            .await
        {
            Ok(response) => SendResult::Okey(response),
            Err(error) => SendResult::Error(error),
        }
    }
}
