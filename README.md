# Russenger - Facebook Messenger Webhook Handling in Rust

![Russenger Logo](./image.png)

Russenger is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a
convenient way to construct and send various types of responses, including text messages, quick replies, generic
templates, and media attachments.

## New Features 
- Now you can creat your own model using [rusql-alchemy](https://github.com.com/russenger/rusql-alchemy) orm
  - You can see the [examples](https://github.com.com/russenger/rusql-alchemy/examples)

## How to Create new Project
- ### **1**: Install Cargo Generate
```bash
cargo install cargo-generate
```
- ### **2**: Generate new project
```bash
cargo generate --git https://github.com/j03-dev/russenger_template
```

## How To Use

### Example

Here's an example of how to use Russenger to handle different actions in a chatbot:

#### Russenger `Cargo.toml`

```toml
russenger = "0.2.0"
actix-web = "4"
sqlx = "^0.7.0"
rusql-alchemy = "0.1.0" # the default feature is sqlite
# rusql-alchemy = { version = "0.1.0", default-features = false, features = ["mysql"] }
# rusql-alchemy = { version = "0.1.0", default-features = false, features = ["postgres"] }
```

#### Environment `.env`

```bash
# You can change this
PORT=6969
HOST=0.0.0.0

# change this
VERIFY_TOKEN=<your-verify-token>

# change this
PAGE_ACCESS_TOKEN=<your-page-access-token>

#### postgres
# DATABASE=postgres://<user>:<password>@<host>/<db_name>

#### mysql
# DATABASE=mysql://<user>:<password>@<host>/<db_name>

#### sqlite
DATABASE=sqlite:<db_name>
```

#### Code `src/main.rs`

```rust
use russenger::prelude::*;
use russenger::models::RussengerUser;

#[derive(FromRow, Clone, Model)]
pub struct Register {
    #[model(primary_key = true, auto = true, null = false)]
    pub id: Integer,
    #[model(foreign_key = "RussengerUser.facebook_user_id", unique = true, null = false)]
    pub user_id: String,
    #[model(size = 30, unique = true, null = false)]
    pub username: String,
}

#[action]
async fn Main(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Hello!")).await;
    if let Some(user_register) = Register::get(kwargs!(user_id = req.user), &req.query.conn).await {
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
    let message = if Register::create(kwargs!(user_id = req.user, username = username), &req.query.conn).await {
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
    let quick_replies: Vec<QuickReply> = vec![
        QuickReply::new("blue", "", payload("blue")),
        QuickReply::new("red", "", payload("red")),
    ];
    let quick_reply_model = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
    res.send(quick_reply_model).await;
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
```

##### Who to get User Input

```rust
use russenger::models::RussengerUser;
use russenger::prelude::*;

#[action]
async fn Main (res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!"))
        .await;
    res.send(TextModel::new(&req.user, "What is your name: "))
        .await;
    req.query.set_action(&req.user, GetUsername).await;
}

#[action]
async fn GetUsername (res: Res, req: Req){
    let username: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &format!("Hello {}", username)))
        .await;
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser, &conn]);
    russenger::action![Main, GetUsername];
    russenger.launch().await;
}
```

##### How to send file from static

```rust
use russenger::models::RussengerUser;
use russenger::prelude::*;

#[action]
async fn Main (res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!"))
        .await;

    // Send Image File from static file
    // Add image file, on static dir
    res.send(MediaModel::new(
        &req.user,
        "image",
        &format!("{host}/static/image.png", host = req.host),
    ))
    .await;
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser, &conn]);
    russenger::action![Main];
    russenger.launch().await;
}
```

#### Run

##### Migrate Database `run it once`

if you use `sqlite` as Bb

```bash
touch <dbname>
```

```bash
cargo run migrate
```

##### Runserver

```bash
cargo run runserver
```

In this example, we define three actions: `Main`, `Option1`, and `Option2`. Each action is associated with a function that handles the action. The `Main` action sends a text message and a quick reply with two options. The `Option1` and `Option2` actions handle the user's selection of the respective options.

### EndPoint

- GET `/webhook`: Verify your chatbot with Facebook Messenger. Facebook will send a challenge, and your bot must respond correctly for verification.

- POST `/webhook`: This is where Facebook Messenger sends messages from users. Handle incoming messages and respond accordingly here.
