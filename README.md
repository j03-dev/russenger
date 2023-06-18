# Potato

Potato is a Rust library for handling Facebook Messenger webhook responses. It provides a convenient way to construct and send various types of responses, such as text messages, quick replies, generic templates, and media attachments.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
potato = "0.1.0"
```

## Usage

To use the Potato library, import the necessary modules and types:

```rust
use potato::{
    Response,
    generic::{GenericElement, GenericMessage, GenericModel, GenericButton},
    media::{MediaAttachment, MediaModel},
    quick_replies::{QuickMessage, QuickReplie, QuickReplieModel},
    text::TextModel,
};
```

### Creating and Sending Responses

You can create different types of responses using the `Response` enum and send them using the `send` method:

```rust
#[tokio::main]
async fn main() {
    let sender_id = "<sender_id>".to_string();

    // Create a text message response
    let text_message = Response::TextMessage("Hello, world!".to_string());
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
        GenericElement {
            title: "Item 2",
            image_url: "https://example.com/item2.png",
            subtitle: "Description 2",
            buttons: vec![
                GenericButton::new("Button 3"),
                GenericButton::new("Button 4"),
            ],
        },
    ];
    let generic = Response::Generic(elements);
    generic.send(sender_id.clone()).await;

    // Create a media attachment response
    let media = Response::Media("image", "https://example.com/image.png");
    media.send(sender_id.clone()).await;
}
```

Make sure to replace `<sender_id>` with the actual sender ID you want to send the responses to.

## Configuration

Potato requires the following environment variables to be set in a `.env` file:

- `API`: The base API URL for the Facebook Messenger platform.
- `PAGE_ACCESS_TOKEN`: The access token for your Facebook page.

Please ensure that you have a valid `.env` file with these variables before running your application.

## License

This project is licensed under the [MIT License](LICENSE).
```