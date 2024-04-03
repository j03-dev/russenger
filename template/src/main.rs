use russenger::{
    button::Button,
    create_action,
    generic::{GenericElement, GenericModel},
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
    russenger_app,
    text::TextModel,
    Data, Req, Res,
};

create_action!(Main, |res: Res, req: Req| async move {
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!"))
        .await;

    let payload_1 = Payload::new(Option1, Some(Data::new("payload_for_option_1", None)));
    let payload_2 = Payload::new(Option2, Some(Data::new("payload_for_option_2", None)));

    let replies = vec![
        QuickReply::new("Option1", "", payload_1),
        QuickReply::new("Option2", "", payload_2),
    ];
    res.send(QuickReplyModel::new(
        &req.user,
        "Choose an option:",
        replies,
    ))
    .await;
});

create_action!(Option1, |res: Res, req: Req| async move {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 1 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;
});

create_action!(Option2, |res: Res, req: Req| async move {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 2 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;

    let generic_elements = vec![GenericElement::new(
        "Option 2",
        "https://example.com/option2.jpg",
        "Option 2 description",
        vec![Button::Postback {
            title: "Choose Option 2",
            payload: Payload::new(Main, None),
        }],
    )];

    res.send(GenericModel::new(
        &req.user,
        generic_elements,
        req.data.get_page(),
    ))
    .await;
});

russenger_app!(Main, Option1, Option2);
