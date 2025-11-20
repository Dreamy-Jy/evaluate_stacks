use actix_web::{patch, web::Json};
use serde::Serialize;

#[derive(Serialize)]
struct UpdateListsResponse {
    hello: String,
}

#[derive(Serialize)]
struct UpdateSetsResponse {
    hello: String,
}

#[derive(Serialize)]
struct UpdateToDosResponse {
    hello: String,
}

#[patch("/api/lists")]
pub async fn update_lists(_req_body: String) -> Json<UpdateListsResponse> {
    Json(UpdateListsResponse {
        hello: "World".to_string(),
    })
}

#[patch("/api/sets")]
pub async fn update_sets(_req_body: String) -> Json<UpdateSetsResponse> {
    Json(UpdateSetsResponse {
        hello: "World".to_string(),
    })
}

#[patch("/api/to_dos")]
pub async fn update_to_dos(_req_body: String) -> Json<UpdateToDosResponse> {
    Json(UpdateToDosResponse {
        hello: "World".to_string(),
    })
}
