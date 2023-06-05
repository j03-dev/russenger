use potato::hooks::{
    messages::FacebookMessage,
    response_models::{
        generic::{GenericButton, GenericElement},
        media::MediaAttachment,
        quick_replies::QuickReplie,
        Response,
    },
};
use rocket::{post, serde::json::Json};

#[post("/", format = "json", data = "<facebook_message>")]
pub async fn core(facebook_message: Json<FacebookMessage>) -> &'static str {
    let message = facebook_message.get_message();
    let sender = facebook_message.get_sender();

    let response: Response;

    if message.eq("Hi") {
        response = Response::TextMessage("Hello World");
    } else if message.eq("Generic") {
        let vostanimey = GenericElement {
            title: "VOSTANIMEY",
            image_url: "https://vostanime.fr/wp-content/uploads/2023/05/xxxx-3.png",
            subtitle: "site de streaming",
            buttons: vec![GenericButton::new("vostanimey")],
        };
        let adkami = GenericElement {
            title: "ADKAMI",
            image_url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTwYDobBrAhxP85kXikY1fDoZIuCP-MdDdvZhhU6kJCMA&s",
            subtitle: "site de streaming",
            buttons: vec![GenericButton::new("adkami")],
        };
        response = Response::Generic(vec![vostanimey, adkami]);
    } else if message.eq("Quick") {
        let red = QuickReplie::new("text", "https://example.com/img/red.png");
        let blue = QuickReplie::new("blue", "https://example.com/img/blue.png");
        response = Response::QuickReply("Pick a color", vec![red, blue]);
    } else {
        let media_attachement = MediaAttachment::new("image", "urls/image.png");
        response = Response::Media(media_attachement);
    }

    Response::send(sender, response).await;

    "ok"
}
