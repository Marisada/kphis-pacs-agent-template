use axum::{
    http,
    response::{IntoResponse, Response},
};
use serde_derive::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest,
    HttpError(http::Error),
}

impl AppError {
    pub fn bad_request() -> Self {
        Self::BadRequest
    }
}

impl IntoResponse for AppError {

    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match &self {
            Self::BadRequest => (http::StatusCode::BAD_REQUEST, String::from("Bad Request")),
            Self::HttpError(http_error) => (http::StatusCode::INTERNAL_SERVER_ERROR, http_error.to_string()),
        };

        let response = (status, AppJson(ErrorResponse { message })).into_response();

        response
    }
}

impl From<http::Error> for AppError {
    fn from(http_error: http::Error) -> Self {
        Self::HttpError(http_error)
    }
}

struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}