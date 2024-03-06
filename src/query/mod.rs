use core::panic;
use std::env::var;

use sqlx::{MySql, Pool, Postgres, Row, Sqlite};

#[derive(Clone)]
pub enum DB {
    Mysql(Pool<MySql>),
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
    Null,
}

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn establish_connection() -> DbResult<DB> {
    let database_url = var("DATABASE").expect("Database name not found in .env file");
    let msg = "Database connection failed";

    let engine = database_url.split(':').next().ok_or(msg)?;

    let pool = match engine {
        "mysql" => Pool::connect(&database_url).await.map(DB::Mysql)?,
        "postgres" | "postgresql" => Pool::connect(&database_url).await.map(DB::Postgres)?,
        "sqlite" => Pool::connect(&database_url).await.map(DB::Sqlite)?,
        _ => return Ok(DB::Null),
    };

    Ok(pool)
}

macro_rules! execute_query {
    ($pool:expr, $sql:expr, $params:expr) => {{
        let mut query = sqlx::query($sql);
        for parm in $params {
            query = query.bind(parm);
        }
        query.execute($pool).await.is_ok()
    }};
}

#[derive(Clone)]
pub struct Query {
    pub db: DB,
}

impl Query {
    pub async fn new() -> Self {
        match establish_connection().await {
            Ok(db) => return Self { db },
            Err(err) => panic!("Can't estabilish the connection {err:?}"),
        }
    }

    pub async fn migrate(&self) -> bool {
        let sql = "
            create table russenger_user (
                facebook_user_id varchar(40) primary key unique,
                action varchar(20)
            );";

        let no_params = Vec::<&str>::new();
        match &self.db {
            DB::Mysql(pool) => execute_query!(pool, sql, no_params),
            DB::Sqlite(pool) => execute_query!(pool, sql, no_params),
            DB::Postgres(pool) => execute_query!(pool, sql, no_params),
            DB::Null => false,
        }
    }

    pub async fn create(&self, user_id: &str) -> bool {
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values (?, ?)";
                execute_query!(pool, sql, [user_id, "Main"])
            }
            DB::Sqlite(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
                execute_query!(pool, sql, [user_id, "Main"])
            }
            DB::Postgres(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
                execute_query!(pool, sql, [user_id, "Main"])
            }
            DB::Null => false,
        }
    }

    pub async fn set_action(&self, user_id: &str, action: &str) -> bool {
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "update russenger_user set action=? where facebook_user_id=?";
                execute_query!(pool, sql, [action, user_id])
            }
            DB::Sqlite(pool) => {
                let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
                execute_query!(pool, sql, [action, user_id])
            }
            DB::Postgres(pool) => {
                let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
                execute_query!(pool, sql, [action, user_id])
            }
            DB::Null => false,
        }
    }

    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=?";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Sqlite(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=$1";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Postgres(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=$1";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Null => None,
        }
    }

    pub async fn reset_action(&self, user_id: &str) -> bool {
        self.set_action(user_id, "Main").await
    }
}
