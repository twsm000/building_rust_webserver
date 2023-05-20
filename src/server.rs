use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                        Ok(_) => println!("Received a request: {}", String::from_utf8_lossy(&buf)),
                    }
                }
            }
        }
    }
}
