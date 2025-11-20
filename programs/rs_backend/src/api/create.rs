use actix_web::{post, web::Json};
use serde::Serialize;

#[derive(Serialize)]
struct CreateListsResponse {
    hello: String,
}

#[derive(Serialize)]
struct CreateSetsResponse {
    hello: String,
}

#[derive(Serialize)]
struct CreateToDosResponse {
    hello: String,
}

#[post("/api/lists/create")]
pub async fn create_lists(_req_body: String) -> Json<CreateListsResponse> {
    Json(CreateListsResponse {
        hello: "World".to_string(),
    })
}

#[post("/api/sets/create")]
pub async fn create_sets(_req_body: String) -> Json<CreateSetsResponse> {
    Json(CreateSetsResponse {
        hello: "World".to_string(),
    })
}

#[post("/api/to_dos/create")]
pub async fn create_to_dos(_req_body: String) -> Json<CreateToDosResponse> {
    Json(CreateToDosResponse {
        hello: "World".to_string(),
    })
}
