use crate::errors::AppError;
use crate::routes::convert;
use crate::{models::{post_action, user_action}, Pool};
use actix_web::{web, HttpResponse, get, post};
use futures::FutureExt;
//use diesel::prelude::*; // догадка, что это для сериализации в json или для block

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String, 
    body: String,
}

#[post("/")]
async fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при добавлении поста");
        let key = user_action::UserKey::ID(user_id.into_inner());
        user_action::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;
            post_action::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .then( |res| async {convert(res)})
    .await
}

#[get("/posts/{id}/publish")]
async fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при публикации поста");
        post_action::publish_post(conn, post_id.into_inner())
    })
    .then(|res| async {convert(res)})
    .await
}

#[get("/posts")]
async fn user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при выдёргивании постов пользователя из базы");
        post_action::user_posts(conn, user_id.into_inner())
        // здесь с into_inner, думаю, перестраховка. Он бы привёл значение к типу, который принимает вызываемая функция. Но так как вызываемая функция принимает i32, то здесь без надобности
    })
    .then(|res| async {convert(res)})
    .await
}

#[get("/")]
async fn all_posts(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    web::block(move|| {
        let conn = &mut pool.get().expect("Ошибка соединения при выдёргивании постов пользователя из базы");
        post_action::all_posts(conn)
    })
    .then(|res| async {convert(res)})
    .await
}

pub fn config_posts(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users/{id}/posts")
            .service(add_post)
            .service(user_posts)
            )
        .service(publish_post)
        .service(user_posts);
}
