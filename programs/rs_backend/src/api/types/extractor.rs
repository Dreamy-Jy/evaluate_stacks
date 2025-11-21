use std::{convert::Infallible, sync::Arc};

use actix_web::{
    FromRequest, HttpMessage, HttpRequest,
    dev::Payload,
    error::JsonPayloadError,
    http::header::{ContentLength, Header},
    mime,
    web::BytesMut,
};
use futures_util::{StreamExt, future::LocalBoxFuture};
use serde::de::DeserializeOwned;
use serde_json as json;

#[derive(Debug)]
pub enum MaybeJson<T> {
    Empty,
    Valid(T),
    Invalid(JsonPayloadError),
}

impl<T: DeserializeOwned> FromRequest for MaybeJson<T> {
    type Error = Infallible;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_config = req
            .app_data::<MaybeJsonConfig>()
            .cloned()
            .unwrap_or_default();

        let limit = json_config.get_limit();
        let content_length = match ContentLength::parse(req) {
            Ok(cl) => cl.0,
            Err(_) => 0, // This isn't the right solution, this is really a parse error
        };
        if content_length <= 0 {
            return Box::pin(async { Ok(MaybeJson::Empty) });
        } else if content_length > limit {
            return Box::pin(async move {
                Ok(MaybeJson::Invalid(JsonPayloadError::OverflowKnownLength {
                    length: content_length,
                    limit,
                }))
            });
        }

        let content_type_required = json_config.get_content_type_required();
        let check_content_type = json_config.get_content_type();
        match (content_type_required, req.mime_type()) {
            (false, _) => {}
            (true, Ok(None) | Err(_)) => {
                return Box::pin(
                    async move { Ok(MaybeJson::Invalid(JsonPayloadError::ContentType)) },
                );
            }
            (true, Ok(Some(mime))) => {
                let can_parse_json = mime.subtype() == mime::JSON
                    || mime.suffix() == Some(mime::JSON)
                    || check_content_type.is_some_and(|check| check(mime));
                if !can_parse_json {
                    return Box::pin(async move {
                        Ok(MaybeJson::Invalid(JsonPayloadError::ContentType))
                    });
                }
            }
        }

        let mut payload = payload.take();
        Box::pin(async move {
            let mut req_body = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = match chunk {
                    Ok(new_bytes) => new_bytes,
                    Err(e) => return Ok(MaybeJson::Invalid(JsonPayloadError::Payload(e))),
                };

                if req_body.len() + chunk.len() > limit {
                    return Ok(MaybeJson::Invalid(JsonPayloadError::Overflow { limit }));
                }

                req_body.extend_from_slice(&chunk);
            }

            if req_body.is_empty() {
                return Ok(MaybeJson::Empty);
            }

            let trimmed: Vec<u8> = req_body
                .iter()
                .copied()
                .filter(|b| !b.is_ascii_whitespace())
                .collect();

            match json::from_slice::<T>(&req_body) {
                Ok(json) => {
                    if trimmed == b"[]" {
                        return Ok(MaybeJson::Empty);
                    }
                    Ok(MaybeJson::Valid(json))
                }
                Err(e) => Ok(MaybeJson::Invalid(JsonPayloadError::Deserialize(e))),
            }
        })
    }
}

#[derive(Clone)]
pub struct MaybeJsonConfig {
    limit: usize,
    content_type: Option<Arc<dyn Fn(mime::Mime) -> bool + Send + Sync>>,
    content_type_required: bool,
}

#[allow(dead_code)]
impl MaybeJsonConfig {
    /// Set maximum accepted payload size. By default this limit is 2MB.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn get_limit(&self) -> usize {
        self.limit
    }

    /// Set predicate for allowed content types.
    pub fn content_type<F>(mut self, predicate: F) -> Self
    where
        F: Fn(mime::Mime) -> bool + Send + Sync + 'static,
    {
        self.content_type = Some(Arc::new(predicate));
        self
    }

    pub fn get_content_type(&self) -> Option<&Arc<dyn Fn(mime::Mime) -> bool + Send + Sync>> {
        self.content_type.as_ref()
    }

    /// Sets whether or not the request must have a `Content-Type` header to be parsed.
    pub fn content_type_required(mut self, content_type_required: bool) -> Self {
        self.content_type_required = content_type_required;
        self
    }

    pub fn get_content_type_required(&self) -> bool {
        self.content_type_required
    }
}

