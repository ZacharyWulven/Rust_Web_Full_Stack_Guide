// use std::error::Error;
use crate::models::course::{Course, UpdateCourse, CreateCourse};
// use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use crate::errors::MyError;

/*
    本函数用于读取老师的课
    PgPool 就是数据库连接池

*/
pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    // query! 就是 format SQL 语句
    // SQL 语句涉及多行，前边加 r 即可写多行, 格式 r#"{ SQL query content}"#

    // query_as! 可将结果转为 Vec<Course>, 第一个参数就是 Course，
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1"#, 
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    // 改为使用 ?，如遇到错误就返回 MyError
    // .unwrap();

    // 这里要声明一下 Vec<Course>
    // let courses:Vec<Course> = rows.iter()
    //     .map(|r| Course {
    //         id: Some(r.id),
    //         teacher_id: r.teacher_id,
    //         name: r.name.clone(),
    //         time: Some(NaiveDateTime::from(r.time.unwrap())),
    //     })
    //     .collect();
    
    // match courses.len() {
    //     0 => Err(MyError::NotFound("Courses not found for teacher".into())),
    //     _ => Ok(courses),
    // }

    Ok(rows)

}
 
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id,
    )
    // fetch_optional 表示有可能查询到结果 有可能查询不到
    .fetch_optional(pool)
    .await?;
    // .unwrap(); 

    // 如果查询到结果了 就返回 Ok
    if let Some(course) = row {
        Ok(course) 
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {
    let course = sqlx::query_as!(
        Course,
        // time 就不写了 因为其在数据库有默认值 就是调用 now() 函数
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
    .fetch_one(pool)
    .await?;
    // .unwrap(); 

    Ok(course)
} 

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course = sqlx::query!(
        "DELETE FROM course where teacher_id = $1 and id = $2",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", course))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course = sqlx::query_as!(
        Course,
        "SELECT * FROM course where teacher_id = $1 and id = $2",
        teacher_id,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course.name
    };

    let description: String = if let Some(desc) = update_course.description {
        desc
    } else {
        current_course.description.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course.format.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course.structure.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course.duration.unwrap_or_default()
    };

    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course.level.unwrap_or_default()
    };

    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course.language.unwrap_or_default()
    };

    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course.price.unwrap_or_default()
    };

    let course_row  = sqlx::query_as!(
        Course,
        "UPDATE course SET name = $1, description = $2, format = $3,
        structure = $4, duration = $5, price = $6, language = $7,
        level = $8 where teacher_id = $9 and id = $10
        RETURNING id, teacher_id, name, time,
        description, format, structure, duration, price, language, level",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id,
    ).fetch_one(pool).await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }

}