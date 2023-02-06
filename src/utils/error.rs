use rocket::{serde::{Serialize, json::Json}, http::Status};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub message: String
}

impl ErrorResponse {
    pub fn create_error(error_status: Status, message: String) -> (Status, Json<ErrorResponse>) {
        (error_status, Json(ErrorResponse {message}))
    }
}