# Russenger - Facebook Messenger Webhook Handling in Rust

![Russenger Logo](./image.png)

Russenger is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a convenient way to construct and send various types of responses, including text messages, quick replies, generic templates, and media attachments.

## Features

Russenger provides the following features:

- **Text messages:** Send text messages to users.
- **Quick replies:** Send quick replies with buttons to users.
- **Generic templates:** Send generic templates with images, titles, and buttons to users.
- **Media attachments:** Send media attachments such as images, audio, and video to users.
- **Webhook verification:** Verify incoming webhook requests from Facebook.

## Installation

To use Russenger in your Rust project, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
russenger = { git = "https://github.com/j03-dev/russenger", branch = "main" }
rocket = { version = "0.5.0-rc.4", features = ["json"] }
```

Ensure that you have set up your `.env` file within your project directory. The file should contain the following configurations:

```env
VERIFY_TOKEN=<your-verify-token>
API=https://graph.facebook.com/v16.0/me/messages?access-token=
PAGE_ACCESS_TOKEN=<your-page-access-token>
DATABASE=postgres://<user>:<password>@<host>/<db_name>
```

## Usage

### Example Application

The following example demonstrates the usage of Russenger for creating a chatbot in Rust. It includes actions named `Hello`, `Option1`, and `Option2`, along with a user scenario:

```rust
use russenger::{Action, Data, Req, Res};
use russenger::response_models::generic::{GenericButton, GenericElement, GenericModel};
use russenger::response_models::payload::Payload;
use russenger::response_models::quick_replies::{QuickReplie, QuickReplieModel};
use russenger::response_models::text::TextModel;
use russenger::{create_action, russenger_app};


create_action!(Hello, |res: Res, req: Req<'l>| async move {
    // Welcome message
    res.send(TextModel::new(req.user, "Hello, I'm your chatbot!"))
        .await
        .unwrap();

    // Example with Quick Replies
    let quick_replies = vec![
        QuickReplie::new(
            "Option 1",
            "",
            Payload::new("/option1", Some(Data::new("payload_for_option1", None))),
        ),
        QuickReplie::new(
            "Option 2",
            "",
            Payload::new("/option2", Some(Data::new("payload_for_option2", None))),
        ),
    ];

    res.send(QuickReplieModel::new(
        req.user,
        "Choose an option:",
        &quick_replies,
    ))
    .await
    .unwrap();
});

// For Option1
create_action!(Option1, |res: Res, req: Req<'l>| async move {
    // Handle Option 1 with a TextModel
    res.send(TextModel::new(
        req.user,
        &format!("You selected Option 1 with payload: {}", req.data.get_value::<String>()),
    ))
    .await
    .unwrap();
});

// For Option2
create_action!(Option2, |res: Res, req: Req<'l>| async move {
    // Handle Option 2 with a TextModel
    res.send(TextModel::new(
        req.user,
        &format!("You selected Option 2 with payload: {}", req.data.get_value::<String>()),
    ))
    .await
    .unwrap();

    // Handle Option 2 with a Generic Template
    let generic_elements = vec![GenericElement {
        title: "Option 2",
        image_url: "https://example.com/option2.jpg",
        subtitle: "Option 2 description",
        buttons: vec![GenericButton::new(
            "Choose Option 2",
            Payload::new("/hello", None),
        )],
    }];

    res.send(GenericModel::new(req.user, &generic_elements))
        .await
        .unwrap();
});

russenger_app!(
    "/" => Hello {},
    "/option1" => Option1 {},
    "/option2" => Option2 {}
);
```

### Endpoints

- **GET `/webhook`:** Verify your chatbot with Facebook Messenger. Facebook will send a challenge, and your bot must respond correctly for verification.

- **POST `/webhook`:** This is where Facebook Messenger sends messages from users. Handle incoming messages and respond accordingly here.

### License

Russenger is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

For more information, visit the [GitHub repository](https://github.com/j03-dev/russenger).

If you have any questions or need assistance, feel free to open an issue or contact the project maintainers.