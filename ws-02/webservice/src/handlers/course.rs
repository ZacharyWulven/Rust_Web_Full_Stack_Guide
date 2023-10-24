use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::course::*;
use crate::models::course::{CreateCourse, UpdateCourse};
// use chrono::Utc;

// 经测试，app_state 与 new_course 顺序可互换
pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>, 
) -> Result<HttpResponse, MyError> {
    // println!("Received new course");

    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()  // 变成便利器
    //     .filter(|course| course.teacher_id == new_course.teacher_id)
    //     .collect::<Vec<Course>>() // 变成 Vector
    //     .len();

    // let new_course = Course {
    //     teacher_id: new_course.teacher_id, // 传进来的 id
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()), // 取当前时间
    // };
    // // 加入新课程到集合中
    // app_state.courses.lock().unwrap().push(new_course);

    // new_course.into() 转为 Course 类型
    // CreateCourse 实现 try_from 所以这里应该改 into 为 try_into,
    // try_into()? 的 ? 表示转换时候可能出错
    // let c = new_course.try_into()?;
    post_new_course_db(&app_state.db, new_course.try_into()?)
    .await
    .map(|course| HttpResponse::Ok().json(course))
    
}

// GET localhost:3000/courses/{teacher_id}
// params: web::Path<(usize,)> 参数改为 web::Path(teacher_id): web::Path<i32> 等于提取了 teacher_id
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    //  web::Path<teacher_id: i32): web::Path<i32>,
    params: web::Path<i32>, // 这里应该是个元组，即 (usize,)
) -> Result<HttpResponse, MyError> {
    // Path 里是一个元组，元组就一个元素，类型是 usize
    // let teacher_id: usize = params.0;

    // let filtered_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == teacher_id) 
    //     .collect::<Vec<Course>>();

    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    //     HttpResponse::Ok().json("No courses found for teacher".to_string())
    // }
    // 尝试转换 params.0 为 i32
    // let teacher_id = i32::try_from(params.0).unwrap();
    let teacher_id: i32 = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
    .await
    .map(|courses|
        HttpResponse::Ok().json(courses)
    )
    // 上边发送错误的话 类型就是 MyError
    // 由于 MyError 实现了 error::ResponseError，所以 actix 会自动转为 HttpResponse 返回给用户

}

// GET localhost:3000/courses/{user_id}/{course_id}
// Path 上参数有俩，即 user_id 和 course_id
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    // web::Path((teacher_id, course_id)): web::Path<(i32, i32)>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    // let (teacher_id, course_id) = params.0;
    // let selected_course = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
    //     // 调用 ok_or 将 Option<T> 类型转为 Result<T,E> 类型
    //     // 如果 Option<T> 中有值就返回 Ok，否则返回 Err
    //     .ok_or("Course not found"); 

    // if let Ok(course) = selected_course {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     HttpResponse::Ok().json("Course not found".to_string())
    // } 

    // let teacher_id = i32::try_from(params.0).unwrap();
    // let course_id = i32::try_from(params.1).unwrap();
    let (teacher_id, course_id) = params.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|course| 
        HttpResponse::Ok().json(course)
    )
}


pub async fn delete_course(
    app_state: web::Data<AppState>,
    // web::Path((teacher_id, course_id)): web::Path<(i32, i32)>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();

    // UpdateCourse 实现了 from trait，所以这里调用 update_course.into()
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}




#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, ResponseError};
    use chrono::Datelike;
    use std::sync::Mutex;
    // use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    // 用于忽略这个测试
    // #[ignore]
    // 通常测试写个 test 就行了，但这里是 async 的所以需要用 actix_rt 异步运行时
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );

        // 本测试只能跑一次，因为跑完数据库里会有 id=3 的条目
        let course = web::Json(
            CreateCourse {
                teacher_id: 1,
                name: "Brand New Course".into(), // 用 to_string() 也行
                description: Some("This is a course".into()),
                format: None,
                structure: None,
                duration: None,
                price: None,
                language: Some("en".into()),
                level: Some("Beginner".into()),
                // id: Some(4), // serial 类型，需要赋一个值
                // time: None,
            }
        );

        let resp = post_new_course(app_state, course).await.unwrap();
        println!("{:?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );
        // Path::from 创建 id 为 1 的课程
        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );
        let params: web::Path<(i32, i32)> = web::Path::from((1,6));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );
        let params: web::Path<(i32, i32)> = web::Path::from((1,100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong ..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );

        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };

        let params: web::Path<(i32, i32)> = web::Path::from((1, 5));
        let update_param = web::Json(update_course);
        let resp = update_course_details(app_state, update_param, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );

        let params: web::Path<(i32, i32)> = web::Path::from((1, 7));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not setup");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(
            AppState {
                health_check_response: "".to_string(),
                visit_count: Mutex::new(0),
                db: db_pool,
            }
        );

        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    } 


}
