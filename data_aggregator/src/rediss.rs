extern crate redis;

use crate::Task;

pub fn get_last_task() -> Result<Task, String> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();

    let keys_length: usize = redis::cmd("LLEN")
        .arg("tasks")
        .query(&mut conn)
        .expect("failed to execute LLEN for 'tasks'");

    if keys_length <= 0 {
        return Err(String::from("Not enough tasks in queue"));
    }

    Ok(redis::cmd("RPOP")
        .arg("tasks")
        .query::<Task>(&mut conn)
        .expect("failed to execute RPOP for 'tasks'"))
}
