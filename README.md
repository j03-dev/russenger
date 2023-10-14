
---

# Russenger - A Rust Chatbot LIB

Russenger is a Rust chatbot lib that simplifies the creation of interactive chatbots for various messaging platforms. Whether you want to build a chatbot for customer support, automation, or entertainment, Russenger provides the tools to get you started quickly.

## Table of Contents

1. [Getting Started](#getting-started)
   - [Prerequisites](#prerequisites)
   - [Installation](#installation)
2. [Usage](#usage)
   - [Creating Actions](#creating-actions)
   - [Defining Routes](#defining-routes)
3. [Examples](#examples)
4. [Documentation](#documentation)
5. [Contributing](#contributing)
6. [License](#license)

## Getting Started

### Prerequisites

Before using Russenger, you'll need to have the following installed on your system:

- [Rust](https://www.rust-lang.org/): Russenger is a Rust library, so ensure you have Rust installed.

### Installation

You can include Russenger in your Rust project by adding it to your `Cargo.toml`:

```toml
[dependencies]
russenger = "0.1"
```

## Usage

Russenger allows you to define actions and routes to create interactive chatbots. Here's how to get started:

### Creating Actions

Actions in Russenger represent the behavior of your chatbot. You can create custom actions by implementing the `Action` trait. For example:

```rust
use russenger::core::action::Action;
use russenger::response_models::text::TextModel;
use russenger::response_models::quick_replies::QuickReplyModel;
use russenger::response_models::generic::{GenericModel, GenericElement, GenericButton};

pub struct Greet;

#[rocket::async_trait]
impl Action for Greet {
    async fn execute(&self, user: &str, _message: &str, _user_conn: &User) {
        let greeting = "Hello, I'm your chatbot!";
        TextModel::new(user, greeting).send().await.unwrap();
        
        // Example Quick Replies
        let quick_replies = vec![
            QuickReplie::new("Option 1", "", Payload::new("/option1", None)),
            QuickReplie::new("Option 2", "", Payload::new("/option2", None)),
        ];
        
        QuickReplieModel::new(user, "Choose an option:", &quick_replies)
            .send()
            .await
            .unwrap();
    }
}
```

### Defining Routes

Routes in Russenger map specific paths to actions. You can define your routes using the `russenger_app!` macro. For example:

```rust
russenger_app!(
    "/" => Greet,
    "/option1" => Option1Action,
    "/option2" => Option2Action,
    // Add more routes here...
);
```

In the example above, the `/` path is mapped to the `Greet` action, which greets the user and provides Quick Replies for further interaction.

## Examples

Here are some examples to illustrate how Russenger can be used to create chatbot interactions with Quick Replies and the Generic Model:

#### Example 1: Greeting the User and Providing Quick Replies

```rust
use russenger::core::action::Action;
use russenger::response_models::text::TextModel;
use russenger::response_models::quick_replies::QuickReplyModel;
use russenger::response_models::quick_replies::QuickReplie;

pub struct Greet;

#[rocket::async_trait]
impl Action for Greet {
    async fn execute(&self, user: &str, _message: &str, _user_conn: &User) {
        let greeting = "Hello, I'm your chatbot!";
        TextModel::new(user, greeting).send().await.unwrap();
        
        // Example Quick Replies
        let quick_replies = vec![
            QuickReplie::new("Option 1", "", Payload::new("/option1", None)),
            QuickReplie::new("Option 2", "", Payload::new("/option2", None)),
        ];
        
        QuickReplieModel::new(user, "Choose an option:", &quick_replies)
            .send()
            .await
            .unwrap();
    }
}
```

In this example, when a user accesses the root path `/`, the chatbot greets the user with a welcome message and provides Quick Replies for further interaction.

#### Example 2: Handling User's Choice

You can create actions to handle user input based on their choice from Quick Replies.

```rust
use russenger::core::action::Action;
use russenger::response_models::text::TextModel;

pub struct Option1Action;

#[rocket::async_trait]
impl Action for Option1Action {
    async fn execute(&self, user: &str, _message: &str, _user_conn: &User) {
        let response = "You chose Option 1. What would you like to do next?";
        TextModel::new(user, response).send().await.unwrap();
    }
}
```

This example demonstrates an action that handles the user's choice from the Quick Replies provided earlier.

## Contributing

We welcome contributions from the community. If you have suggestions, bug reports, or feature requests, please create an issue in this repository. If you'd like to contribute code, feel free to open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
