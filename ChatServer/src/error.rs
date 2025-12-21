// enum to  define all the custom application error types
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Internal(String),
}

// mapping all the errors to http codes and supplied messages
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = axum::Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

// add support for database errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

// add it to the result type
pub type Result<T> = std::result::Result<T, AppError>;

// database errors
pub struct DbErrorConfig {
    pub unique_violations: std::collections::HashMap<String, String>,
    pub foreign_key_message: Option<String>,
    pub check_violation_message: Option<String>,
    pub not_null_message: Option<String>,
}

// set needed custom error messages
impl DbErrorConfig {
    pub fn new() -> Self {
        Self {
            unique_violations: std::collections::HashMap::new(),
            foreign_key_message: None,
            check_violation_message: None,
            not_null_message: None,
        }
    }

    pub fn unique(mut self, constraint_name: &str, message: impl Into<String>) -> Self {
        self.unique_violations
            .insert(constraint_name.to_string(), message.into());
        self
    }

    pub fn foreign_key(mut self, message: impl Into<String>) -> Self {
        self.foreign_key_message = Some(message.into());
        self
    }

    pub fn check_violation(mut self, message: impl Into<String>) -> Self {
        self.check_violation_message = Some(message.into());
        self
    }

    pub fn not_null(mut self, message: impl Into<String>) -> Self {
        self.not_null_message = Some(message.into());
        self
    }
}

// handle the db error and send back the custom string or a default message
pub fn handle_db_error(error: sqlx::Error, config: DbErrorConfig) -> AppError {
    match error {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                if let Some(constraint) = db_err.constraint() {
                    if let Some(custom_msg) = config.unique_violations.get(constraint) {
                        return AppError::BadRequest(custom_msg.clone());
                    }
                }
                return AppError::BadRequest(
                    "A record with this information already exists".to_string(),
                );
            }

            if db_err.is_foreign_key_violation() {
                if let Some(msg) = config.foreign_key_message {
                    return AppError::BadRequest(msg);
                }
                return AppError::BadRequest("Referenced record does not exist".to_string());
            }

            if db_err.is_check_violation() {
                if let Some(msg) = config.check_violation_message {
                    return AppError::BadRequest(msg);
                }
                return AppError::BadRequest(format!("Validation failed: {}", db_err.message()));
            }

            if let Some(code) = db_err.code() {
                if code.as_ref() == "23502" {
                    if let Some(msg) = config.not_null_message {
                        return AppError::BadRequest(msg);
                    }
                    return AppError::BadRequest("Required field is missing".to_string());
                }
            }

            tracing::error!("Unhandled database error: {:?}", db_err);
            AppError::Database(sqlx::Error::Database(db_err))
        }
        other => AppError::Database(other),
    }
}

pub async fn map_db_error<T>(
    fut: impl Future<Output=std::result::Result<T, sqlx::Error>>,
    config: DbErrorConfig,
) -> Result<T> {
    fut.await
        .map_err(|err| handle_db_error(err, config))
}
