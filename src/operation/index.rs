// стр. 81 - закончил читать на Option
use actix_web::{get, web, Result};
use serde::Serialize;
use super::AppState;

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize, 
    request_count: usize,
    message: Vec<String>,
}

#[get("/")]  
async fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
// стр. 70
    let requst_count = state.request_count.get() + 1;
    state.request_count.set(requst_count);

    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count: requst_count,
        message: ms.clone(),
    }))
}


/*

    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v
            .to_str()
            .ok())
        .unwrap_or_else(|| "world");
 */