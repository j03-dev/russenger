use russenger::prelude::*;
use russenger::models::RussengerUser;

#[action]
async fn Main(res: Res, _req: Req) {
    let message: String = req.data.get_value();
    if message.to_lowercase() == "hello" {
        res.send(TextModel::new(&req.user, "Hello, welcome !"))
            .await;
    }
    res.send(GetStartedModel::new(Payload::new(Start, None)))
        .await;
}

#[action]
async fn Start(res: Res, req: Req) {
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "hello world".to_string(),
            payload: Payload::new(HelloWorld, None),
        }],
    ))
    .await;
}

#[action]
async fn HelloWorld(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Hello World")).await; // End
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser], &conn);
    
    russenger::actions![Main, HelloWorld, Start];
    russenger::launch().await;
}