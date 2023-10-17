use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; // 序列化，反序列化

// Clone 用于解决所有权相关问题
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>, // 时间类型
}
 
// 实现 From 将 json 格式数据转为 Course
/*
    web::Json<T>、web::Data<T> 都属于叫数据提取器
    这里作用就是可以把 json 格式数据转为 Course 等特定类型的数据
*/
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    } 
}