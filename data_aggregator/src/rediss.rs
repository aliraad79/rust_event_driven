extern crate redis;

use redis::{Commands, RedisError};

use crate::task::NewTask;

pub fn get_last_n_task(n: i64) -> Result<Vec<NewTask>, RedisError> {
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut conn = client.get_connection().unwrap();

    redis::cmd("RPOP")
        .arg("tasks")
        .arg(n)
        .query::<Vec<NewTask>>(&mut conn)
}

pub fn get_number_of_tasks_in_queue() -> Result<i64, RedisError> {
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut conn = client.get_connection().unwrap();

    conn.llen("tasks")
}
