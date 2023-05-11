use crate::http::request::ParseError;
use crate::http::{request, response, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Write;
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request, {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
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
                                Ok(request) => {
                                    handler.handle_request(&request)
                                    //    dbg!(request);
                                    //    Response::new(
                                    //        crate::http::StatusCode::Ok,
                                    //        Some("<h1>Hello world!</h1>".to_string()),
                                    //    )
                                }
                                Err(err) => handler.handle_bad_request(&err),
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
