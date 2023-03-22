extern crate redis;

use redis::RedisError;

use crate::task::NewTask;

pub fn get_last_n_task(n: i32) -> Result<Vec<NewTask>, RedisError> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();

    redis::cmd("RPOP")
        .arg("tasks")
        .arg(n)
        .query::<Vec<NewTask>>(&mut conn)
}
