use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default;
use std::env;
use std::fs;


pub trait Handler {
    // handle 用于处理外部请求
    fn handle(req: &HttpRequest) -> HttpResponse;

    /*
        当 crate 编译时，cargo 会暴露一些环境变量，CARGO_MANIFEST_DIR 就是其中一个
        
        编译时 cargo 还会设置另一些环境变量就需要使用 env::var 
     */
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        println!("CARGO_MANIFEST_DIR={}", default_path);
        // 如果环境变量 PUBLIC_PATH 值有就取这个值，否则就取 CARGO_MANIFEST_DIR 的值
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        // println!("PUBLIC_PATH={}", env::var("PUBLIC_PATH").unwrap());

        let full_path = format!("{}/{}", public_path, file_name);

        // 将文件中内容读取到 contents 中
        let contents = fs::read_to_string(full_path);
        // 调用 ok() 把 Result<T, E> 转化为 Option<T>
        contents.ok()
    }


}

pub struct PageNotFoundHandler;
pub struct StaticPageHandler;
pub struct WebServiceHandler;


// 表示订单状态，业务逻辑
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        // 返回 404 响应，加载 404 页面
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            // localhost:3000/ 或 localhost:3000
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            // localhost:3000/health
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            // 其他路径,尝试读取路径可能对应的文件
            path => match Self::load_file(path) {
                // contents 是文件的内容
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") { // 如果是 .css 文件就添加 header field
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                },
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        println!("CARGO_MANIFEST_DIR={}", default_path);
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        // println!("data_path={}", env::var("DATA_PATH").unwrap());
        // 读取 orders.json 文件，将其内容转为 Vec<OrderStatus>
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        // localhost:3000/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            },
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
 