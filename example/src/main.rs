use russenger::prelude::*;

create_action!(Main, |res: Res, req: Req| async move {
    res.send(TextModel::new(&req.user, "Hello!")).await;
    res.send(TextModel::new(&req.user, "What is your name: "))
        .await;
    req.query.set_action(&req.user, GetUserInput).await;
});

create_action!(GetUserInput, |res: Res, req: Req| async move {
    let username: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &format!("hello {username}")))
        .await;

    let payload = |value: &str| Payload::new(NextAction, Some(Data::new(value, None)));

    // QuickReply
    let quickreplies: Vec<QuickReply> = vec![
        QuickReply::new("blue", "", payload("blue")),
        QuickReply::new("red", "", payload("red")),
    ];
    let quickreplymodel = QuickReplyModel::new(&req.user, "choose one color", quickreplies);
    res.send(quickreplymodel).await;
});

create_action!(NextAction, |res: Res, req: Req| async move {
    let color: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &color)).await;
    Main.execute(res, req).await; // goto Main action
});

russenger_app!(Main, GetUserInput, NextAction);
