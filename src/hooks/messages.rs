use rocket::serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Sender {
    pub id: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Recipient {
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Message {
    pub text: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Postback {
    pub title: String,
    pub payload: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Messaging {
    pub sender: Sender,
    pub postback: Option<Postback>,
    pub message: Option<Message>
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub messaging: Vec<Messaging>,
}

#[derive(Debug, Deserialize)]
pub struct MsgFromFb {
    pub entry: Vec<Entry>,
}

impl MsgFromFb {
    pub fn get_sender(&self) -> String {
        self.entry[0].messaging[0].sender.id.clone()
    }

    pub fn get_message(&self) -> Option<Message> {
       self.entry[0].messaging[0].message.clone()
    }

    pub fn get_postback(&self) -> Option<Postback> {
       self.entry[0].messaging[0].postback.clone()
    }
}

