use crate::Task;

extern crate redis;

pub fn add_task(task: Task) {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();
    let task_string = convert_task_to_string(task);
    let _: () = redis::cmd("LPUSH")
        .arg("tasks")
        .arg(task_string)
        .query(&mut conn)
        .expect("failed to execute LPUSH for 'items'");
}

fn convert_task_to_string(task: Task) -> String {
    format!("{} {}", task.title, task.description)
}
