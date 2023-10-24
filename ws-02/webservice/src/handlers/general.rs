// 处理通用的，这里只有健康检查

use crate::state::AppState;
use actix_web::{web, HttpResponse};

// 任何数据在 Actix 中注册后，在 handler 中就可以将它们注入，形式是 web::Data<AppState>
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    // 这里可直接访问，但类型还是 web::Data<AppState>
    let health_check_response = &app_state.health_check_response;
    // 访问 visit_count 前先调用 lock() 防止其他线程更新这个值
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    // 更新数据，这时已经上锁了，当走完这个函数锁才解开
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
