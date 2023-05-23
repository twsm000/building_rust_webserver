#![allow(dead_code)]
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}
