mod routes;
mod models;
mod errors;
mod schema;

use actix_web::{middleware, App, HttpServer, web::Data};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Blog {
    port: u16,
}

impl Blog {  // стр. 106
    pub fn new (port: u16) -> Self {
        Blog { port }
    }

     pub async fn run (&self, database_url: String) -> std::io::Result<()> {

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(pool.clone()))  // всегда нужно оборачивать в Data::new!!!!
                .wrap(middleware::Logger::default())    // возможно, эта штука выкидывает из запроса браузера всё лишнее. 
                .configure(routes::users::config_users)
                .configure(routes::posts::config_posts)
                .configure(routes::comments::config_comments)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}
