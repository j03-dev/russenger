use rusql_alchemy::prelude::*;

#[derive(FromRow, Model, Clone, Default)]
pub struct RussengerUser {
    #[model(primary_key = true, null = false)]
    pub facebook_user_id: String,
    #[model(default = "Main")]
    pub action: String,
}