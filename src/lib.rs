pub mod cli;
pub mod core;
pub mod hooks;
pub mod models;
pub mod response_models;

#[macro_export]
macro_rules! register_action {
    ($path:expr, $action:expr) => {
        ACTION_REGISTRY
            .lock()
            .await
            .insert($path, Box::new($action));
    };
}

#[macro_export]
macro_rules! russenger_app {
    ($($path:expr => $action:expr),* $(,)?) => {
        #[macro_use]
        extern crate rocket;

        use russenger::cli::execute_command;
        use russenger::core::action::ACTION_REGISTRY;
        use russenger::register_action;

        #[rocket::main]
        async fn main(){
            $(register_action!($path, $action);)*
            execute_command().await;
        }
    };
}

#[cfg(test)]
mod test {
    use crate::core::action::{Action, ACTION_REGISTRY};
    use crate::core::{migrate, run_server};
    use crate::models::User;
    use crate::register_action;
    use crate::response_models::payload::Payload;
    use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};
    use crate::response_models::text::TextModel;
    use crate::response_models::SendResponse;
    use dotenv::dotenv;

    struct HelloWorld {}

    #[rocket::async_trait]
    impl Action for HelloWorld {
        async fn execute(&self, user_id: &str, _message: &str, _user_conn: &User) {
            TextModel::new(user_id, "Hello World!")
                .send()
                .await
                .unwrap();

            QuickReplieModel::new(
                user_id,
                "Choose one",
                &vec![
                    QuickReplie::new(
                        "red",
                        "",
                        Payload::new("/next_action", Some("RED".to_string())),
                    ),
                    QuickReplie::new(
                        "blue",
                        "",
                        Payload::new("/next_action", Some("BLUE".to_string())),
                    ),
                    QuickReplie::new("Retry", "", Payload::new("/", None)),
                ],
            )
            .send()
            .await
            .unwrap();
        }
    }

    struct NextAction {}

    #[rocket::async_trait]
    impl Action for NextAction {
        async fn execute(&self, user_id: &str, message: &str, user_conn: &User) {
            TextModel::new(user_id, &format!("Your choice is {message}"))
                .send()
                .await
                .unwrap();
            user_conn.reset_action(user_id).await;
        }
    }

    #[rocket::async_test]
    async fn test_migration() {
        dotenv().ok();
        migrate().await;
    }

    #[rocket::async_test]
    async fn test_run() {
        dotenv().ok();
        register_action!("/", HelloWorld {});
        register_action!("/next_action", NextAction {});
        run_server().await;
    }
}
