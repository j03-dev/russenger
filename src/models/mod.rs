use std::env::var;

use dotenv::dotenv;
use sqlx::{Pool, Row, Sqlite, SqlitePool};

pub async fn database_connection() -> Pool<Sqlite> {
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

    pub async fn migrate(&self) {
        let model_sql = "
                    create table user (
                        facebook_user_id varchar(40) primary key unique,
                        action varchar(20)
                    );
                    create table choices(
                        choice_name varchar(20),
                        content varchar(255),
                        facebook_user_id varchar(40),
                        foreign key(facebook_user_id) references user(facebook_user_id)
                    )";
        if sqlx::query(model_sql)
            .execute(&self.connection)
            .await
            .is_ok()
        {
            println!("migrate succes");
        } else {
            println!("failde to migrate");
        }
    }

    pub async fn create(&self, facebook_user_id: &str) -> bool {
        let sql = "insert into user (facebook_user_id, action) values (?, ?)";
        sqlx::query(sql)
            .bind(facebook_user_id)
            .bind("/")
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
        let sql = "update user set action=? where faceboook_user_id=?";
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
        let sql = "insert into choices(choice_name, content, facebook_user_id) values(?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(choice_name)
            .bind(content)
            .bind(facebook_user_id)
            .execute(&self.connection)
            .await;

        match result {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Why it's panic! {err}");
                false
            }
        }
    }

    pub async fn delete_all_choices(&self, facebook_user_id: &str) -> bool {
        let sql = "delete from choices where facebook_user_id=?";
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
        let sql = "select content from choices where choice_name=? and facebook_user_id=?";
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
