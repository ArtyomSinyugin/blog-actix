use crate::{errors::AppError, routes::convert, Pool, models::comment_action};

use actix_web::{web, HttpResponse, get, post};
use futures::FutureExt;

use super::*;

#[post("")]
async fn add_comment(
    post_id: web::Path<i32>,
    comment: web::Json<CommentInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при добавлении комментария");
        let data = comment.into_inner();
        let user_id = data.user_id;
        let body = data.body;
        comment_action::create_comment(conn, user_id, post_id.into_inner(), body.as_str())
    })
    .then( |res| async {convert(res)})
    .await
}

#[get("")]
async fn post_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при запросе всех комментариев для поста");
        comment_action::post_comments(conn, post_id.into_inner())
    })
    .then(|res| async {convert(res)})
    .await
}

#[get("")]
async fn user_comments(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при запросе всех комментариев для пользователя");
        comment_action::user_comments(conn, user_id.into_inner())
    })
    .then(|res| async {convert(res)})
    .await
}

pub fn config_comments(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users/{id}/comments").service(user_comments))
        .service(web::scope("/posts/{id}/comments")
            .service(add_comment)
            .service(post_comments));
}