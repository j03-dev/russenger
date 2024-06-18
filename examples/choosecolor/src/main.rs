use russenger::prelude::*;
use russenger::models::RussengerUser;

#[derive(FromRow, Clone, Model)]
pub struct Register {
    #[model(primary_key=true, auto=true, null=false)]
   pub id: Integer,
    #[model(foreign_key="RussengerUser.facebook_user_id", null=false, unique=true)]
   pub user: String,
    #[model(size=30, unique=true, null=false)]
   pub username: String 
}

#[action]
async fn Main(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Hello!")).await;
    if let Some(user_register) = Register::get(kwargs!(user = req.user), &req.query.conn).await {
        res.send(TextModel::new(&req.user, &format!("Hello {}", user_register.username)))
            .await;
    } else {
        res.send(TextModel::new(&req.user, "What is your name: "))
            .await;
        req.query.set_action(&req.user, SignUp).await;
        return;
    }
    req.query.set_action(&req.user, GetUserInput).await;
}

#[action]
async fn SignUp(res: Res, req: Req) {
    let username: String = req.data.get_value();
    let message = if Register::create(kwargs!(user= req.user, username = username), &req.query.conn).await  {
        "Register success"
    } else {
        "Register failed"
    };
    res.send(TextModel::new(&req.user, message)).await; 
    Main.execute(res, req).await;
}

#[action]
async fn GetUserInput(res: Res, req: Req) {
    let payload = |value: &str| Payload::new(NextAction, Some(Data::new(value, None)));

    // QuickReply
    let quickreplies: Vec<QuickReply> = vec![
        QuickReply::new("blue", "", payload("blue")),
        QuickReply::new("red", "", payload("red")),
    ];
    let quickreplymodel = QuickReplyModel::new(&req.user, "choose one color", quickreplies);
    res.send(quickreplymodel).await;
}

#[action]
async fn NextAction(res: Res, req: Req) {
    let color: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &color)).await;
    Main.execute(res, req).await; // goto Main action
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser, Register], &conn);
    
    russenger::actions![Main, GetUserInput, NextAction, SignUp];
    russenger::launch().await;
}
