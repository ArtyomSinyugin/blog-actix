use super::*;
use crate::{errors::AppError, {models::{self, user_action}, Pool}};
use actix_web::{web, HttpResponse, post};
use futures::{Future, FutureExt};

// на стр. 123 какая-то хрень
#[post("/users")]
async fn create_user(new_user: web::Json<UserInput>, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {  // здесь экспериментирую, чтобы сделать функцию асинхронной
    println!("Создаём пользователя!");
    web::block(move || {
        let conn = &mut pool.get().expect("Пул create_user не запустился...");
        let username = new_user.into_inner().username;
        println!("{}", username);
        user_action::create_user(conn, username.as_str())
    })
    .then(|res| async move {convert(res)})
    .await

    // здесь должен использоваться .then из крейта futures. Но я на него как-то забил. Зачем он нужен?
}

fn find_user(name: web::Path<String>, pool: web::Data<Pool>) -> impl Future<Output = Result<HttpResponse, AppError>> {   // экспериментирую, чтобы возвращать фьючерс в фабрику app
    println!("Ищем пользователя!");
    web::block(move|| {
        let conn = &mut pool
            .get()
            .expect("Пул find_user не запустился...");
        let name = name.into_inner();
        let key = models::UserKey::Username(name.as_str());
        user_action::find_user(conn, key)
    })
    .then(|res| async move {convert(res)})
}

fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Future<Output = Result<HttpResponse, AppError>>  {
    println!("Ищем по ID");
    web::block(move|| {
        let conn = &mut pool.get().expect("Пул get_user не запустился...");           // сделал мутабельной для исправления ошибки в find_user на свой риск
        let id = user_id.into_inner();
        let key = models::UserKey::ID(id);
        user_action::find_user(conn, key)
    })
    .then(|res| async move {convert(res)})
}

pub fn config_users(cfg: &mut web::ServiceConfig) {
    cfg   
        .service(create_user)
        .service(
            web::resource("/users/{user_id}").route(web::get().to(get_user)))
        .service(
            web::resource("/users/find/{name}").route(web::get().to(find_user)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::read_body_json;
    use actix_web::{test, App, web};
    use serde_json::json;
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};
    use dotenvy::dotenv;
    use std::env;

    #[actix_web::test]
    async fn check_create_user () {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL не установлена");

        let request_body = json!({
            "username": "Mikael",
        });

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(create_user)).await;

        let resp = test::TestRequest::post()
            .set_json(&request_body)
            .uri("/users")
            .send_request(&mut app)
            .await;

        let user: UserInput = read_body_json(resp).await;

        assert_eq!(user.username, "Mikael");       

    }
}