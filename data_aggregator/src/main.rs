mod db;
mod rediss;
mod task;
mod schema;

use task::Task;

fn main() {
    // let tasks_string = rediss::get_tasks();
    let last_task = rediss::get_last_task();

    match last_task {
        Ok(task) => db::add_to_db(task),
        Err(string) => println!("Error {:?}", string),
    }
}
