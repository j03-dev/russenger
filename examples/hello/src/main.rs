use russenger::prelude::*;

#[action]
async fn Main(res: Res, _req: Req) {
    res.send(GetStartedModel::new(Payload::new(Start, None)))
        .await;
}

#[action]
async fn Start(res: Res, req: Req) {
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "hello world",
            payload: Payload::new(HelloWorld, None),
        }],
    ))
    .await;
}

#[action]
async fn HelloWorld(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Hello World")).await; // End
}

russenger_app!(Main, Start, HelloWorld);
