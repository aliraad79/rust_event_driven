mod rediss;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task<'r> {
    title: &'r str,
    description: &'r str,
}

fn main() {
    // let tasks_string = rediss::get_tasks();

    // let tasks: Vec<Task> = tasks_string
    //     .into_iter()
    //     .map(|f| serde_json::from_str::<Task>(f.clone().as_str()).expect("Exec"))
    //     .collect::<Vec<Task>>();
    // println!("{:?}", tasks);
}
