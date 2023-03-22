mod db;
mod rediss;

use redis::{from_redis_value, ErrorKind, FromRedisValue, RedisResult, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    title: String,
    description: String,
}
impl FromRedisValue for Task {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let json_str: String = from_redis_value(v)?;

        let result: Self = match serde_json::from_str::<Self>(&json_str) {
            Ok(v) => v,
            Err(_err) => return Err((ErrorKind::TypeError, "Parse to JSON Failed").into()),
        };

        Ok(result)
    }
}

fn main() {
    // let tasks_string = rediss::get_tasks();
    let last_task = rediss::get_last_task();

    match last_task {
        Ok(task) => db::add_to_db(task),
        Err(string) => println!("Error {:?}", string),
    }
}
