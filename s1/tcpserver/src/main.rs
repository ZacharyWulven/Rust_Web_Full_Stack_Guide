use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // 绑定到 3000 端口
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // 只接受一次请求使用 accept，而实际很少用，因为要持续监听进来的连接
    // let result = listener.accept().unwrap();

    /*
        incoming 返回迭代器，迭代器它就会监听 listener 接收到的连接
        而每个连接就代表接收到的字节流，这个字节流的类型就是 TcpStream
        数据就可以在 TcpStream 上传输和接收
        对 TcpStream 的读写是使用原始字节来完成的

        运行：cargo run -p tcpserver
     */
    for stream in listener.incoming() {
        // 使用 unwrap 简单处理下，如果没 err 就返回 stream，否则 panic
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }

}
