use actix_web::{App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
