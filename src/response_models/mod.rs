use std::env;

use dotenv::dotenv;

use self::{
    generic::{GenericElement, GenericMessage, GenericModel},
    media::{MediaAttachment, MediaModel},
    quick_replies::{QuickMessage, QuickReplie, QuickReplieModel},
    text::TextModel,
};

pub mod generic;
pub mod media;
pub mod quick_replies;
pub mod text;

pub enum Response<'l> {
    TextMessage(String),
    QuickReply(&'l str, Vec<QuickReplie<'l>>),
    Generic(Vec<GenericElement>),
    Media(&'l str, &'l str),
}

impl<'l> Response<'l> {
    pub async fn send(&self, sender: String) -> Result<reqwest::Response, reqwest::Error> {
        dotenv().ok();
        let api = env::var("API").expect("please check your .env file (api)");
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("please check your .env file (page access token)");
        let facebook_api = api + &page_access_token;

        let client = reqwest::Client::new();

        match &self {
            Response::TextMessage(text) => {
                let text = TextModel::new(sender, text);
                client.post(facebook_api).json(&text).send().await
            }
            Response::QuickReply(text, quick_replies) => {
                let message = QuickMessage::new(text, quick_replies);
                let quick_repilie = QuickReplieModel::new(sender, message);
                client
                    .post(facebook_api)
                    .json(&quick_repilie)
                    .send()
                    .await
                    
            }
            Response::Generic(elements) => {
                let generic_model = GenericModel::new(sender, GenericMessage::new(elements));
                client
                    .post(facebook_api)
                    .json(&generic_model)
                    .send()
                    .await
            }
            Response::Media(r#type, url) => {
                let message = &MediaAttachment::new(r#type, url);
                let media_model = MediaModel::new(sender, message);
                client
                    .post(facebook_api)
                    .json(&media_model)
                    .send()
                    .await
            }
        }
    }
}
