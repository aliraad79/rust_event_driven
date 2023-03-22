extern crate redis;

use crate::Task;

pub fn get_tasks() -> Vec<String> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();

    let keys_length: usize = redis::cmd("LLEN")
        .arg("tasks")
        .query(&mut conn)
        .expect("failed to execute LLEN for 'tasks'");

    let items: Vec<String> = redis::cmd("RPOP")
        .arg("tasks")
        .arg(keys_length)
        .query(&mut conn)
        .expect("failed to execute RPOP for 'tasks'");

    return items;
}

pub fn get_last_task() -> Task<'static> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();

    let bar: String = redis::cmd("GET")
        .arg("tasks")
        .query(&mut conn)
        .expect("failed to execute GET for 'tasks'");

    serde_json::from_str::<Task>(bar.as_str()).expect("Exec")
}
