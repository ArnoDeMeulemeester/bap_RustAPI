use actix_web::{web::{self}, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
mod models;
mod service;
pub mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = "mysql://root:qM%25z%25f3EAbiK%40%25ua@localhost/users?charset=utf8mb4".to_string();
    let pool = MySqlPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(service::get_all_users))
            .route("/users/{id}", web::get().to(service::get_user_by_id))
            .route("/users", web::post().to(service::save_user))
            .route("/users/{id}", web::put().to(service::update_user))
            .route("/users/{id}", web::delete().to(service::delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}