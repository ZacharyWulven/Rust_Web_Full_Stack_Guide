#[path = "../mod.rs"]
mod wa;   // 定义 mod 模块为 wa

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use wa::{errors, handlers, models, routers};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Listening on: {}", &host_port);

    HttpServer::new(move || {
        // CARGO_MANIFEST_DIR 就是 webapp 的目录地址
        // 这里即从 CARGO_MANIFEST_DIR/static/ 下寻找静态文件
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        // 注册路由 routers::app_config 函数
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}

