use crate::errors::AppError;
use actix_web::HttpResponse;

pub(super) mod users;
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