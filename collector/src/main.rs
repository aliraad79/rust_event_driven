#[macro_use]
extern crate rocket;

mod my_cors;
mod rediss;
use rocket::serde::{json::Json, Deserialize};

use rocket::{get, routes};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task<'r> {
    title: &'r str,
    description: &'r str,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/task", data = "<task>")]
fn new(task: Json<Task<'_>>) {
    println!("Task {} {}", task.title, task.description);
    rediss::add_task(task.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(my_cors::CORS)
        .mount("/", routes![index, new])
}
