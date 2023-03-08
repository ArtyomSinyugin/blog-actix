use crate::errors::AppError;
use crate::routes::convert;
use crate::{models::{post_action, user_action}, Pool};
use actix_web::{web, HttpResponse, get, post};
use futures::FutureExt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String, 
}

#[post("/")]
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

// остановился на стр. 157 (151)