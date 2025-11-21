use actix_web::{App, HttpServer, web::Data};
use sqlx::SqlitePool;

mod api;
mod db;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match SqlitePool::connect("../database/database_v2.db").await {
        Ok(p) => p,
        Err(e) => {
            panic!("Failed to connect to the database: {}", e);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(api::create_lists)
            .service(api::create_sets)
            .service(api::create_to_dos)
            .service(api::read_lists)
            .service(api::read_sets)
            .service(api::read_to_dos)
            .service(api::update_lists)
            .service(api::update_sets)
            .service(api::update_to_dos)
            .service(api::delete_lists)
            .service(api::delete_sets)
            .service(api::delete_to_dos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
