use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 配置 route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    /*
        设置路由 path 为 /health
        web::get() 表示使用 http get，
        to 中函数 health_check_handler 就是对应的 Handler
     */
    cfg.route("/health", web::get().to(health_check_handler));
}

// 配置 Handler

pub async fn health_check_handler() -> impl Responder {
    /*
        返回 Ok Response，并返回 json 为 Actix Web Service is running
        要求返回值实现 Responder trait
     */
    HttpResponse::Ok().json("Actix Web Service is running!")
}
 
// 实例化 HTTP server 并运行
#[actix_rt::main] // 用到 actix_rt 运行时
async fn main() -> io::Result<()> {
    // 构建 app，配置 route 路由，传入 general_routes 函数
    let app = move || App::new().configure(general_routes);

    // 运行 HTTP server
    // new 函数初始化一个 HttpServer，然后绑定到 3000 地址并运行
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}