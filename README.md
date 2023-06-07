Here's a documentation for the provided code:

# Potato Messenger Hooks

This documentation provides an overview and usage instructions for the Potato Messenger Hooks module.
The module is responsible for handling incoming Facebook webhook requests, processing messages, 
and sending appropriate responses using the Potato Messenger API module.

## Usage

To use the Potato Messenger Hooks module in your project, follow these steps:

1. Import the necessary modules and types into your code:

```rust
use potato::{
    hooks::{
        messages::FacebookMessage,
        response_models::{quick_replies::QuickReplie, Response, send},
        FacebookRequest,
    },
    models,
};
use rocket::serde::json::Json;
```

2. Implement the webhook verification endpoint:

```rust
#[get("/")]
pub fn hooks_verify(request: FacebookRequest) -> String {
    request.0
}
```

The `hooks_verify` function handles the GET request at the specified endpoint ("/").
It extracts the verification token from the `FacebookRequest` struct and returns it as a string to verify the webhook.

3. Implement the webhook core endpoint:

```rust
#[post("/", format = "json", data = "<facebook_message>")]
pub async fn hooks_core(facebook_message: Json<FacebookMessage>) -> &'static str {
    // ...
}
```

The `hooks_core` function handles the POST request at the specified endpoint ("/").
It receives a JSON payload containing the `FacebookMessage` data from the webhook.
The function processes the message, generates an appropriate response, and sends it using the `send` function from the Potato Messenger API module.

4. Within the `hooks_core` function, you can access the received message and sender ID:

```rust
let message = facebook_message.get_message();
let facebook_user_id = facebook_message.get_sender();
```

5. Implement your business logic based on the received message and sender ID. Here's an example:

```rust
let user = models::User::default();
user.create(&facebook_user_id);

let response: Response = match user
    .get_action(&facebook_user_id)
    .expect("maybe there is no user in base")
    .as_str()
{
    "/" => {
        let red = QuickReplie::new("red", "red.png");
        let blue = QuickReplie::new("blue", "blue.png");
        user.set_action(&facebook_user_id, "/pick_color");
        Response::QuickReply("pick color", vec![red, blue])
    }
    "/pick_color" => {
        user.reset_action(&facebook_user_id);
        let answer = format!("the color is {message}");
        Response::TextMessage(answer)
    }
    _ => {
        user.reset_action(&facebook_user_id);
        Response::TextMessage("action is reset".to_string())
    }
};
```

In this example, the code checks the user's current action and generates a corresponding response.
If the action is "/", it sets up quick reply options for color selection. If the action is "/pick_color",
it generates a response with the chosen color. Otherwise, it resets the action and sends a default response.

6. Send the response using the `send` function:

```rust
send(facebook_user_id, response).await;
```

The `send` function from the Potato Messenger API module is used to send the constructed response to the specified user.

7. Return a response to acknowledge the webhook request:

```rust
"ok"
```

This string is returned to the Facebook webhook to confirm the successful processing of the webhook request.

## Conclusion

The Potato Messenger Hooks module provides functionality for handling Facebook webhook requests, 
processing messages, and sending responses using the Potato Messenger API module. 
This documentation outlines the module's structure and usage instructions. Use this documentation as
