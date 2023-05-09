use std::net::TcpListener;
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
                    continue;
                }
                Ok((stream, _)) => {}
            }
        }
    }
}
