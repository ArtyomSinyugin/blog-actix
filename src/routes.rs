use crate::errors::AppError;
use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};

pub(super) mod users;
pub(super) mod posts;
pub(super) mod comments;

// стр. 122
fn convert<T, E>(res: Result<Result<T, AppError>, E>) -> Result<HttpResponse, AppError> 
where 
    T: serde::Serialize,
    E: std::fmt::Debug,
    AppError: From<E>,
{
    res.unwrap().map(|d| HttpResponse::Ok().json(d))
    //    .map_err(Into::into)   // строчка, по идее, не нужна, если использовать ранее unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String, 
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String, 
    body: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}