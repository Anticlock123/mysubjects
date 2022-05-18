use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[path = "../db_access.rs"]
mod db_access;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;
use errors::MyError;
use pedersencommit::test;
#[actix_rt::main]
async fn main() -> io::Result<()> {

   
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not");
    
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,

    });
    let app = move ||{
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT,http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            // .app_data(web::JsonConfig::default().error_handler(|_err,_req|{
            //     MyError::InvalidInput("Please provide a valid".to_string()).into()
            // }))
            .configure(course_routes)
            .wrap(cors)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}