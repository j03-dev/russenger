use std::env;

use self::{
    generic::{GenericElement, GenericModel},
    media::MediaModel,
    quick_replies::{QuickReplie, QuickReplieModel},
    text::TextModel,
};
use dotenv::dotenv;

pub mod generic;
pub mod media;
pub mod quick_replies;
pub mod text;

pub enum Response<'l> {
    TextMessage(&'l str),
    QuickReply(&'l str, Vec<QuickReplie<'l>>),
    Generic(Vec<GenericElement<'l>>),
    Media(&'l str, &'l str),
}

impl<'l> Response<'l> {
    pub async fn send(&self, sender: &'l str) -> Result<reqwest::Response, reqwest::Error> {
        // Load environment variables from a .env file if present.
        dotenv().ok();

        let api = env::var("API").expect("Please check your .env file (api)");
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("Please check your .env file (page access token)");
        let facebook_api = format!("{}{}", api, page_access_token);

        let client = reqwest::Client::new();
        let response = match &self {
            Response::TextMessage(text) => {
                let text_model = TextModel::new(sender, text);
                client.post(&facebook_api).json(&text_model).send().await
            }
            Response::QuickReply(message, quick_replies) => {
                let quick_reply_model = QuickReplieModel::new(sender, message, quick_replies);
                client
                    .post(&facebook_api)
                    .json(&quick_reply_model)
                    .send()
                    .await
            }
            Response::Generic(elements) => {
                let generic_model = GenericModel::new(sender, elements);
                client.post(&facebook_api).json(&generic_model).send().await
            }
            Response::Media(r#type, url) => {
                let media_model = MediaModel::new(sender, r#type, url);
                client.post(&facebook_api).json(&media_model).send().await
            }
        };

        response
    }
}
