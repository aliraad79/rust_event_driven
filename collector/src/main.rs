use actix_cors::Cors;
use actix_web::{
    get, http, middleware::Logger, post, web::Json, App, HttpResponse, HttpServer, Responder,
};
use env_logger;
use serde::{Deserialize, Serialize};

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
        let cors = Cors::default()
            .allowed_origin("127.0.0.1")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(index)
            .service(new_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
