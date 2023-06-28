use std::env::var;

use dotenv::dotenv;
use sqlx::{Pool, Row, Sqlite, SqlitePool};

async fn database_connection() -> Pool<Sqlite> {
    dotenv().ok();
    let database_url =
        var("DATABASE").expect("check your .env file \n pls spécifie your database name");
    SqlitePool::connect(&database_url)
        .await
        .expect("failed to open database")
}

pub struct User {
    pub connection: Pool<Sqlite>,
}

impl User {
    pub async fn new() -> Self {
        Self {
            connection: database_connection().await,
        }
    }

    pub async fn create(&self, facebook_user_id: &str) -> bool {
        let sql = "insert into user (facebook_user_id, action) values (?, ?)";
        sqlx::query(sql)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn set_action(&self, facebook_user_id: &str, action: &str) -> bool {
        let sql = "update user set action=? where facebook_user_id=?";
        sqlx::query(sql)
            .bind(action)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn get_action(&self, facebook_user_id: &str) -> Option<String> {
        let sql = "select action from user where facebook_user_id=?";
        match sqlx::query(sql)
            .bind(facebook_user_id)
            .fetch_one(&self.connection)
            .await
        {
            Ok(row) => Some(row.get(0)),
            Err(_) => None,
        }
    }

    pub async fn reset_action(&self, facebook_user_id: &str) -> bool {
        let sql = "update set action=? where faceboook_user_id=?";
        sqlx::query(sql)
            .bind("/")
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }
}
