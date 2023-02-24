use actix_web::{error::{Error, JsonPayloadError, InternalError},
    web, HttpRequest, HttpResponse
    };

use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct PostError {
    server_id: usize,                
    request_count: usize,
    error: String,
}

pub fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
// стр. 82

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let request_count = state.request_count.get() + 1;

    //let extns = req.extensions();                            // эти три строчки из книжки легко
    //let state = extns.get::<web::Data<AppState>>().unwrap(); // заменяются двумя с помощью метода app_data, если
    //let request_count = state.request_count.get() + 1;       // весь их смысл - обновить количество запросов к серверу

    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),   // err.to_string() тоже вроде норм
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}