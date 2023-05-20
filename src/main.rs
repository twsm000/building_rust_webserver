use http::Request;
use http::Method;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}
