use rocket::serde::Serialize;
use serde_json::Value;
use sqlx::{Database, MySql, Pool, Row};
use std::env::var;

pub trait Table: Serialize + Default {
    fn to(&self) -> (Vec<String>, Vec<String>) {
        let mut keys: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        if let Value::Object(map) =
            serde_json::to_value(&self).expect("Failed to convert this to tuple")
        {
            for (k, v) in map.iter() {
                keys.push(k.to_string());
                values.push(v.to_string());
            }
        }

        (keys, values)
    }

    fn table_name() -> &'static str
    where
        Self: Sized;

    fn schema() -> &'static str
    where
        Self: Sized;
}

#[rocket::async_trait]
trait Repository<D: Database, T: Table> {
    async fn conn() -> sqlx::Pool<D> {
        let url = var("DATABASE").expect("check your .env file \n pls spécifie your database name");
        Pool::connect(&url)
            .await
            .expect("Connect to the database is Failed!")
    }

    async fn migrate() -> bool
    where
        Self: Sized;

    async fn create(item: T) -> bool
    where
        Self: Sized;

    async fn update(item: T) -> bool
    where
        Self: Sized;

    async fn delete(pk: &str) -> bool
    where
        Self: Sized;

    async fn get(pk: &str) -> Option<T>
    where
        Self: Sized;
    async fn all() -> Vec<T>
    where
        Self: Sized;
}

#[derive(Serialize, Default)]
pub struct User {
    pub id: String,
    pub action: String,
}

impl Table for User {
    fn table_name() -> &'static str {
        "user"
    }

    fn schema() -> &'static str {
        "create table user (id varchar(50) primary key, action varchar(20));"
    }
}

struct UserRepository;

#[rocket::async_trait]
impl Repository<MySql, User> for UserRepository {
    async fn migrate() -> bool {
        sqlx::query(User::schema())
            .execute(&Self::conn().await)
            .await
            .is_ok()
    }

    async fn create(item: User) -> bool {
        let sql = format!(
            "insert into {table_name} (id, action) values (?, ?)",
            table_name = User::table_name()
        );
        sqlx::query(&sql)
            .bind(&item.id)
            .bind(&item.action)
            .execute(&Self::conn().await)
            .await
            .is_ok()
    }

    async fn update(item: User) -> bool {
        let sql = format!(
            "update {table_name} set action=? where id=?",
            table_name = User::table_name()
        );

        sqlx::query(&sql)
            .bind(&item.action)
            .bind(&item.id)
            .execute(&Self::conn().await)
            .await
            .is_ok()
    }

    async fn delete(pk: &str) -> bool {
        let sql = format!(
            "delete from {table_name} where id=?",
            table_name = User::table_name()
        );
        sqlx::query(&sql)
            .bind(pk)
            .execute(&Self::conn().await)
            .await
            .is_ok()
    }

    async fn get(pk: &str) -> Option<User> {
        let sql = format!(
            "select from {table_name} where id=?",
            table_name = User::table_name()
        );
        match sqlx::query(&sql)
            .bind(pk)
            .fetch_one(&Self::conn().await)
            .await
        {
            Ok(row) => Some(User {
                id: pk.into(),
                action: row.get(0),
            }),
            Err(_) => None,
        }
    }
    async fn all() -> Vec<User> {
        todo!()
    }
}


#[cfg(test)]
mod test_repository {
    use super::{User, Table};

    #[test] 
    fn print()  {
       let user = User {
           id: "3".into(),
           action: "hello".into()
       };

       let left = User::table_name();
       assert_eq!(left, "user");
    }

}
