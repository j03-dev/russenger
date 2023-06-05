use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct Sender {
    pub id: String,
}

#[derive(Deserialize)]
pub struct Message {
    pub text: String,
}

#[derive(Deserialize)]
pub struct Messaging {
    pub message: Message,
    pub sender: Sender,
}

#[derive(Deserialize)]
pub struct Entry {
    pub messaging: Vec<Messaging>,
}

#[derive(Deserialize)]
pub struct FacebookMessage {
    pub entry: Vec<Entry>,
}

impl FacebookMessage {
    pub fn get_message(&self) -> String {
        self.entry[0].messaging[0].message.text.clone()
    }

    pub fn get_sender(&self) -> String {
        self.entry[0].messaging[0].sender.id.clone()
    }
}
