// стр. 81 - закончил читать на Option
use actix_web::{post, web, Result};
use crate::operation::index::IndexResponse;

use super::AppState;

#[post("/clear")]  
async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
// стр. 79
    let requst_count = state.request_count.get() + 1;
    state.request_count.set(requst_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count: requst_count,
        messages: Vec::new(),
    }))
}