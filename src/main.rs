use std::env;
use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, Result, web};
use actix_web::http::header;
use dotenv::dotenv;
// use reqwest::header;
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

mod api;
mod models;
mod schemas;

#[derive(Serialize)]
pub struct Response {
    // pub status: String,
    pub message: String,
}

pub struct AppState {
    db: MySqlPool,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            // .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(api::todo::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((
        env::var("SERVER_URL").unwrap(),
        env::var("SERVER_PORT").unwrap().parse().unwrap(),
    ))?
    .run()
    .await
}
