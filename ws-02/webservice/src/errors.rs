use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

// actix_web::error
// 实现这个 trait 后当错误发生时 actix 就可以把 MyError 中的错误自动转为 HttpResponse
// 实现 error::ResponseError trait 时要求必须实现 Debug 和 Display 这俩 trait
// Display 需要自己实现
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            // 未使用的变量前边加下划线 _ , 消除警告
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

// 可使 actix_web::error::Error 转为 MyError
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

// 可使 SQLxError 转为 MyError
impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}

// impl From<Infallible> for MyError {
//     fn from(err: Infallible) -> Self {
//         MyError::NotFound(err.to_string())
//     }
// }
