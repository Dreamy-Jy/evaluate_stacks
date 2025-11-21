use actix_web::{
    error::JsonPayloadError,
    web::{Data, Json},
};
use len_trait::Len;
use sqlx::{Error as SQLXError, Pool, Sqlite};

use crate::api::types::{JsonError, MaybeJson};

pub async fn query_some<In, Out, Qsome, Fut>(
    req: MaybeJson<In>,
    db: Data<Pool<Sqlite>>,
    query_some: Qsome,
) -> Result<Json<Out>, JsonError>
where
    In: Len,
    Fut: Future<Output = Result<Out, SQLXError>>,
    Qsome: Fn(Data<Pool<Sqlite>>, In) -> Fut,
{
    match req {
        MaybeJson::Valid(json) => match query_some(db, json).await {
            Ok(result) => Ok(Json(result)),
            Err(err) => match err {
                SQLXError::InvalidArgument(msg) => Err(JsonError::BadRequest(format!(
                    "Invalid Argument Provided: {}",
                    msg
                ))),
                _ => Err(JsonError::ServerError(format!(
                    "Database Insertion Error: {}",
                    err
                ))),
            },
        },
        MaybeJson::Empty => Err(JsonError::BadRequest(
            "Empty request not allowed".to_string(),
        )),
        MaybeJson::Invalid(err) => match err {
            JsonPayloadError::Overflow { limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload is greater than the limit for {} bytes",
                    limit
                )));
            }
            JsonPayloadError::OverflowKnownLength { length, limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload length of {} bytes is greater than the limit for {} bytes",
                    length, limit
                )));
            }
            JsonPayloadError::ContentType => {
                return Err(JsonError::UnsupportedMediaType(
                    "Unsupported 'Content-Type' header or missing 'Content-Type' header"
                        .to_string(),
                ));
            }
            JsonPayloadError::Payload(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error processing your payload: {}",
                    e
                )));
            }
            JsonPayloadError::Deserialize(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error deserializing your payload: {}",
                    e
                )));
            }
            _ => {
                return Err(JsonError::Unknown(format!(
                    "Unknown JSON payload error: {}",
                    err
                )));
            }
        },
    }
}

#[cfg(test)]
mod test {

    use std::sync::atomic::{AtomicBool, Ordering};

    use actix_web::{
        ResponseError,
        error::{JsonPayloadError, PayloadError},
        web::Data,
    };
    use serde::{Deserialize, Serialize};
    use sqlx::{Error, Pool, Sqlite, SqlitePool};

    use crate::api::{
        types::{JsonError, MaybeJson},
        utils::query_some,
    };

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    struct TestInput {
        caller: String,
    }

    async fn setup_test_db() -> Pool<Sqlite> {
        SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create test database")
    }

    static TOGGLE_INVALID_ARGUMENT: AtomicBool = AtomicBool::new(false);

    fn set_invalid_argument(set: bool) {
        TOGGLE_INVALID_ARGUMENT.store(set, Ordering::Relaxed);
    }

    async fn query_success(
        _db: Data<Pool<Sqlite>>,
        _input: Vec<TestInput>,
    ) -> Result<Vec<TestInput>, Error> {
        Ok(vec![TestInput {
            caller: "query some".to_string(),
        }])
    }

    async fn query_fail(
        _db: Data<Pool<Sqlite>>,
        _input: Vec<TestInput>,
    ) -> Result<Vec<TestInput>, Error> {
        if TOGGLE_INVALID_ARGUMENT.load(Ordering::Relaxed) {
            return Err(Error::InvalidArgument(
                "query_fail: argument error".to_string(),
            ));
        }

        Err(Error::Protocol(
            "query_fail: stand in for any db error".to_string(),
        ))
    }

    // TEST if Valid query some
    #[actix_web::test]
    async fn test_valid_request_calls_query_some() {
        let db = Data::new(setup_test_db().await);
        let req = MaybeJson::Valid(vec![TestInput {
            caller: "test input".to_string(),
        }]);

        let result = query_some(req, db, query_success).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().to_vec(),
            vec![TestInput {
                caller: "query some".to_string(),
            }]
        );
    }

    // TEST Error if query fails
    #[actix_web::test]
    async fn test_query_some_fail() {
        // Test generic database error
        let db1 = Data::new(setup_test_db().await);
        let req1 = MaybeJson::Valid(vec![TestInput {
            caller: "test input".to_string(),
        }]);

        let result1 = query_some(req1, db1, query_fail).await;

        assert!(result1.is_err());
        assert_eq!(
            result1.unwrap_err().status_code(),
            JsonError::ServerError("".to_string()).status_code()
        );

        // Test invalid argument error
        set_invalid_argument(true);

        let db2 = Data::new(setup_test_db().await);
        let req2 = MaybeJson::Valid(vec![TestInput {
            caller: "test input".to_string(),
        }]);

        let result2 = query_some(req2, db2, query_fail).await;

        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err().status_code(),
            JsonError::BadRequest("test".to_string()).status_code()
        );

        // Reset for other tests
        set_invalid_argument(false);
    }

    // TEST Error empty request
    #[actix_web::test]
    async fn test_empty_request_returns_error() {
        let db = Data::new(setup_test_db().await);
        let req = MaybeJson::Empty;

        let result = query_some(req, db, query_success).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().status_code(),
            JsonError::BadRequest("".to_string()).status_code()
        );
    }

    // TEST if Invalid Error
    #[actix_web::test]
    async fn test_invalid_json_fails() {
        // Test Payload error
        let db1 = Data::new(setup_test_db().await);
        let req1 = MaybeJson::Invalid(JsonPayloadError::Payload(PayloadError::Overflow));

        let result1 = query_some(req1, db1, query_success).await;

        assert!(result1.is_err());
        assert_eq!(
            result1.unwrap_err().status_code(),
            JsonError::BadRequest("".to_string()).status_code()
        );

        // Test Overflow error
        let db2 = Data::new(setup_test_db().await);
        let req2 = MaybeJson::Invalid(JsonPayloadError::Overflow { limit: 1024 });

        let result2 = query_some(req2, db2, query_success).await;

        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err().status_code(),
            JsonError::PayloadTooLarge("".to_string()).status_code()
        );

        // Test OverflowKnownLength error
        let db3 = Data::new(setup_test_db().await);
        let req3 = MaybeJson::Invalid(JsonPayloadError::OverflowKnownLength {
            length: 2048,
            limit: 1024,
        });

        let result3 = query_some(req3, db3, query_success).await;

        assert!(result3.is_err());
        assert_eq!(
            result3.unwrap_err().status_code(),
            JsonError::PayloadTooLarge("".to_string()).status_code()
        );

        // Test ContentType error
        let db4 = Data::new(setup_test_db().await);
        let req4 = MaybeJson::Invalid(JsonPayloadError::ContentType);

        let result4 = query_some(req4, db4, query_success).await;

        assert!(result4.is_err());
        assert_eq!(
            result4.unwrap_err().status_code(),
            JsonError::UnsupportedMediaType("".to_string()).status_code()
        );

        // Test Deserialize error
        let db5 = Data::new(setup_test_db().await);
        let req5 = MaybeJson::Invalid(JsonPayloadError::Deserialize(serde_json::Error::io(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "test deserialize error"),
        )));

        let result5 = query_some(req5, db5, query_success).await;

        assert!(result5.is_err());
        assert_eq!(
            result5.unwrap_err().status_code(),
            JsonError::BadRequest("".to_string()).status_code()
        );
    }
}
