use std::str::FromStr;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, State};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::ACTION_REGISTRY;
use app_state::AppState;
use data::Data;
use deserializers::MessageDeserializer;
use facebook_request::FacebookRequest;
use request::Req;
use response::Res;

use crate::payload::Payload;
use crate::query::Query;
use crate::text::TextModel;

pub mod action;
pub mod data;
pub mod request;
pub mod response;

mod app_state;
mod deserializers;
mod facebook_request;

#[catch(404)]
fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
fn server_panic() -> &'static str {
    "Server panic: 500"
}

// Define the webhook_verify function
// This function is an asynchronous function that handles GET requests to the "/webhook" endpoint
// It takes a FacebookRequest as an argument and returns a String
#[get("/webhook")]
async fn webhook_verify(request: FacebookRequest) -> String {
    // The function simply returns the string contained in the FacebookRequest
    // This is the "hub.challenge" parameter that Facebook sends when verifying the webhook
    request.0
}

// This function executes the payload.
// It takes a user, a URI, and a query as parameters.
async fn execute_payload(user: &str, uri: &str, query: &Query) {
    match Payload::from_uri_string(uri) {
        Ok(payload) => {
            // If the payload is valid, we get the corresponding action from the registry
            if let Some(action) = ACTION_REGISTRY.lock().await.get(payload.get_path()) {
                // We create a new request and execute the action
                let data = Data::from_string(payload.get_data_to_string());
                let request = Req::new(user, query.clone(), data);
                action.execute(Res, request).await;
            }
        }
        Err(err) => {
            // If the payload is not valid, we send an error message
            Res.send(TextModel::new(user, &err)).await;
        }
    }
}

// This function is the core of the webhook.
// It processes incoming data and updates the state accordingly.
#[post("/webhook", format = "json", data = "<data>")]
async fn webhook_core(
    data: Json<MessageDeserializer>,
    app_state: &State<AppState>,
) -> &'static str {
    // Extract the query from the application state
    let query = &app_state.query;

    // Extract the sender from the incoming data
    let user = data.get_sender();

    // Create user if user is not yet created
    query.create(user).await;

    if app_state.action_lock.lock(user).await {
        // If there is a message in the incoming data
        if let Some(message) = data.get_message() {
            // Get the action path from the query
            let action_path = query.get_action(user).await.unwrap_or("lock".to_string());

            // If there is a quick reply in the message
            if let Some(quick_reply) = message.get_quick_reply() {
                let uri_payload = quick_reply.get_payload();
                execute_payload(user, uri_payload, query).await;
            }
            if let Some(action) = ACTION_REGISTRY.lock().await.get(action_path.as_str()) {
                let request = Req::new(user, query.clone(), Data::new(message.get_text(), None));
                action.execute(Res, request).await;
            }
        }
        // If there is a postback in the incoming data
        else if let Some(postback) = data.get_postback() {
            // Get the payload from the postback
            let uri = postback.get_payload();
            // Execute the payload
            execute_payload(user, uri, query).await;
        }
    }

    app_state.action_lock.unlock(user).await;

    // Return "Ok" to indicate that the function has executed successfully
    "Ok"
}

// This function runs the server.
pub async fn run_server() {
    if !ACTION_REGISTRY.lock().await.contains_key("Main") {
        panic!("The ACTION_REGISTRY should contain an action with path 'Main' implementing the Action trait.");
    }
    let allowed_origins = AllowedOrigins::some_regex(&["graph.facebook.com"]);

    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let cors = (CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    })
    .to_cors()
    .expect("Failed to create CORS: Something went wrong with CORS");

    // Build and launch the server
    rocket::build()
        .attach(cors)
        .manage(AppState::init().await)
        .mount("/", routes![webhook_verify, webhook_core])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![page_not_found, server_panic])
        .launch()
        .await
        .expect("Failed to run Rocket server");
}

// This function handles the database migration.
pub async fn migrate() {
    // Create a new query instance
    let query = Query::new().await;

    // Print a success message if the connection is successful
    println!("Connection successful!");

    // Perform the migration and print a success or failure message
    let migration_result = match query.migrate().await {
        true => "Migration successful!",
        false => "Migration failed",
    };

    println!("{}", migration_result);
}
