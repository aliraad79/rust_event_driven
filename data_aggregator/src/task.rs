use diesel::prelude::*;
use redis::{from_redis_value, ErrorKind, FromRedisValue, RedisResult, Value};
use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct Task {
    id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

impl FromRedisValue for NewTask {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let json_str: String = from_redis_value(v)?;

        let result: Self = match serde_json::from_str::<Self>(&json_str) {
            Ok(v) => v,
            Err(_err) => return Err((ErrorKind::TypeError, "Parse to JSON Failed").into()),
        };

        Ok(result)
    }
}
