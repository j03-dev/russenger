use std::env::var;

use sqlx::{MySql, Pool, Postgres, Row, Sqlite};

#[derive(Clone)]
pub enum DB {
    Mysql(Pool<MySql>),
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
    Null,
}

async fn establish_connection() -> DB {
    let url = var("DATABASE").expect("check your .env file \n pls specified your database name");
    let msg = "Database connection failed";
    if let Some(engine) = url.split(':').next() {
        return match engine {
            "mysql" => {
                let pool: Pool<MySql> = Pool::connect(&url).await.expect(msg);
                DB::Mysql(pool)
            }
            "postgres" | "postgresql" => {
                let pool: Pool<Postgres> = Pool::connect(&url).await.expect(msg);
                DB::Postgres(pool)
            }
            "sqlite" => {
                let pool: Pool<Sqlite> = Pool::connect(&url).await.expect(msg);
                DB::Sqlite(pool)
            }
            _ => DB::Null,
        };
    }
    DB::Null
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
        Self {
            db: establish_connection().await,
        }
    }

    pub async fn migrate(&self) -> bool {
        let create_table_user = "
            create table russenger_user (
                facebook_user_id varchar(40) primary key unique,
                action varchar(20)
            );";

        let no_params = Vec::<&str>::new();
        match &self.db {
            DB::Mysql(pool) => execute_query!(pool, create_table_user, no_params),
            DB::Sqlite(pool) => execute_query!(pool, create_table_user, no_params),
            DB::Postgres(pool) => execute_query!(pool, create_table_user, no_params),
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
