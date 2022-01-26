use derive_more::{Display, Error};

#[derive(Debug, Clone, Display, Error)]
#[display(fmt = "{:?}", self)]
pub struct Error {
    pub code: u32,
    pub message: String,
}

#[allow(non_snake_case)]
pub fn BadRequest<T>(message: &str) -> Result<T, Error> {
    Err(Error {
        code: 400,
        message: message.to_string(),
    })
}

#[allow(non_snake_case)]
pub fn InternalServer<T>(message: &str) -> Result<T, Error> {
    Err(Error {
        code: 500,
        message: message.to_string(),
    })
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error {
            code: 500,
            message: e.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            code: 500,
            message: err.to_string(),
        }
    }
}
