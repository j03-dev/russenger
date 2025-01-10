# Russenger - Facebook Messenger Webhook Handling in Rust

Welcome to Russenger, a Rust library designed to simplify the handling of Facebook Messenger webhook responses. Russenger offers a convenient way to construct and send various types of responses, including text messages, quick replies, generic templates, and media attachments.

## Features

- **Custom Models**: Developers can now use their own models with the Russenger library. This is made possible by the integration with [rusql-alchemy](https:://github.com/russenger/rusql-alchemy), an ORM for sqlx. This means that models are defined in Rust code, eliminating the need to write SQL queries.
- **Easy to Use**: The Russenger library is designed to be easy to use. It provides a set of modules and macros that abstract away the complexities of building a bot, allowing you to focus on the logic of your application.
- **Flexible**: The Russenger library is flexible and can be used to build a wide variety of bots. It supports text-based conversations, quick replies, and custom actions.

## Getting Started

To get started with the Russenger library, you'll need to install it as a dependency in your Rust project. You can do this by adding the following line to your `Cargo.toml` file:

```toml
[dependencies]
russenger = { version = "0.3.4-rc", features = ["postgres"] } # features 'sqlite, postgres, mysql'
actix-web = "4"
sqlx = "^0.8.0"
```

env

```
PORT=8000
HOST=0.0.0.0
VERIFY_TOKEN=your_verify_token
FACEBOOK_API_VERSION=v19.0
DATABASE_URL=postgres://<username>:<password>@<hostname>/<dbname>
PAGE_ACCESS_TOKEN=your_page_acces_token_from_facebook_developer
```

Once you've installed the library, you can start building your bot! Check out the [documentation](https://docs.rs/russenger) for more information on how to use the library.

## Creating a New Project

To create a new project using the Russenger library, you can use the `cargo-generate` tool. Here are the steps:

1. Install `cargo-generate`:

```bash
cargo install cargo-generate
```

1. Generate a new project:

```bash
cargo generate --git https://github.com/j03-dev/russenger_template
```

## Examples

Here are some examples of what you can build with the Russenger library:

### A simple bot that greets users and asks for their name

```rust
use russenger::{models::RussengerUser, prelude::*, App};

#[derive(FromRow, Clone, Model)]
pub struct Register {
    #[model(primary_key = true, auto = true)]
    pub id: Integer,

    #[model(foreign_key = "RussengerUser.facebook_user_id", unique = true)]
    pub user_id: String,

    #[model(size = 30, unique = true)]
    pub username: String,
}

#[action]
async fn index(res: Res, req: Req) -> Result<()> {
    res.send(TextModel::new(&req.user, "Hello!")).await?;
    if let Some(user_register) =
        Register::get(kwargs!(user_id == req.user), &req.query.conn).await
    {
        res.send(TextModel::new(
            &req.user,
            &format!("Hello {}", user_register.username),
        ))
        .await?;
    } else {
        res.send(TextModel::new(&req.user, "What is your name: "))
            .await?;
        res.redirect("/signup").await?;
        return Ok(());
    }
    get_user_input(res, req).await?;

    Ok(())
}

#[action]
async fn signup(res: Res, req: Req) -> Result<()> {
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
    index(res, req).await?;

    Ok(())
}

#[action]
async fn get_user_input(res: Res, req: Req) -> Result<()> {
    let payload = |value: &str| Payload::new("/print", Some(Data::new(value)));

    // QuickReply
    let quick_replies: Vec<QuickReply> = vec![
        QuickReply::new("blue", None, payload("blue")),
        QuickReply::new("red", None, payload("red")),
    ];
    let quick_reply_model = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
    res.send(quick_reply_model).await?;

    Ok(())
}

#[action]
async fn print_color(res: Res, req: Req) -> Result<()> {
    let color: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &color)).await?;
    index(res, req).await?; // go back to index action

    Ok(())
}

#[russenger::main]
async fn main() -> error::Result<()> {
    let database = Database::new().await?;
    let conn = database.conn;

    migrate!([RussengerUser, Register], &conn);

    let mut app = App::init().await?;
    app.add("/", index).await;
    app.add("/signup", signup).await;
    app.add("/get_user_input", get_user_input).await;
    app.add("/print", print_color).await;

    russenger::launch(app).await?;

    Ok(())
}
```

This example shows how to create a simple bot that greets users and asks for their name. It uses custom models to store and retrieve user data.

### A bot that sends users a quick reply with a list of options and handles their response

```rust
use russenger::models::RussengerUser;
use russenger::{prelude::*, App};

#[action]
async fn index(res: Res, req: Req) {
    let payload = |value: &str| Payload::new("/next", Some(Data::new(value)));

    // QuickReply
    let quick_replies: Vec<QuickReply> = vec![
        QuickReply::new("Option 1", None, payload("Option 1")),
        QuickReply::new("Option 2", None, payload("Option 2")),
        QuickReply::new("Option 3", None, payload("Option 3")),
    ];
    let quick_reply_model = QuickReplyModel::new(&req.user, "Choose an option:", quick_replies);
    res.send(quick_reply_model).await?;

    Ok(())
}

#[action]
async fn next(res: Res, req: Req) {
    let option: String = req.data.get_value();
    res.send(TextModel::new(&req.user, &format!("You chose: {}", option)))
        .await?;
    Ok(())
}

#[russenger::main]
async fn main() -> error::Result<()> {
    let database = Database::new().await?;
    let conn = database.conn;
    migrate!([RussengerUser], &conn);

    let mut app = App::init().await?;
    app.add("/", index).await;
    app.add("/next", next).await;

    russenger::launch(app).await?;
    Ok(())
}
```

This example shows how to create a bot that sends users a quick reply with a list of options and handles their response.

## Contributing

We welcome contributions to the Russenger library! If you have an idea for a new feature or have found a bug, please open an issue on the [GitHub repository](https://github.com/russenger/russenger). If you'd like to contribute code, please fork the repository and submit a pull request.

## License

The Russenger library is licensed under the MIT License. See the [LICENSE](https://github.com/russenger/russenger/blob/main/LICENSE) file for more information.
