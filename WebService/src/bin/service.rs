use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models/mod.rs"]
mod models;
#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../error.rs"]
mod error;

use routers::*;
use state::AppState;

#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("not database url in .env");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        // book: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        let cors = Cors::default()
        .allowed_origin("http://localhost:8000")
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().starts_with(b"http://localhost")
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);


        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(book_routes)
            .wrap(cors)
            .configure(author_routes)
    };

     HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}