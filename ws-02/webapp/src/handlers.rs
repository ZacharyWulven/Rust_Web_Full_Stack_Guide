use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;


// 获得所有的老师，这个页面显示一个列表
pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 创建一个 http 客户端,用它可访问之前的 webservice
    let awc_client = awc::Client::default();

    // 访问 webservice
    let res = awc_client
        .get("http://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        // 可理解为泛型明确写为 Vec<TeacherResponse>，结果就是 Teacher 集合，这里转为 TeacherResponse 类型
        .json::<Vec<TeacherResponse>>() 
        .await
        .unwrap();


    // 使用上下文可向 html 模板中添加数据
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");      // 添加 error 数据为 空

    // 添加 teachers 数据为 上边返回的 Vec<TeacherResponse>
    // 即对应 html 中 {% for t in teachers %} 的 teachers
    ctx.insert("teachers", &res); 

    // 开始渲染模板，在 ./static 下找 teachers.html
    let s = tmpl.render("teachers.html", &ctx)
                .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");      // 添加 error 数据为 空
    ctx.insert("current_name", "");     
    ctx.insert("current_image_url", "");     
    ctx.insert("current_profile", "");      

    let s = tmpl.render("register.html", &ctx)
                .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave" { 
        ctx.insert("error", "Dave has already existed!");      // 添加 error 数据为 空
        ctx.insert("current_name", &params.name);     
        ctx.insert("current_image_url", &params.image_url);     
        ctx.insert("current_profile", &params.profile);
        s = tmpl.render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    } else {
        // json 字段要跟 webservice 中新增老师的字段名对应上 即 CreateTeacher
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.image_url,
            "profile": &params.profile
        });

        let awc_client = awc::Client::default();

        // 访问 webservice
        let res = awc_client
            .post("http://localhost:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            // 可理解为泛型明确写为 Vec<TeacherResponse>，结果就是 Teacher 集合，这里转为 TeacherResponse 类型
            .body() 
            .await?;

        let resp: TeacherResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = format!("Congratulations! You id is {}.", resp.id);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))

}

