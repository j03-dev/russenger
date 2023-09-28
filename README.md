# Potato - Facebook Messenger Webhook Handling in Rust

Potato is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a convenient way to construct and send various types of responses, including text messages, quick replies, generic templates, and media attachments.

## Features

Potato provides the following features:

- **Text messages:** Send text messages to users.
- **Quick replies:** Send quick replies with buttons to users.
- **Generic templates:** Send generic templates with images, titles, and buttons to users.
- **Media attachments:** Send media attachments such as images, audio, and video to users.
- **Webhook verification:** Verify incoming webhook requests from Facebook.

## Installation

To include the Potato library in your Rust project, add the following line to your `Cargo.toml` file:
```toml
[dependencies]
potato = "0.2.0-rc.3"
```

## Usage

To start using Potato, import the required modules and types:

```rust
use potato::{
    Response,
    generic::{GenericElement, GenericModel, GenericButton},
    media::MediaModel,
    quick_replies::{QuickMessage, QuickReplie, QuickReplieModel},
    text::TextModel,
};
```

### Creating and Sending Responses

You can create different types of responses using the `Response` enum and send them using the `send` method. Here's an example:
Replace `<sender_id>` with the actual sender ID to which you want to send the responses.

```rust
#[tokio::main]
async fn main() {
    let sender_id = "<sender_id>".to_string();

    // Create a text message response
    let text_message = Response::TextMessage("Hello, world!");
    text_message.send(sender_id.clone()).await;

    // Create a quick reply response
    let quick_replies = vec![
        QuickReplie::new("Option 1", "https://example.com/option1.png"),
        QuickReplie::new("Option 2", "https://example.com/option2.png"),
    ];
    let quick_reply = Response::QuickReply("Choose an option:", quick_replies);
    quick_reply.send(sender_id.clone()).await;

    // Create a generic template response
    let elements = vec![
        GenericElement {
            title: "Item 1",
            image_url: "https://example.com/item1.png",
            subtitle: "Description 1",
            buttons: vec![
                GenericButton::new("Button 1"),
                GenericButton::new("Button 2"),
            ],
        },
        // Add more GenericElement entries as needed
    ];
    let generic = Response::Generic(elements);
    generic.send(sender_id.clone()).await;

    // Create a media attachment response
    let media = Response::Media("image", "https://example.com/image.png");
    media.send(sender_id.clone()).await;
}
```

### Example Application
```rust
#[macro_use]
extern crate rocket;

use std::error::Error;

use potato::potato_app;
use potato::core::Action;
use potato::models::User;
use potato::response_models::Response;

pub struct HelloBot {}

#[rocket::async_trait]
impl Action for HelloBot {
    async fn execute(&self, user: &str, _message: &str, user_conn: &User) {
        Response::TextMessage("Hello, From Potato")
            .send(user)
            .await
            .unwrap();
        user_conn.set_action(user, "/search").await;
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    potato_app!(
        "/" => HelloBot {},
        // add more acton here
    )
}
```
## Configuration

Potato relies on specific environment variables defined in a `.env` file. Ensure that you set the following variables before running your application:

- `API`: The base API URL for the Facebook Messenger platform.
- `PAGE_ACCESS_TOKEN`: The access token for your Facebook page.

Please make sure you have a valid `.env` file with these variables configured properly.

## License

This project is licensed under the [MIT License](LICENSE).
```