use std::env::var;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

async fn database_connection() -> Pool<Postgres> {
    let database_url =
        var("DATABASE").expect("check your .env file \n pls spécifie your database name");

    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Database connection failed")
}

pub struct Query {
    pub connection: Pool<Postgres>,
}

impl Query {
    pub async fn new() -> Self {
        Self {
            connection: database_connection().await,
        }
    }

    pub async fn migrate(&self) -> bool {
        let create_table_user = "
                    create table russenger_user ( 
                        facebook_user_id varchar(40) primary key unique,
                        action varchar(20)
                    );";
        sqlx::query(create_table_user)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn create(&self, user_id: &str) -> bool {
        let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
        sqlx::query(sql)
            .bind(user_id)
            .bind("/")
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn set_action(&self, user_id: &str, action: &str) -> bool {
        let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
        sqlx::query(sql)
            .bind(action)
            .bind(user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        let sql = "select action from russenger_user where facebook_user_id=$1";
        match sqlx::query(sql)
            .bind(user_id)
            .fetch_one(&self.connection)
            .await
        {
            Ok(row) => row.get(0),
            Err(_) => None,
        }
    }

    pub async fn reset_action(&self, user_id: &str) -> bool {
        let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
        sqlx::query(sql)
            .bind("/")
            .bind(user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }
}
