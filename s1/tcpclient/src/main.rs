use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;


// 运行 client：cargo run -p tcpclient

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    // 向服务器传输消息
    // 传输时候需要使用原始字节，所以这里用 as_bytes 这个方法
    stream.write("Hello".as_bytes()).unwrap();
    println!("Hello, world!");

    
    // 从服务器接收消息
    let mut buffer = [0; 5];
    // 读取服务器内容到 buffer
    stream.read(&mut buffer).unwrap();

    // 将 buffer 内容转为 utf-8 字符串
    println!("Response from server:{:?}", str::from_utf8(&buffer).unwrap());
}
