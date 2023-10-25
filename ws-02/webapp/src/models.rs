use serde::{Deserialize, Serialize};

// 这些 model 都是给页面用的


// 用于老师注册用
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub image_url: String,
    pub profile: String,
}

// 用于查询老师返回的结果
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}