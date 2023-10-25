pub mod command;
pub mod core;
pub mod query;
pub mod response_models;

#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        pub struct $name;

        #[rocket::async_trait]
        impl Action for $name {
            async fn execute<'l>(&self, res: Res, req: Req<'l>) {
                ($handler)(res, req).await;
            }
        }
    };
}

#[macro_export]
macro_rules! russenger_app {
    ($($path:expr => $action:expr),* $(,)?) => {
        #[macro_use]
        extern crate rocket;

        use russenger::command::execute_command;
        use russenger::core::action::ACTION_REGISTRY;

        #[rocket::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($path, Box::new($action));)*
            execute_command().await;
        }
    };
}

#[cfg(test)]
mod test {
    use crate::core::action::Action;
    use crate::core::request::Req;
    use crate::core::response::Res;
    use crate::core::{migrate, run_server};
    use crate::response_models::payload::Data;
    use crate::response_models::payload::Payload;
    use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};
    use crate::response_models::text::TextModel;
    use dotenv::dotenv;

    #[rocket::async_test]
    async fn test_migration() {
        dotenv().ok();
        migrate().await;
    }

    create_action!(NextAction, |res: Res, req: Req<'l>| async move {
        if let Some(data) = Data::from_str(req.data) {
            res.send(TextModel::new(
                req.user,
                &format!("Your choice is {}", data.get_value::<String>()),
            ))
            .await
            .unwrap();
            req.query.reset_action(req.user).await;
        } else {
            res.send(TextModel::new(req.user, "There is no data"))
                .await
                .unwrap();
        }
    });

    create_action!(Hello, |res: Res, req: Req<'l>| async move {
        res.send(TextModel::new(req.user, "Hello World!"))
            .await
            .unwrap();
        res.send(QuickReplieModel::new(
            req.user,
            "Choose one",
            &vec![
                QuickReplie::new(
                    "red",
                    "",
                    Payload::new("/next_action", Some(Data::new("BLUE", None))),
                ),
                QuickReplie::new(
                    "blue",
                    "",
                    Payload::new("/next_action", Some(Data::new("RED", None))),
                ),
                QuickReplie::new("Retry", "", Payload::new("/", None)),
            ],
        ))
        .await
        .unwrap();
    });

    #[rocket::async_test]
    async fn test_run() {
        dotenv().ok();
        run_server().await;
    }
}
