use russenger::prelude::*;

#[derive(FromRow, Clone, Model)]
pub struct Register {
    #[field(primary_key = true)]
    pub id: Serial,
    #[field(foreign_key = "RussengerUser.facebook_user_id", unique = true)]
    pub user_id: String,

    #[field(size = 30, unique = true)]
    pub username: String,
}

async fn index(res: Res, req: Req) -> Result<()> {
    res.send(TextModel::new(&req.user, "Hello!")).await?;
    if let Some(user_register) =
        Register::get(kwargs!(user_id == req.user), &req.query.conn).await?
    {
        res.send(TextModel::new(
            &req.user,
            format!("Hello {}", user_register.username),
        ))
        .await?;
    } else {
        res.send(TextModel::new(&req.user, "What is your name: "))
            .await?;
        res.redirect("/signup").await?;
        return Ok(());
    }
    res.redirect("/get_user_input").await?;

    Ok(())
}

async fn signup(res: Res, req: Req) -> Result<()> {
    let username: String = req.data.get_value()?;
    let message = if Register::create(
        kwargs!(user_id = req.user, username = username),
        &req.query.conn,
    )
    .await
    .is_ok()
    {
        "Register success"
    } else {
        "Register failed"
    };
    res.send(TextModel::new(&req.user, message)).await?;
    index(res, req).await?;

    Ok(())
}

async fn get_user_input(res: Res, req: Req) -> Result<()> {
    let payload = |value: &str| Payload::new("/next_action", Some(Data::new(value)));

    // QuickReply
    let quick_replies = [
        QuickReply::new("blue", None, payload("blue")),
        QuickReply::new("red", None, payload("red")),
    ];
    let quick_reply_field = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
    res.send(quick_reply_field).await?;

    Ok(())
}

async fn next_action(res: Res, req: Req) -> Result<()> {
    let color: String = req.data.get_value()?;
    res.send(TextModel::new(&req.user, &color)).await?;
    index(res, req).await?; // goto index action
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    App::init()
        .await?
        .attach(
            Router::new()
                .add("/", index)
                .add("/signup", signup)
                .add("/get_user_input", get_user_input)
                .add("/next_action", next_action),
        )
        .launch()
        .await?;

    Ok(())
}
