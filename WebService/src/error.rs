use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum SelfDefinedError {
    DBError(String),
    ActixError(String),
    #[allow(dead_code)]
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct SelfDefinedResponse {
    error_msg: String,
}

impl SelfDefinedError {
    fn error_response(&self) -> String {
        match self {
            SelfDefinedError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            SelfDefinedError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            SelfDefinedError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for SelfDefinedError {
    fn status_code(&self) -> StatusCode {
        match self {
            SelfDefinedError::DBError(_msg) | SelfDefinedError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            SelfDefinedError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(SelfDefinedResponse {
            error_msg: self.error_response(),
        })
    }
}

impl fmt::Display for SelfDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for SelfDefinedError {
    fn from(err: actix_web::error::Error) -> Self {
        SelfDefinedError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for SelfDefinedError {
    fn from(err: SQLxError) -> Self {
        SelfDefinedError::DBError(err.to_string())
    }
}