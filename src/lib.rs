pub mod cli;
pub mod core;
pub mod prelude;
mod query;
pub mod response_models;
#[cfg(test)]
mod test;

pub use cli::command_handler;
pub use core::action::{Action, ACTION_REGISTRY};
pub use dotenv::dotenv;
pub use rocket::{async_trait, main};

/// The `create_action!` macro is used to create a new action.
///
/// An action is a struct that implements the `Action` trait, which has two methods: `execute` and `path`.
///
/// The `execute` method is where you define how to handle the user input. It receives two parameters: `res` and `req`. `res` is a `Res` struct that you can use to send responses to the user, and `req` is a `Req` struct that contains the user input.
///
/// The `path` method returns the name of the action as a string.
///
/// # Syntax
///
/// ```
/// create_action!(name, handler);
/// ```
///
/// * `name`: The name of the action. This will be used as the name of the struct and will be returned by the `path` method.
/// * `handler`: An expression that defines how to handle the user input. This will be used in the `execute` method.
///
/// # Examples
///
/// Creating a new action that sends a greeting message when the user input is "Hello":
///
/// ```rust
/// use russenger::prelude::*;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     let message: String = req.data.get_value();
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// });
/// ```
#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        pub struct $name;

        #[russenger::async_trait]
        impl russenger::Action for $name {
            async fn execute(&self, res: russenger::prelude::Res, req: russenger::prelude::Req) {
                ($handler)(res, req).await;
            }

            fn path(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

/// The `russenger_app!` macro is used to create the main application.
///
/// It registers all the provided actions in the `ACTION_REGISTRY` and then starts the command handler.
///
/// # Syntax
///
/// ```rust
/// use russenger::russenger_app;
///
/// russenger_app!(action1, action2, ..., actionN);
/// ```
///
/// * `action1, action2, ..., actionN`: The actions to be registered. These should be instances of structs that implement the `Action` trait.
///
/// # Examples
///
/// Creating the main application with the `Main` and `Greet` actions:
///
/// ```rust
/// use russenger::prelude::*;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     res.send(TextModel::new(&req.user, "welcome to our bot!")).await;
///     res.send(TextModel::new(&req.user, "What is your name: ")).await;
///     req.query.set_action(&req.user, Greet).await;
/// });
///
/// create_action!(Greet, |res: Res, req: Req| async move {
///     let name: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &format!("Hello : {name}"))).await;
/// });
///
/// russenger_app!(Main, Greet);
/// ```
#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        use russenger::{command_handler, Action, ACTION_REGISTRY};

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            command_handler().await;
        }
    };
}
