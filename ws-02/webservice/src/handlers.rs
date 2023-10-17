use super::state::AppState;
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

use super::models::Course;
use chrono::Utc;

// 经测试，app_state 与 new_course 顺序可互换
pub async fn new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>, 
) -> HttpResponse {
    println!("Received new course");

    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()  // 变成便利器
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>() // 变成 Vector
        .len();

    let new_course = Course {
        teacher_id: new_course.teacher_id, // 传进来的 id
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()), // 取当前时间
    };
    // 加入新课程到集合中
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}

// GET localhost:3000/courses/{user_id}
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>,
) -> HttpResponse {
    // Path 里是一个元组，元组就一个元素，类型是 usize
    let teacher_id: usize = params.0;

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id) 
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for teacher".to_string())
    }
}

// GET localhost:3000/courses/{user_id}/{course_id}
// Path 上参数有俩，即 user_id 和 course_id
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.0;
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
        // 调用 ok_or 将 Option<T> 类型转为 Result<T,E> 类型
        // 如果 Option<T> 中有值就返回 Ok，否则返回 Err
        .ok_or("Course not found"); 

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    // 通常测试写个 test 就行了，但这里是 async 的所以需要用 actix_rt 异步运行时
    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(
            Course {
                teacher_id: 1,
                name: "Test Course".into(), // 用 to_string() 也行
                id: None,
                time: None,
            }
        );
        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                courses: Mutex::new(vec![]),
            }
        );
        let resp = new_course(app_state, course).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                courses: Mutex::new(vec![]),
            }
        );
        // Path::from 创建 id 为 1 的课程
        let teacher_id: web::Path<(usize)> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                courses: Mutex::new(vec![]),
            }
        );
        let params: web::Path<(usize, usize)> = web::Path::from((1,1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }


}