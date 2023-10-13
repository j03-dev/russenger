use std::env::var;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};

async fn database_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url =
        var("DATABASE").expect("check your .env file \n pls spécifie your database name");

    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Database connection failed")
}

pub struct User {
    pub connection: Pool<Postgres>,
}

impl User {
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
        let create_table_choices = "
                    create table choices(
                        choice_name varchar(20),
                        content varchar(255),
                        facebook_user_id varchar(40),
                        foreign key(facebook_user_id) references russenger_user(facebook_user_id)
                    );";
        if sqlx::query(create_table_user)
            .execute(&self.connection)
            .await
            .is_err()
        {
            return false;
        }
        if sqlx::query(create_table_choices)
            .execute(&self.connection)
            .await
            .is_err()
        {
            return false;
        }
        true
    }

    pub async fn create(&self, facebook_user_id: &str) -> bool {
        let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
        sqlx::query(sql)
            .bind(facebook_user_id)
            .bind("/")
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn set_action(&self, facebook_user_id: &str, action: &str) -> bool {
        let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
        sqlx::query(sql)
            .bind(action)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn get_action(&self, facebook_user_id: &str) -> Option<String> {
        let sql = "select action from russenger_user where facebook_user_id=$1";
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
        let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
        sqlx::query(sql)
            .bind("/")
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn set_choices(
        &self,
        facebook_user_id: &str,
        choice_name: &str,
        content: &str,
    ) -> bool {
        let sql = "insert into choices(choice_name, content, facebook_user_id) values($1, $2, $3)";
        sqlx::query(sql)
            .bind(choice_name)
            .bind(content)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn delete_all_choices(&self, facebook_user_id: &str) -> bool {
        let sql = "delete from choices where facebook_user_id=$1";
        sqlx::query(sql)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn get_choices_content(
        &self,
        facebook_user_id: &str,
        choice_name: &str,
    ) -> Option<String> {
        let sql = "select content from choices where choice_name=$1 and facebook_user_id=$2";
        match sqlx::query(sql)
            .bind(choice_name)
            .bind(facebook_user_id)
            .fetch_one(&self.connection)
            .await
        {
            Ok(row) => Some(row.get(0)),
            Err(_) => None,
        }
    }
}