impl Default for MaybeJsonConfig {
    fn default() -> Self {
        MaybeJsonConfig {
            limit: 2_097_152, // 2 mb
            content_type: None,
            content_type_required: true,
        }
    }
}

#[cfg(test)]
mod test {
    use std::str;

    use super::*;
    use actix_web::{App, HttpResponse, http::header, test, web};
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
            MaybeJson::Invalid(e) => match e {
                JsonPayloadError::OverflowKnownLength { length, limit } => {
                    HttpResponse::PayloadTooLarge().body(format!(
                        "Payload overflow: {} bytes exceeds limit of {} bytes",
                        length, limit
                    ))
                }
                JsonPayloadError::Overflow { limit } => HttpResponse::PayloadTooLarge()
                    .body(format!("Payload overflow: limit is {} bytes", limit)),
                JsonPayloadError::ContentType => {
                    HttpResponse::UnsupportedMediaType().body("Content type error")
                }
                JsonPayloadError::Deserialize(err) => {
                    HttpResponse::BadRequest().body(format!("JSON deserialize error: {}", err))
                }
                JsonPayloadError::Payload(err) => {
                    HttpResponse::BadRequest().body(format!("Payload error: {}", err))
                }
                _ => HttpResponse::NotImplemented()
                    .body(format!("You shouldn't see this error: {}", e)),
            },
        }
    }

    async fn test_collection_handler(req: MaybeJson<Vec<ValidJson>>) -> HttpResponse {
        match req {
            MaybeJson::Empty => HttpResponse::NoContent().finish(),
            MaybeJson::Valid(data) => HttpResponse::Ok().json(data),
            MaybeJson::Invalid(e) => match e {
                JsonPayloadError::OverflowKnownLength { length, limit } => {
                    HttpResponse::PayloadTooLarge().body(format!(
                        "Payload overflow: {} bytes exceeds limit of {} bytes",
                        length, limit
                    ))
                }
                JsonPayloadError::Overflow { limit } => HttpResponse::PayloadTooLarge()
                    .body(format!("Payload overflow: limit is {} bytes", limit)),
                JsonPayloadError::ContentType => {
                    HttpResponse::UnsupportedMediaType().body("Content type error")
                }
                JsonPayloadError::Deserialize(err) => {
                    HttpResponse::BadRequest().body(format!("JSON deserialize error: {}", err))
                }
                JsonPayloadError::Payload(err) => {
                    HttpResponse::BadRequest().body(format!("Payload error: {}", err))
                }
                _ => HttpResponse::NotImplemented()
                    .body(format!("You shouldn't see this error: {}", e)),
            },
        }
    }

    // TEST MaybeJson START

    #[actix_web::test]
    async fn valid_on_valid_input() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req1 = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .to_request();

        let resp1: actix_web::dev::ServiceResponse = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());
        assert_eq!(resp1.status().as_u16(), 200);

        let resp1_json: ValidJson = test::read_body_json(resp1).await;
        assert_eq!(
            resp1_json,
            ValidJson {
                name: "Alice".to_string(),
                age: 30
            }
        );
    }

    #[actix_web::test]
    async fn invalid_on_invalid_input() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req1 = test::TestRequest::post()
            .uri("/")
            .set_json(InvalidJson {
                list: vec!["item1".to_string(), "item2".to_string()],
                bool_field: true,
            })
            .to_request();

        let resp1: actix_web::dev::ServiceResponse = test::call_service(&app, req1).await;
        assert!(resp1.status().is_client_error());
        assert_eq!(resp1.status().as_u16(), 400);

        let req2 = test::TestRequest::post()
            .uri("/")
            .set_payload("{invalid json")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp2: actix_web::dev::ServiceResponse = test::call_service(&app, req2).await;
        assert!(resp2.status().is_client_error());
        assert_eq!(resp2.status().as_u16(), 400);
    }

    #[actix_web::test]
    async fn empty_on_empty_input() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req1 = test::TestRequest::post()
            .uri("/")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp1: actix_web::dev::ServiceResponse = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());
        assert_eq!(resp1.status().as_u16(), 204); // NoContent

        let req2 = test::TestRequest::post()
            .uri("/")
            .set_payload("")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp2: actix_web::dev::ServiceResponse = test::call_service(&app, req2).await;
        assert!(resp2.status().is_success());
        assert_eq!(resp2.status().as_u16(), 204); // NoContent

        let app2 =
            test::init_service(App::new().route("/", web::to(test_collection_handler))).await;

        let data: Vec<ValidJson> = Vec::new();

        let req3 = test::TestRequest::post()
            .uri("/")
            .set_json(data)
            .to_request();

        let resp3: actix_web::dev::ServiceResponse = test::call_service(&app2, req3).await;
        assert!(resp3.status().is_success());
        assert_eq!(resp3.status().as_u16(), 204); // NoContent
    }

    // TEST MaybeJson END

    // TEST MaybeJsonConfig START

    #[actix_web::test]
    async fn invalid_with_wrong_header() {
        let app = test::init_service(App::new().route("/", web::to(test_handler))).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .insert_header((header::CONTENT_TYPE, mime::TEXT_PLAIN))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        assert_eq!(resp.status().as_u16(), 415); // Unsupported Media Type
        assert_eq!(
            str::from_utf8(&test::read_body(resp).await).unwrap(),
            "Content type error"
        );
    }

    #[actix_web::test]
    async fn invalid_on_known_overflow() {
        let limit = 100;
        let app = test::init_service(
            App::new()
                .app_data(MaybeJsonConfig::default().limit(limit)) // 100 bytes limit
                .route("/", web::to(test_handler)),
        )
        .await;

        let payload = format!(r#"{{"name":"{}","age":30}}"#, "A".repeat(200));

        // Create a large JSON payload that exceeds the 100 byte limit
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((header::CONTENT_TYPE, mime::APPLICATION_JSON))
            .set_payload(payload.clone())
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_client_error());
        assert_eq!(resp.status().as_u16(), 413); // Payload Too Large
        assert_eq!(
            str::from_utf8(&test::read_body(resp).await).unwrap(),
            format!(
                "Payload overflow: {} bytes exceeds limit of {} bytes",
                payload.as_bytes().len(),
                limit
            )
        );
    }

    #[actix_web::test]
    async fn invalid_on_unknown_overflow() {
        let limit = 100;
        let app = test::init_service(
            App::new()
                .app_data(MaybeJsonConfig::default().limit(limit)) // 100 bytes limit
                .route("/", web::to(test_handler)),
        )
        .await;

        let payload = format!(r#"{{"name":"{}","age":30}}"#, "A".repeat(200));

        // Create a large JSON payload without Content-Length header
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload)
            .insert_header((header::CONTENT_LENGTH, "2"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        assert_eq!(resp.status().as_u16(), 413); // Payload Too Large
        assert_eq!(
            str::from_utf8(&test::read_body(resp).await).unwrap(),
            format!("Payload overflow: limit is {} bytes", limit)
        );
    }

    #[actix_web::test]
    async fn can_extend_acceptable_content_types() {
        let app = test::init_service(
            App::new()
                .app_data(MaybeJsonConfig::default().content_type(|mime: mime::Mime| {
                    mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
                }))
                .route("/", web::to(test_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 200);

        let req2 = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Bob".to_string(),
                age: 25,
            })
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let resp2 = test::call_service(&app, req2).await;
        assert!(resp2.status().is_success());
        assert_eq!(resp2.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn can_turn_off_content_type_check() {
        let app = test::init_service(
            App::new()
                .app_data(MaybeJsonConfig::default().content_type_required(false))
                .route("/", web::to(test_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(ValidJson {
                name: "Alice".to_string(),
                age: 30,
            })
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status().as_u16(), 200);
    }

    // TEST MaybeJsonConfig END
}
