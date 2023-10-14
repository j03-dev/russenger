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
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
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
use russenger::russenger_app;
use russenger::core::action::Action;
use russenger::models::User;
use russenger::response_models::text::TextModel;
use russenger::response_models::quick_replies::{QuickReplie, QuickReplieModel};
use russenger::response_models::generic::{GenericModel, GenericElement, GenericButton, Payload};

struct Hello {}
struct Option1 {}
struct Option2 {}

#[rocket::async_trait]
impl Action for Hello {
    async fn execute(&self, user: &str, _message: &str, _user_conn: &User) {
        // Welcome message
        TextModel::new(user, "Hello, I'm your chatbot!")
            .send()
            .await
            .unwrap();

        // Example with Quick Replies
        let quick_replies = vec![
            QuickReplie::new("Option 1", Payload::new("/option1", Some("payload_for_option1".to_string()))),
            QuickReplie::new("Option 2", Payload::new("/option2", Some("payload_for_option2".to_string()))),
        ];

        QuickReplieModel::new(user, "Choose an option:", &quick_replies)
            .send()
            .await
            .unwrap();
    }
}

// For Option1
#[rocket::async_trait]
impl Action for Option1 {
    async fn execute(&self, user: &str, message: &str, _user_conn: &User) {
        // Handle Option 1 with a TextModel
        TextModel::new(user, &format!("You selected Option 1 with payload: {}", message))
            .send()
            .await
            .unwrap();
    }
}

// For Option2
#[rocket::async_trait]
impl Action for Option2 {
    async fn execute(&self, user: &str, message: &str, _user_conn: &User) {
        // Handle Option 2 with a TextModel
        TextModel::new(user, &format!("You selected Option 2 with payload: {}", message))
            .send()
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

        GenericModel::new(user, &generic_elements)
            .send()
            .await
            .unwrap();
    }
}

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