use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;


#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 读取 .env 的环境变量，返回 Result，为了防止警告这里调用了 ok()
    // 在生成环境不会使用 .env 设置环境变量
    // 调用 ok() 会把 Result 转为 Option 类型，即使失败了也会被忽略  
    dotenv().ok();

    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not in .env file");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where id = $1"#,
        1,
    ).fetch_all(&db_pool)
    .await
    .unwrap();

    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(
            Course {
                id: row.id,
                teacher_id: row.teacher_id,
                name: row.name,
                time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
            }
        )
    }
    println!("Courses = {:?}", courses_list);
    Ok(())
}