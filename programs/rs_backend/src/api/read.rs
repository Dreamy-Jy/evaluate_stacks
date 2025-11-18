use actix_web::{get, web::Json};
use serde::Serialize;

#[derive(Serialize)]
struct ReadListsResponse {
    hello: String
}

#[derive(Serialize)]
struct ReadSetsResponse {
    hello: String
}

#[derive(Serialize)]
struct ReadToDosResponse {
    hello: String
}

#[get("/api/lists/read")]
pub async fn read_lists(_req_body: String) -> Json<ReadListsResponse> {
    Json(ReadListsResponse {
        hello: "World".to_string(),
    })
}

#[get("/api/sets/read")]
pub async fn read_sets(_req_body: String) -> Json<ReadSetsResponse> {
    Json(ReadSetsResponse {
        hello: "World".to_string(),
    })
}

#[get("/api/to_dos/read")]
pub async fn read_to_dos(_req_body: String) -> Json<ReadToDosResponse> {
    Json(ReadToDosResponse {
        hello: "World".to_string(),
    })
}