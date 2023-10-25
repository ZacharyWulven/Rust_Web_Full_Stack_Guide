use crate::handlers::*;
use actix_files as fs;
use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            // 所有静态页面都在 static 目录下, /static 是 url 的路径，就会找本地 ./static 路径
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // / 路径会获得所有老师信息
            .service(web::resource("/").route(web::get().to(get_all_teachers)))
            // /register 路径对应显示注册页面的路由
            .service(web::resource("/register").route(web::get().to(show_register_form)))
            // 提交注册表单的路由
            .service(web::resource("/register-post").route(web::post().to(handle_register)))
    );
}