use russenger::prelude::*;

create_action!(Main, |res: Res, _req: Req| async move {
    res.send(GetStartedModel::new(Payload::new(Start, None)))
        .await;
});

create_action!(Start, |res: Res, req: Req| async move {
    res.send(PersistentMenuModel::new(
        &req.user,
        vec![Button::Postback {
            title: "hello world",
            payload: Payload::new(HelloWorld, None),
        }],
    ))
    .await;
});

create_action!(HelloWorld, |res: Res, req: Req| async move {
    res.send(TextModel::new(&req.user, "Hello World")).await; // End
});

russenger_app!(Main, Start, HelloWorld);
