use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = "mysql://user:userpassword@localhost:3306/mydb";
        
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}