mod operation;

use crate::operation::{index::*, post::*, clear::*, errors::post_error};

use actix_web::{middleware, App, HttpServer, web};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::cell::Cell;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
static LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

pub struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {  // стр. 53
    pub fn new (port: u16) -> Self {
        MessageApp { port }
    }

     pub async fn run (&self) -> std::io::Result<()> {

        let messages = Arc::new(Mutex::new(Vec::new()));

        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                }))
                .wrap(middleware::Logger::new(LOG_FORMAT))    // возможно, эта штука выкидывает из запроса браузера всё лишнее. 
                .service(index)
                .service(
                    web::resource("/send")
                        .app_data(web::JsonConfig::default()
                            .limit(4096)              // эта штука ограничивает объём данных во время десериализации
                            .error_handler(post_error)) // возвращает ошибку, если запрос не удалось десериализовать   
                        .route(web::post().to(post))
                )
                .service(clear)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}