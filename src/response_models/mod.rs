use std::env;

use dotenv::dotenv;

use crate::response_models::media::{MediaAttachment, MediaModel};
use crate::response_models::quick_replies::{QuickMessage, QuickReplieModel};

use self::{
    generic::{GenericElement, GenericMessage, GenericModel, Recipient},
    quick_replies::QuickReplie,
    simple_model::{SimpleModel, Text},
};

pub mod generic;
pub mod media;
pub mod quick_replies;
pub mod simple_model;

pub enum Response<'l> {
    TextMessage(String),
    QuickReply(&'l str, Vec<QuickReplie<'l>>),
    Generic(Vec<GenericElement<'l>>),
    Media(MediaAttachment<'l>),
}

pub async fn send(sender: String, response: Response<'_>) -> bool {
    dotenv().ok();
    let api = env::var("API").expect("please check your .env file (api)");
    let page_access_token =
        env::var("PAGE_ACCESS_TOKEN").expect("please check your .env file (page access token)");
    let facebook_api = api + &page_access_token;

    let client = reqwest::Client::new();

    match response {
        Response::TextMessage(text) => {
            let text = SimpleModel {
                recipient: Recipient { id: sender },
                message: Text {
                    text: text.as_str(),
                },
            };
            client.post(facebook_api).json(&text).send().await.is_ok()
        }
        Response::QuickReply(text, quick_replies) => {
            let message = QuickMessage {
                text,
                quick_replies,
            };
            let quick_repilie = QuickReplieModel::new(Recipient { id: sender }, message);
            client
                .post(facebook_api)
                .json(&quick_repilie)
                .send()
                .await
                .is_ok()
        }
        Response::Generic(elements) => {
            let generic_model =
                GenericModel::new(Recipient { id: sender }, GenericMessage::new(elements));
            client
                .post(facebook_api)
                .json(&generic_model)
                .send()
                .await
                .is_ok()
        }
        Response::Media(attachment) => {
            let media_model = MediaModel::new(Recipient { id: sender }, attachment);
            client
                .post(facebook_api)
                .json(&media_model)
                .send()
                .await
                .is_ok()
        }
    }
}
