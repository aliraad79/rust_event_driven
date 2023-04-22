extern crate redis;

use redis::{Commands, RedisError};
use std::env;

use crate::task::NewTask;

fn get_redis_url() -> String {
    if env::var("DOCKER").unwrap_or("false".to_string()) != "true" {
        env::var("REDIS_URL").expect("REDIS_URL must be set")
    } else {
        env::var("DOCKER_REDIS_URL").expect("DOCKER_REDIS_URL must be set")
    }
}

pub fn get_last_n_task(n: i64) -> Result<Vec<NewTask>, RedisError> {
    let client = redis::Client::open(get_redis_url()).unwrap();
    let mut conn = client.get_connection().unwrap();

    redis::cmd("RPOP")
        .arg("tasks")
        .arg(n)
        .query::<Vec<NewTask>>(&mut conn)
}

pub fn get_number_of_tasks_in_queue() -> Result<i64, RedisError> {
    let client = redis::Client::open(get_redis_url()).unwrap();
    let mut conn = client.get_connection().unwrap();

    conn.llen("tasks")
}
