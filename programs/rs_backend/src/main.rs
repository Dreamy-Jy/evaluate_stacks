use actix_web::{App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::create::create_lists)
            .service(api::create::create_sets)
            .service(api::create::create_to_dos)
            .service(api::read::read_lists)
            .service(api::read::read_sets)
            .service(api::read::read_to_dos)
            .service(api::update::update_lists)
            .service(api::update::update_sets)
            .service(api::update::update_to_dos)
            .service(api::delete::delete_lists)
            .service(api::delete::delete_sets)
            .service(api::delete::delete_to_dos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
