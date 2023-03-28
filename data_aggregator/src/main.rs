mod db;
mod rediss;
mod schema;
mod task;
mod celery_tasks;

use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    let last_task = rediss::get_last_n_task(10);

    match last_task {
        Ok(tasks) => db::add_to_db(tasks),
        Err(string) => println!("Error {:?}", string),
    }
}
