use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::env;

use crate::schema::tasks;
use crate::task::NewTask;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_to_db(tasks: Vec<NewTask>) {
    let mut conn = establish_connection();

    diesel::insert_into(tasks::table)
        .values(&tasks)
        .execute(&mut conn)
        .expect("Error saving new post");
}
