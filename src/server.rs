use crate::http::{ParseError, Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server running at address: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Err(e) => {
                    eprintln!("Failed to establish a connection: {e}");
                }
                Ok((mut stream, client_addr)) => {
                    println!("Client connection established: {}", client_addr.to_string());
                    let mut buf = [0u8; 1024];

                    match stream.read(&mut buf) {
                        Err(e) => eprintln!("Failed to read from connection: {}", e),
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Err(err) => handler.handle_bad_request(&err),
                                Ok(request) => handler.handle_request(&request),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}
