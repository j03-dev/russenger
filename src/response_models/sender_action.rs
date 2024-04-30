use super::{recipient::Recipient, ResponseModel};
use rocket::serde::Serialize;

pub enum Actions {
    MarkSeen,
    TypingOn,
    TypingOff,
}

#[derive(Debug, Clone, Serialize)]
pub struct SenderActionModel<'a> {
    messaging_type: &'a str,
    recipient: Recipient<'a>,
    sender_action: &'a str,
}

impl<'a> SenderActionModel<'a> {
    pub fn new(sender: &'a str, action: Actions) -> Self {
        let sender_action = match action {
            Actions::MarkSeen => "mark_seen",
            Actions::TypingOn => "typing_on",
            Actions::TypingOff => "typing_off",
        };
        Self {
            messaging_type: "RESPONSE",
            recipient: Recipient { id: sender },
            sender_action,
        }
    }
}

impl ResponseModel for SenderActionModel<'_> {
    const END_POINT: &'static str = "messages";
}
