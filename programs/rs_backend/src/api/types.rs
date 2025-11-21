use std::future::{self, Ready, ready};

use actix_web::{
    FromRequest, HttpRequest,
    dev::Payload,
    error::JsonPayloadError,
    http::header,
    web::{BytesMut, Data, Json, JsonConfig, head},
};
use futures_util::{StreamExt, future::LocalBoxFuture};
use serde::de::DeserializeOwned;
use serde_json as json;

pub enum MaybeJson<T> {
    Empty,
    Valid(T),
    Invalid(JsonPayloadError),
}

impl<T: DeserializeOwned> FromRequest for MaybeJson<T> {
    type Error = JsonPayloadError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let content_length = match req.headers().get(header::CONTENT_LENGTH) {
            None => 0,
            Some(header) => header.to_str().unwrap_or("0").parse::<usize>().unwrap_or(0),
        };
        if content_length <= 0 {
            return Box::pin(async { Ok(MaybeJson::Empty) });
        }

        let content_type = match req.headers().get(header::CONTENT_TYPE) {
            None => "",
            Some(header) => header.to_str().unwrap_or(""),
        };
        if content_type != "application/json" {
            return Box::pin(async { Ok(MaybeJson::Invalid(JsonPayloadError::ContentType)) });
        }

        // I haven't checked the size requirement.
        let mut payload = payload.take();
        Box::pin(async move {
            let mut req_body = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = match chunk {
                    Ok(new_bytes) => new_bytes,
                    Err(e) => return Ok(MaybeJson::Invalid(JsonPayloadError::Payload(e))),
                };

                req_body.extend_from_slice(&chunk);
            }

            if req_body.is_empty() {
                return Ok(MaybeJson::Empty)
            }

            match json::from_slice::<T>(&req_body) {
                Ok(json) => Ok(MaybeJson::Valid(json)),
                Err(e) => Ok(MaybeJson::Invalid(JsonPayloadError::Deserialize(e)))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use std::str;

    use super::*;
    use actix_web::{
        App, HttpResponse,
        http::header,
        test,
        web::{self, JsonConfig},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct ValidJson {
        name: String,
        age: i32,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct InvalidJson {
        list: Vec<String>,
        bool_field: bool,
    }

    async fn test_handler(req: MaybeJson<ValidJson>) -> HttpResponse {
        match req {
            MaybeJson::Empty => HttpResponse::NoContent().finish(),
            MaybeJson::Valid(data) => HttpResponse::Ok().json(data),
            MaybeJson::Invalid(e) => {
                HttpResponse::BadRequest().body(format!("Invalid JSON: {}", e))
            }
        }
    }

    #[actix_web::test]
    async fn fails_with_wrong_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        // JsonConfig

        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    //TEST: Valid JSON, Valid Request
    //  Got valid JSON that matches expected type
    #[actix_web::test]
    async fn test_valid_json_right_type() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    //TEST: Valid JSON, Invalid request
    //  Got Valid Json that does not match expected type
    #[actix_web::test]
    async fn test_valid_json_wrong_type() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(InvalidJson {
                list: vec!["item1".to_string(), "item2".to_string()],
                bool_field: true,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    //TEST: Invalid JSON
    //  Test malformed JSON
    #[actix_web::test]
    async fn test_invalid_json_proper_headers() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("{invalid json")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    //TEST: Non-JSON payloads with JSON header
    #[actix_web::test]
    async fn test_non_json_payload_with_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("plain text content")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    //TEST: Non-JSON payloads without JSON header
    #[actix_web::test]
    async fn test_non_json_payload_without_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("plain text content")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 204); // NoContent
    }

    //TEST: Empty Payloads - No Body, Json Header
    #[actix_web::test]
    async fn test_no_body_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Empty body with JSON header should result in error
        assert!(resp.status().is_client_error());
    }

    //TEST: Empty Payloads - No Body, No Json Header
    #[actix_web::test]
    async fn test_no_body_no_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post().uri("/").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 204); // NoContent
    }

    //TEST: Empty Payloads - Empty body, Json Header
    #[actix_web::test]
    async fn test_empty_body_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Empty string body with JSON header should result in error
        assert!(resp.status().is_client_error());
    }

    //TEST: Empty Payloads - Empty body, No Json Header
    #[actix_web::test]
    async fn test_empty_body_no_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 204); // NoContent
    }

    //TEST: Empty Payloads - Empty JSON Object, Json Header
    #[actix_web::test]
    async fn test_empty_json_object_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("{}")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Empty JSON object doesn't match ValidJson structure (missing required fields)
        assert!(resp.status().is_client_error());
    }

    //TEST: Empty Payloads - Empty JSON Object, No Json Header
    #[actix_web::test]
    async fn test_empty_json_object_no_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("{}")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 204); // NoContent
    }

    //TEST: Empty Payloads - Empty JSON Array, Json Header
    #[actix_web::test]
    async fn test_empty_json_array_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("[]")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Empty JSON array doesn't match ValidJson structure
        assert!(resp.status().is_client_error());
    }

    //TEST: Empty Payloads - Empty JSON Array, No Json Header
    #[actix_web::test]
    async fn test_empty_json_array_no_json_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("[]")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 204); // NoContent
    }

    //TEST: JSON That Exceeds Size Limit
    #[actix_web::test]
    async fn test_json_exceeds_size_limit() {
        let app = test::init_service(
            App::new()
                .app_data(JsonConfig::default().limit(100)) // 100 bytes limit
                .route("/", web::to(test_handler)),
        )
        .await;

        // Create a large JSON payload that exceeds the 100 byte limit
        let large_name = "A".repeat(200);
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: large_name,
                age: 30,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
