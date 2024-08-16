use axum::{http::StatusCode, response::IntoResponse};


pub enum AppError {
    SurrealDbError(surrealdb::Error),
    NotFound,
    WordAlreadyExists,
    WordNotFound
}

impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::SurrealDbError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::SurrealDbError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            },
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            }
            AppError::WordNotFound => {
                (StatusCode::BAD_REQUEST, "Word not found in list").into_response()
            }
            AppError::WordAlreadyExists => {
                (StatusCode::BAD_REQUEST, "Word already exists").into_response()
            }
        }
    }
}
