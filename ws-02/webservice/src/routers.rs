use crate::handlers::{course::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    /*
        service 方法使用 scope 先定义了作用域

        /courses 就是这套的根路径
     */
    cfg
    .service(web::scope("/courses")
    // POST localhost:3000/courses/
    .route("/", web::post().to(post_new_course))
    // GET localhost:3000/courses/{teacher_id}
    .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
    // GET localhost:3000/courses/{teacher_id}/{course_id}
    .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
    .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
    .route("/{teacher_id}/{course_id}", web::put().to(update_course_details))
    );
}