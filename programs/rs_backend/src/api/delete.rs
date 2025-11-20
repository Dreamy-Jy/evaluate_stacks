use actix_web::{HttpResponse, Responder, delete};

#[delete("/api/lists/delete")]
pub async fn delete_lists(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body(_req_body)
}

#[delete("/api/sets/delete")]
pub async fn delete_sets(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body(_req_body)
}

#[delete("/api/to_dos/delete")]
pub async fn delete_to_dos(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body(_req_body)
}
