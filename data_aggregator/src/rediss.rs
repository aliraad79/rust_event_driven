extern crate redis;

pub fn get_tasks() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut conn = client.get_connection().unwrap();
    // let task_string = convert_task_to_string(task);
    // con.LPUSH
    let _: () = redis::cmd("LRANGE")
        .arg("tasks")
        .arg("-1 0")
        .query(&mut conn)
        .expect("failed to execute LPUSH for 'items'");
}

// fn convert_task_to_string(task: Task) -> String {
//     format!("{} {}", task.title, task.description)
// }
