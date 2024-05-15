# Russenger - Facebook Messenger Webhook Handling in Rust

![Russenger Logo](./image.png)

Russenger is a Rust library designed to simplify the handling of Facebook Messenger webhook responses. It offers a
convenient way to construct and send various types of responses, including text messages, quick replies, generic
templates, and media attachments.

## Features

Russenger provides the following features:

- **Text messages:** Send text messages to users.
- **Quick replies:** Send quick replies with buttons to users.
- **Generic templates:** Send generic templates with images, titles, and buttons to users.
- **Media attachments:** Send media attachments such as images, audio, and video to users.
- **Webhook verification:** Verify incoming webhook requests from Facebook.
- **Response Models:** Russenger provides various response models to handle different types of responses. These include:
- **Data struct:** A struct to store and manipulate data with a value and an optional page.
- **Deserialization:** Deserialize the value of the data into a specified type with the `get_value` method.
- **Page retrieval:** Retrieve the page of the data with the `get_page` method.
- **Button:** A model to create and manipulate buttons in messages.
- **Getstart:** A model to handle the "Get Started" button in Messenger.
- **PersistenceMenu:** A model to handle persistent menus in Messenger.
- **Action:** A model to handle sender actions like typing indicators.

## New Features
- **Actix-web Integration**: Russenger now leverages Actix-web for improved speed and performance in handling Messenger webhook responses.

## How To Use

### Project

- [gemini-bot](https://github.com/j03-dev/gemini-bot)

### Example

Here's an example of how to use Russenger to handle different actions in a chatbot:

#### Russenger `Cargo.toml`

```toml
russenger = "0.1.4"
actix-web = "4"
```

#### Environnement `.env`

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

#[action]
async fn Main (res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!"))
        .await;

    let payload_1 = Payload::new(Option1, Some(Data::new("payload_for_option_1", None)));
    let payload_2 = Payload::new(Option2, Some(Data::new("payload_for_option_2", None)));

    let replies = vec![
        QuickReply::new("Option1", "", payload_1),
        QuickReply::new("Option2", "", payload_2),
    ];
    res.send(QuickReplyModel::new(
        &req.user,
        "Choose an option:",
        replies,
    ))
    .await;
}

#[action]
async fn Option1 (res: Res, req: Req) {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 1 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;
}

#[action]
async fn Option2 (res: Res, req: Req) {
    let value: String = req.data.get_value();
    let message = format!("You selected Option 2 with payload: {}", value);
    res.send(TextModel::new(&req.user, &message)).await;

    let generic_elements = vec![GenericElement::new(
        "Option 2",
        "https://example.com/option2.jpg", // use existe url
        "Option 2 description",
        vec![Button::Postback {
            title: "Choose Option 2".to_string(),
            payload: Payload::new(Main, None),
        }],
    )];

    res.send(GenericModel::new(
        &req.user,
        generic_elements,
        req.data.get_page(),
    ))
    .await;
}

russenger_app!(Main, Option1, Option2);
```

##### Who to get User Input

```rust
use russenger::prelude::*;

#[action]
async fn Main (res: Res, req: Req){
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

russenger_app!(Main, GetUsername);
```

##### How to send file from static

```rust
use russenger::prelude::*;

#[action]
async fn Main (res: Res, req: Req){
    res.send(TextModel::new(&req.user, "Main, I'm your chatbot!"))
        .await;

    // Send Image File from static file
    // Add image file, on static dir
    res.send(MediaModel::new(
        &req.user,
        "image",
        &format!("{host}/image.png", host = req.host),
    ))
    .await;
}
russenger_app!(Main);
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
