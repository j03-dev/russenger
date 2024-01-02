use std::env::var;

use sqlx::{MySql, MySqlPool, Pool, Row};

async fn database_connection() -> Pool<MySql> {
    let url = var("DATABASE").expect("check your .env file \n pls spécifie your database name");
    MySqlPool::connect(&url)
        .await
        .expect("Database connection failed")
}

pub struct Query {
    pub connection: Pool<MySql>,
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
        let sql = "insert into russenger_user (facebook_user_id, action) values (?, ?)";
        sqlx::query(sql)
            .bind(user_id)
            .bind("Main")
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn set_action(&self, user_id: &str, action: &str) -> bool {
        let sql = "update russenger_user set action=? where facebook_user_id=?";
        sqlx::query(sql)
            .bind(action)
            .bind(user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }

    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        let sql = "select action from russenger_user where facebook_user_id=?";
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
        let sql = "update russenger_user set action=? where facebook_user_id=?";
        sqlx::query(sql)
            .bind("Main")
            .bind(user_id)
            .execute(&self.connection)
            .await
            .is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::query::Query;
    use dotenv::dotenv;

    #[rocket::async_test]
    async fn insert_user() {
        dotenv().ok();
        let query = Query::new().await;
        let left = query.create("test").await;
        assert_eq!(true, left);
    }

    #[rocket::async_test]
    async fn set_action() {
        dotenv().ok();
        let query = Query::new().await;
        let left = query.set_action("test", "Test").await;
        assert_eq!(true, left);
    }
}
