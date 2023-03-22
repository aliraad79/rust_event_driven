use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::schema::tasks;
use crate::task::{NewTask, Task};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    // println!("{:?}", env::var("DATABASE_URL"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn add_to_db(task: Task) {
    let mut conn = establish_connection();

    let new_task = NewTask {
        title: &task.title,
        description: &task.description,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut conn)
        .expect("Error saving new post");
    println!("Inserted");
}
