use crate::http::{request, response, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Write;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        'mainloop: loop {
            match listener.accept() {
                Err(err) => {
                    println!("Failed to establish a connection: {}", err);
                    continue 'mainloop;
                }
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 1024] = [0; 1024];

                    match stream.read(&mut buf) {
                        Err(err) => {
                            println!("Failed to read from connection: {}", err);
                            continue 'mainloop;
                        }
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Err(err) => {
                                    println!("Failed to parse request, {}", err);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        crate::http::StatusCode::Ok,
                                        Some("<h1>Hello world!</h1>".to_string()),
                                    )
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed tro send response {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}
