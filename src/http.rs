use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Route<'a> {
    path: &'a str,
    html: &'a str,
}

pub struct HttpServer<'a> {
    routes: Vec<Route<'a>>,
    listener: TcpListener,
    host: String,
}

impl<'a> HttpServer<'a> {
    pub fn new(host: &str) -> Self {
        let listener = TcpListener::bind(host).expect("Failed to bind to host");

        HttpServer {
            routes: Vec::new(),
            listener,
            host: host.to_string(),
        }
    }
    pub fn start(&self) {
        println!("Starting server on host: {}", self.host);
    
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server_clone = self.clone(); // Clone self to move into the closure
                    server_clone.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
    

    fn route_request(&self, request: &str) -> String {
        for route in &self.routes {
            if request.contains(&route.path) {
                return format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    route.html.len(),
                    route.html
                );
            }
        }
        "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string()
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if let Ok(read_bytes) = stream.read(&mut buffer) {
            if read_bytes == 0 {
                return;
            }
            let request = String::from_utf8_lossy(&buffer[..read_bytes]);
            let response = self.route_request(&request);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    pub fn add_route(&mut self, path: &'a str, html: &'a str) {
        self.routes.push(Route { path, html });
    }
}
