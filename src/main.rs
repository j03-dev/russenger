use potato::models::database_connection;

use std::{env, fmt::Error};

fn parser() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Ok(String::from(&args[1])),
        _ => Err(Error),
    }
}

#[tokio::main]
async fn main() {
    if let Ok(cmd) = parser() {
        match cmd.as_str() {
            "migrate" => {
                let model_sql = "
                    create table user (
                        facebook_user_id varchar(40) primary key,
                        action varchar(20)
                    );
                    create table choices(
                        choices_name varchar(20),
                        content varchar(255),
                        facebook_user_id varchar(40),
                        foreign key(facebook_user_id) references user(facebook_user_id)
                    )";
                let connection = database_connection().await;
                if sqlx::query(model_sql).execute(&connection).await.is_ok() {
                    println!("migrate succes");
                } else {
                    println!("failde to migrate");
                }
            }
            _ => println!("potato migrate : for create new db"),
        }
    } else {
        println!("command not found");
    }
}
