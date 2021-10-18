use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Error {
    status: StatusCode,
    code: u32,
    message: String,
}

impl Error {
    #[allow(non_snake_case)]
    pub fn InternalServerError(code: u32, message: &str) -> Self {
        Self::new(code, message, StatusCode::INTERNAL_SERVER_ERROR)
    }
    #[allow(non_snake_case)]
    pub fn BadRequest(code: u32, message: &str) -> Self {
        Self::new(code, message, StatusCode::BAD_REQUEST)
    }
    #[allow(non_snake_case)]
    pub fn UnprocessableEntity(code: u32, message: &str) -> Self {
        Self::new(code, message, StatusCode::UNPROCESSABLE_ENTITY)
    }

    pub fn new(code: u32, message: &str, status: StatusCode) -> Self {
        Self {
            status,
            code,
            message: String::from(message),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "code": self.code,
            "message": self.message
        }))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::InternalServerError(500, e.to_string().as_str())
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::InternalServerError(500, e.to_string().as_str())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::InternalServerError(500, e.to_string().as_str())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::BadRequest(400, e.to_string().as_str())
    }
}
