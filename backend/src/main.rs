use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use rust_api::models::AppState;
use rust_api::routes;
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::panic;
use std::time::Duration;

#[actix_web::main]
async fn main() -> io::Result<()> {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
    }));

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("📦 Database connected");

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("🚀 Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState::new(db.clone())))
            .configure(routes::config)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
