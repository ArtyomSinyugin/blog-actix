use blog_actix::Blog;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()>{  // стр. 46
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");  // первое - это ключ (key), второе - значение (value). Предположу, что где-то (например, в консоли) написать ключ, то выводится значение
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL не установлена");

    let app = Blog::new(8000);
    app.run(database_url).await
}

// http://127.0.0.1:8000/