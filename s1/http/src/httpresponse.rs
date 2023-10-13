use std::collections::HashMap;
use std::io::{Read, Write, Error};

use crate::httprequest;


// PartialEq 使其成员可与其他值进行比较
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self { 
            version: "HTTP/1.1".into(), 
            status_code: "200".into(), 
            status_text: "OK".into(), 
            headers: None, 
            body: None 
        }
    }   
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let v1 = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}", 
            &v1.version(),
            &v1.status_code(),
            &v1.status_text(),
            &v1.headers(),
            &value.body.unwrap().len(),
            &v1.body()
         )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str, 
        headers: Option<HashMap<&'a str, &'a str>>, 
        body: Option<String>
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "404 Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;
         
        response
    }

    // 接收 tcpstream 为参数并且实现了 Write
    pub fn send_response(&self, write_stream:&mut impl Write) -> Result<(), Error> {
        let res = self.clone();
        let response_string: String = String::from(res);
        // 把字符串发送到 tcpstream
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    } 

    fn headers(&self) -> String { 
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;


    // 测试 new 函数
    #[test]
    fn test_response_struct_creation_200() {
        // 通过 new 函数创建
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxxx".into()),
        );

        // 直接创建
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        // 通过 new 函数创建
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxxx".into()),
        );

        // 直接创建
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "404 Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "404 Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        let http_string: String = response_expected.into();

        let actual_string = "HTTP/1.1 404 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx";

        assert_eq!(http_string, actual_string);

    }

}

