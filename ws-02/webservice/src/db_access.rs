use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use super::errors::MyError;

/*
    本函数用于读取老师的课
    PgPool 就是数据库连接池

*/
pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    // query! 就是 format SQL 语句
    // SQL 语句涉及多行，前边加 r 即可写多行, 格式 r#"{ SQL query content}"#
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = $1"#, 
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    // 改为使用 ?，如遇到错误就返回 MyError
    // .unwrap();

    // 这里要声明一下 Vec<Course>
    let courses:Vec<Course> = rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: Some(NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();
    
    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }

}
 
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id,
    )
    .fetch_one(pool)
    .await;
    // .unwrap(); 

    if let Ok(row) = row {
        Ok(
            Course {
                id: Some(row.id),
                teacher_id: row.teacher_id,
                name: row.name.clone(),
                time: Some(NaiveDateTime::from(row.time.unwrap())),
            }
        ) 
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let row = sqlx::query!(
        // time 就不写了 因为其在数据库有默认值 就是调用 now() 函数
        r#"INSERT INTO course (id, teacher_id, name)
        VALUES ($1, $2, $3)
        RETURNING id, teacher_id, name, time"#,
        new_course.id,
        new_course.teacher_id,
        new_course.name,
    )
    .fetch_one(pool)
    .await?;
    // .unwrap(); 

    Ok(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: Some(NaiveDateTime::from(row.time.unwrap())),
    })

} 