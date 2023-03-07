use actix_web::{error::BlockingError, HttpResponse};
use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::{DatabaseError, NotFound}};
use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

impl From<diesel::result::Error> for AppError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(value),
        }
    }
}

impl From<BlockingError> for AppError {
    fn from(_: BlockingError) -> Self {
        AppError::OperationCanceled
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {  // обычно self используется как информация для выведения на экран в трейте Display. Однако при обработке ошибок self лишь варианты, в зависимости от которых выводится ошибка
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse{ err })
    }    
}

