mod db;
mod rediss;
mod schema;
mod task;

use dotenv::dotenv;
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    dotenv().ok();
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_secs(60 * 5);

        // Make this an infinite loop
        // Or some control path to exit the loop
        loop {
            let start = Instant::now();
            eprintln!("Scheduler starting at {:?}", start);

            let thread_a = thread::spawn(heavy_task);

            thread_a.join().expect("Thread A panicked");

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                eprintln!(
                    "schedule slice has time left over; sleeping for {:?}",
                    remaining
                );
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().expect("Scheduler panicked");
}

fn heavy_task() {
    println!("Running heavy task");
    let number_of_tasks_in_queue = match rediss::get_number_of_tasks_in_queue() {
        Ok(n) => n,
        Err(_err) => {
            println!("Redis paniced!");
            return;
        }
    };

    println!("Doing {} task in loop", number_of_tasks_in_queue);
    match rediss::get_last_n_task(number_of_tasks_in_queue) {
        Ok(tasks) => db::add_to_db(tasks),
        Err(_err) => println!("Redis paniced!"),
    };
}
