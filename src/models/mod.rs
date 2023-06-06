use dotenv::dotenv;
use rusqlite::Connection;
use std::env::var;

fn database_connection() -> Connection {
    dotenv().ok();
    let database_url =
        var("DATABASE").expect("ckeck your .env file \n pls spécifie your database name");
    Connection::open(database_url).expect("failed to open database")
}

pub struct User {
    pub connection: Connection,
}

impl Default for User {
    fn default() -> Self {
        Self {
            connection: database_connection(),
        }
    }
}

impl User {
    fn is_alredy_exist() -> bool {
        let _sql = "select * from user where facebook_user_id=?";
        todo!()
    }

    pub fn create(&self, facebook_user_id: &str) -> bool {
        if !Self::is_alredy_exist() {
            let sql = "insert user (facebook_user_id, action) values (?, ?)";
            self.connection
                .execute(sql, [facebook_user_id, "/"])
                .is_ok()
        } else {
            false
        }
    }

    pub fn set_action(&self, facebook_user_id: &str, action: &str) -> bool {
        let sql = "update user set action=? where facebook_user_id=?";
        self.connection
            .execute(sql, [action, facebook_user_id])
            .is_ok()
    }

    pub fn get_action(&self, facebook_user_id: &str) -> Option<String> {
        let sql = "select action where facebook_user_id=?";
        if let Ok(action) = self
            .connection
            .query_row(sql, [facebook_user_id], |row| Ok(row.get(0).unwrap()))
        {
            Some(action)
        } else {
            None
        }
    }

    pub fn reset_action(&self, facebook_user_id: &str) -> bool {
        let sql = "update set action=? where faceboook_user_id=?";
        self.connection
            .execute(sql, ["/", facebook_user_id])
            .is_ok()
    }
}
