# Russenger

**Russenger** is a Rust library that makes it easy to create Facebook Messenger bots. You can check out the documentation [here](https://docs.rs/russenger).

### Projects using *Russenger*:

- [ankamantatra-bot](https://github.com/j03-dev/ankamantatra-bot)
- [gemini-bot](https://github.com/j03-dev/gemini-bot)
- [examples](https://github.com/j03-dev/russenger/tree/main/examples)

## Features

- **ORM Integration:** Built-in support for [rusql-alchemy](https://github.com/j03-dev/rusql-alchemy).
- **Facebook Verify Token Support:** Automatically handles token verification at the `/webhook` endpoint when you start the application.
- **Response Types Supported**:
  - Text Messages
  - GenericModel
  - QuickReply
  - Media
  - Actions (e.g., TypingOn/TypingOff)
  - PersistentMenu

## Getting Started

### Install `cargo-generate`

```bash
cargo install cargo-generate
```

### Create a New Project

```bash
cargo generate --git https://github.com/j03-dev/russenger_template
```

## Configuration

For more configuration, update the `.env` file in your project's root directory:

```env
PORT=8000
HOST=0.0.0.0
VERIFY_TOKEN=your_verify_token
FACEBOOK_API_VERSION=v19.0
DATABASE_URL=postgres://<username>:<password>@<hostname>/<dbname>
PAGE_ACCESS_TOKEN=your_page_access_token_from_facebook_developer
```

### Manual Setup

```toml
[dependencies]
russenger = { version = "0.3.0", features = ["postgres"] } # supports 'sqlite, postgres, mysql'
tokio = { version = "1.43.0", features = ["macros", "rt-multi-thread"] }
sqlx = "0.8.0"
```

```rust
use russenger::prelude::*;

async fn get_name(res: Res, req: Req) -> Result<()> {
    let name: String = req.data.get_value();
    res.send(TextModel::new(&req.user, format!("Hello {name}")))
        .await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    App::init().await?.attach(
        Router::new()
            .add("/", |res, req| async move {
                res.send(TextModel::new(&req.user, "Enter your name: "))
                    .await?;
                res.redirect("/get_name").await?;
                Ok(())
            })
            .add("/get_name", get_name),
    );
    Ok(())
}

```

## Contributing

Feel free to contribute by fixing typos, improving documentation, or adding new features. Any help is appreciated!
