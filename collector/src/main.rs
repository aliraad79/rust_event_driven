use actix_web::{
    get, middleware::Logger, post, web::Json, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use env_logger;

mod rediss;

#[derive(Deserialize, Serialize)]
pub struct Task {
    title: String,
    description: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/task")]
async fn new_task(task: Json<Task>) -> impl Responder {
    rediss::add_task(task.into_inner()).await;
    HttpResponse::Ok().body("OK!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(new_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
