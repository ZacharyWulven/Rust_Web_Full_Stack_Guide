
/*
    main 调用 server，
    server 中会调用 router，
    router 中会调用 handler
*/

mod server;
mod router;
mod handler;

use server::Server;


fn main() {
    println!("Hello, world!");
    let server = Server::new("localhost:3000");
    server.run();
}
