use russenger::models::RussengerUser;
use russenger::prelude::*;

#[derive(FromRow, Clone, Model)]
pub struct Register {
    #[model(primary_key = true)]
    pub id: Serial,
    #[model(
        foreign_key = "RussengerUser.facebook_user_id",
        unique = true,
        null = false
    )]
    pub user_id: String,
    #[model(size = 30, unique = true, null = false)]
    pub username: String,
}

#[action]
async fn Main(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Hello!")).await?;
    if let Some(user_register) = Register::get(kwargs!(user_id == req.user), &req.query.conn).await
    {
        res.send(TextModel::new(
            &req.user,
            &format!("Hello {}", user_register.username),
        ))
        .await?;
    } else {
        res.send(TextModel::new(&req.user, "What is your name: "))
            .await?;
        req.query.set_action(&req.user, SignUp).await;
        return Ok(());
    }
    req.query.set_action(&req.user, GetUserInput).await;

    Ok(())
}

#[action]
async fn SignUp(res: Res, req: Req) {
    let username: String = req.data.get_value();
    let message = if Register::create(
        kwargs!(user_id = req.user, username = username),
        &req.query.conn,
    )
    .await
    {
        "Register success"
    } else {
        "Register failed"
    };
    res.send(TextModel::new(&req.user, message)).await?;
    Main.execute(res, req).await?;

    Ok(())
}

#[action]
async fn GetUserInput(res: Res, req: Req) {
    let payload = |value: &str| Payload::new(NextAction, Some(Data::new(value, None)));

    // QuickReply
    let quick_replies: Vec<QuickReply> = vec![
        QuickReply::new("blue", "", payload("blue")),
        QuickReply::new("red", "", payload("red")),
    ];
    let quick_reply_model = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
    res.send(quick_reply_model).await?;

    Ok(())
}

#[action]
async fn NextAction(res: Res, req: Req) {
    let color: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &color)).await?;
    Main.execute(res, req).await?; // goto Main action
    Ok(())
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser, Register], &conn);
    russenger::actions![Main, GetUserInput, NextAction, SignUp];
    russenger::launch().await.ok();
}
