mod services;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use services::emily_service;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn hello_world() -> impl Responder {
    let _asd = emily_service::connect_check();
    let emoji = "🙏";
    let body_string = String::from("Hello World with Rust");
    // let var_name = emily_service::test_emily();
    let return_message = body_string + emoji + &_asd;
    // let return_message: String = String::from("Hello World");
    HttpResponse::Ok().body(return_message)
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Health Check Completed!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust Actix-web server started at 127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(healthcheck)
            .route("/hey", web::get().to(manual_hello))
            .route("/hello_world", web::get().to(hello_world))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}