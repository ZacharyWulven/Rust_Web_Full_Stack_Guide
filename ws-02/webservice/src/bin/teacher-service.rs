use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions; // 数据库的连接池


// 定义模块指明路径, 声明模块
#[path = "../handlers.rs"]
mod handlers;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

#[path = "../db_access.rs"]
mod db_access;


// 引入 routers 模块所有内容
use routers::*;
use state::AppState;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not setup");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(
        // 初始化 AppState
        AppState {
            health_check_response: "I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,       // 使用数据库改动
            // courses: Mutex::new(vec![]),
        }
    );

    // 这个闭包就是创建应用
    /*
        app_data(shared_data.clone()) 就是 把 shared_data 注册到 web 应用，
        这时就可以向 handler 中注入数据了

        configure(general_routes) 即配置它的路由
     */
    let app = move || {
        App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)  // 添加路由注册，general_routes 就是  routers 里的方法
        .configure(course_routes)
    };

    HttpServer::new(app).bind("localhost:3003")?.run().await
}