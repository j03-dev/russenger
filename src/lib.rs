pub mod command;
pub mod core;
pub mod query;
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

        use russenger::command::execute_command;
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
    use dotenv::dotenv;

    use crate::core::action::{Action, ACTION_REGISTRY};
    use crate::core::{migrate, run_server};
    use crate::query::Query;
    use crate::register_action;
    use crate::response_models::payload::Payload;
    use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};
    use crate::response_models::text::TextModel;
    use crate::response_models::SendResponse;

    struct HelloWorld {}

    #[rocket::async_trait]
    impl Action for HelloWorld {
        async fn execute(&self, user_id: &str, _message: &str, _query: &Query) {
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
        async fn execute(&self, user_id: &str, payload: &str, query: &Query) {
            TextModel::new(user_id, &format!("Your choice is {payload}"))
                .send()
                .await
                .unwrap();
            query.reset_action(user_id).await;
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
