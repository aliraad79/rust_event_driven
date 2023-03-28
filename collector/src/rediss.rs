use serde_json;

use crate::Task;

extern crate redis;

pub async fn add_task(task: Task) {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = redis::cmd("LPUSH")
        .arg("tasks")
        .arg(serde_json::to_string(&task).unwrap())
        .query(&mut conn)
        .expect("failed to execute LPUSH for 'items'");
}
