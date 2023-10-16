use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    // 运行 server
    pub fn run(&self) {
        // 通过 TcpListener 绑定到 socket_addr 地址
        let connect_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connect_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();

            /*
                将 buffer 内容转化为 HttpRequest，
                先调用 to_vec 转化为 String
                因为其实现了 From trait 所以可以使用 into 转为 HttpRequest
             */
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            /*
                调用路由器分发给不同的 handler 
             */
            Router::route(req, &mut stream);
        }
    }
}