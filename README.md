# Russenger - Facebook Messenger Webhook Handling in Rust

![Russenger Logo](./image.png)

Welcome to Russenger, a Rust library designed to simplify the handling of Facebook Messenger webhook responses. Russenger offers a convenient way to construct and send various types of responses, including text messages, quick replies, generic templates, and media attachments.

## Features

- **Custom Models**: Developers can now use their own models with the Russenger library. This is made possible by the integration with [rusql-alchemy](https:://github.com/russenger/rusql-alchemy), an ORM for sqlx. This means that models are defined in Rust code, eliminating the need to write SQL queries.
- **Easy to Use**: The Russenger library is designed to be easy to use. It provides a set of modules and macros that abstract away the complexities of building a bot, allowing you to focus on the logic of your application.
- **Flexible**: The Russenger library is flexible and can be used to build a wide variety of bots. It supports text-based conversations, quick replies, and custom actions.

## Getting Started

To get started with the Russenger library, you'll need to install it as a dependency in your Rust project. You can do this by adding the following line to your `Cargo.toml` file:

```toml
[dependencies]
russenger = { version = "0.3.O-rc", features = ["postgres"] } # features 'sqlite, postgres, mysql'
actix-web = "4"
sqlx = "^0.7.0"
```

Once you've installed the library, you can start building your bot! Check out the [documentation](https://docs.rs/russenger) for more information on how to use the library.

## Creating a New Project

To create a new project using the Russenger library, you can use the `cargo-generate` tool. Here are the steps:

1. Install `cargo-generate`:

```bash
cargo install cargo-generate
```

2. Generate a new project:

```bash
cargo generate --git https://github.com/j03-dev/russenger_template
```

## Examples

Here are some examples of what you can build with the Russenger library:

### A simple bot that greets users and asks for their name

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
    if let Some(user_register) = Register::get(kwargs!(user_id == req.user), &req.query.conn).await {
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
    Main.execute(res, req).await; // go back to Main action
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser, Register], &conn);

    russenger::actions![Main, GetUserInput, NextAction, SignUp];
    russenger::launch().await;
}
```

This example shows how to create a simple bot that greets users and asks for their name. It uses custom models to store and retrieve user data.

### A bot that sends users a quick reply with a list of options and handles their response

```rust
use russenger::models::RussengerUser;
use russenger::prelude::*;

#[action]
async fn Main(res: Res, req: Req) {
    let payload = |value: &str| Payload::new(NextAction, Some(Data::new(value, None)));

    // QuickReply
    let quick_replies: Vec<QuickReply> = vec![
        QuickReply::new("Option 1", "", payload("Option 1")),
        QuickReply::new("Option 2", "", payload("Option 2")),
        QuickReply::new("Option 3", "", payload("Option 3")),
    ];
    let quick_reply_model = QuickReplyModel::new(&req.user, "Choose an option:", quick_replies);
    res.send(quick_reply_model).await;
}

#[action]
async fn NextAction(res: Res, req: Req) {
    let option: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &format!("You chose: {}", option))).await;
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser], &conn);
    russenger::actions![Main, NextAction];
    russenger::launch().await;
}
```

This example shows how to create a bot that sends users a quick reply with a list of options and handles their response.

## Contributing

We welcome contributions to the Russenger library! If you have an idea for a new feature or have found a bug, please open an issue on the [GitHub repository](https://github.com/russenger/russenger). If you'd like to contribute code, please fork the repository and submit a pull request.

## License

The Russenger library is licensed under the MIT License. See the [LICENSE](https://github.com/russenger/russenger/blob/main/LICENSE) file for more information.

