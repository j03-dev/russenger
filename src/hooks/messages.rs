use rocket::serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Sender {
    id: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Recipient {
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Message {
    text: Option<String>,
}

impl Message {
    pub fn get_text(&self) -> String {
        self.text.clone().unwrap_or("".into())
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Postback {
    title: String,
    payload: String,
}

impl Postback {
    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Messaging {
    sender: Sender,
    postback: Option<Postback>,
    message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    messaging: Vec<Messaging>,
}

#[derive(Debug, Deserialize)]
pub struct MsgFromFb {
    entry: Vec<Entry>,
}

impl MsgFromFb {
    pub fn get_sender(&self) -> &String {
        &self.entry[0].messaging[0].sender.id
    }

    pub fn get_message(&self) -> Option<Message> {
        self.entry[0].messaging[0].message.clone()
    }

    pub fn get_postback(&self) -> Option<Postback> {
        self.entry[0].messaging[0].postback.clone()
    }
}
