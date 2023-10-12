# Russenger - Facebook Messenger Webhook Handling in Rust

![image](./image.png)

Russenger is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a convenient way to construct and send various types of responses, including text messages, quick replies, generic templates, and media attachments.

## Features

Russenger provides the following features:

- **Text messages:** Send text messages to users.
- **Quick replies:** Send quick replies with buttons to users.
- **Generic templates:** Send generic templates with images, titles, and buttons to users.
- **Media attachments:** Send media attachments such as images, audio, and video to users.
- **Webhook verification:** Verify incoming webhook requests from Facebook.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
russenger = { git = "https://github.com/j03-dev/russenger", branch = "main" }
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
```

Set `.env` file on your project 
```bash
VERIFY_TOKEN=<yourverifytoken>
API=https://graph.facebook.com/v16.0/me/messages?access_token=
PAGE_ACCESS_TOKEN=<youraccesspagetoken>
DATABASE=postgres://<user>:<password>@<host>/<db_name>
```

## Usage


### Example Application
```rust
use russenger::russenger_app;
use russenger::core::action::Action;
use russenger::models::User;
use russenger::response_models::SendResponse;
use russenger::response_models::text::TextModel;

struct HelloBot {}

#[rocket::async_trait]
impl Action for HelloBot {
    async fn execute(&self, user: &str, _message: &str, user_conn: &User) {
        TextModel::new(user, "Hello World")
            .send()
            .await
            .unwrap();

        TextModel::new(user, "Enter your name: ")
            .send()
            .await
            .unwrap();

        user_conn.set_action(user, "/next_action").await;
    }
}


struct NextAction {}

#[rocket::async_trait]
impl Action for NextAction {
    async fn execute(&self, user: &str, message: &str, user_conn: &User) {
        let response = format!("Hello {message}");

        TextModel::new(user, &response)
            .send()
            .await
            .unwrap();

        user_conn.set_action(user, "/").await;
    }
}

russenger_app!(
    "/" => HelloBot {},
    "/next_action" => NextAction {}
);
```

### End-point
- GET `/webhook`
- POST `/webhook`
