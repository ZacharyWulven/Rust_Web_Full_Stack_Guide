use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; // 序列化，反序列化

// use crate::models::course::Course

// Clone 用于解决所有权相关问题
// 使用 sqlx::FromRow 默认实现就可以将读表的结果自动映射成 Course struct
// 去掉 Deserialize 使其只用于查询，不需要从 json-> Course
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>, // 时间类型

    // 新增字段
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}


// 新增时候用，只添加这些字段就行了
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}


// 实现 From 将 json 格式数据转为 Course
/*
    web::Json<T>、web::Data<T> 都属于叫数据提取器
    这里作用就是可以把 json 格式数据转为 Course 等特定类型的数据
*/
// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course: web::Json<CreateCourse>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     } 
// }

// 怕出错可以实现 try_from trait，但实现 try_from trait 就必须注释掉 from trait
use std::convert::TryFrom;

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    // Self 就是 CreateCourse
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
                teacher_id: course.teacher_id,
                name: course.name.clone(),
                description: course.description.clone(),
                format: course.format.clone(),
                structure: course.structure.clone(),
                duration: course.duration.clone(),
                price: course.price,
                language: course.language.clone(),
                level: course.level.clone(),
            })
    }
}


// 用于修改课程，只能修改以下字段
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse { 
            name: course.name.clone(), 
            description: course.description.clone(), 
            format: course.format.clone(), 
            structure: course.structure.clone(), 
            duration: course.duration.clone(), 
            price: course.price, 
            language: course.language.clone(), 
            level: course.level.clone()
        }
    }
}