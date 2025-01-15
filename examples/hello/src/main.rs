use russenger::models::RussengerUser;
use russenger::prelude::*;

#[action]
async fn index(res: Res, req: Req) -> Result<()> {
    let message: String = req.data.get_value();
    if message.to_lowercase() == "hello" {
        res.send(TextModel::new(&req.user, "Hello, welcome !"))
            .await?;
    }
    res.send(GetStartedButtonModel::new(Payload::new("/start", None)))
        .await?;

    Ok(())
}

#[action]
async fn start(res: Res, req: Req) -> Result<()> {
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "hello world".to_string(),
            payload: Payload::new("/hello_world", None),
        }],
    ))
    .await?;

    Ok(())
}

#[action]
async fn hello_world(res: Res, req: Req) -> Result<()> {
    res.send(TextModel::new(&req.user, "Hello World")).await?; // End
    Ok(())
}

#[russenger::main]
async fn main() -> Result<()> {
    let conn = Database::new().await?.conn;
    migrate!([RussengerUser], &conn);

    App::init()
        .await?
        .attach(router![
            ("/", index),
            ("/start", start),
            ("/hello_world", hello_world)
        ])
        .launch()
        .await?;
    Ok(())
}
