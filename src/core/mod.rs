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

use crate::query::Query;
use crate::response_models::payload::Payload;
use crate::response_models::text::TextModel;

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

#[get("/webhook")]
async fn webhook_verify(request: FacebookRequest) -> String {
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
#[post("/webhook", format = "json", data = "<incoming_data>")]
async fn webhook_core(
    incoming_data: Json<MessageDeserializer>, // The incoming data from the webhook
    app_state: &State<AppState>,              // The current state of the application
) -> &'static str {
    // Extract the query from the application state
    let query = &app_state.query;

    // Extract the sender from the incoming data
    let user = incoming_data.get_sender();

    // If there is a message in the incoming data
    if let Some(message) = incoming_data.get_message() {
        // Get the action path from the query
        let action_path = query.get_action(user).await.unwrap_or("lock".to_string());

        // If there is a quick reply in the message
        if let Some(quick_reply) = message.get_quick_reply() {
            // Get the payload from the quick reply
            let uri_payload = quick_reply.get_payload();

            // Execute the payload
            execute_payload(user, uri_payload, query).await;
        }
        // If the action path is not "lock"
        else if action_path.ne("lock") {
            // If there is an action corresponding to the action path
            if let Some(action) = ACTION_REGISTRY.lock().await.get(action_path.as_str()) {
                // Set the action in the query to "lock"
                query.set_action(user, "lock").await;

                // Create a new request
                let request = Req::new(user, query.clone(), Data::new(message.get_text(), None));

                // Execute the action
                action.execute(Res, request).await;
            }
        } else {
            // Reset the action in the query
            query.reset_action(user).await;
        }
    }
    // If there is a postback in the incoming data
    else if let Some(postback) = incoming_data.get_postback() {
        // Get the payload from the postback
        let uri = postback.get_payload();

        // Execute the payload
        execute_payload(user, uri, query).await;
    }

    // Return "Ok" to indicate that the function has executed successfully
    "Ok"
}

// This function runs the server.
pub async fn run_server() {
    // Define the allowed origins for CORS
    let allowed_origins = AllowedOrigins::some_regex(&["graph.facebook.com"]);

    // Define the allowed methods for CORS
    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    // Set up CORS options
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
        .attach(cors) // Attach CORS options
        .manage(AppState::init().await) // Manage application state
        .mount("/", routes![webhook_verify, webhook_core]) // Mount routes
        .mount("/static", FileServer::from("static")) // Serve static files
        .register("/", catchers![page_not_found, server_panic]) // Register error catchers
        .launch()
        .await // Launch the server
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
