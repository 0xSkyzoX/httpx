use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Route<'a> {
    path: &'a str, // Make sure path is a static string
    html: &'a str
}

pub struct HttpServer<'a> {
    routes: Vec<Route<'a>>,
    listener: TcpListener,
    host: String
}

impl <'a>HttpServer<'a> {
    pub fn new(host: &str) -> Self {

        let listener: TcpListener = TcpListener::bind(host).unwrap();

        HttpServer {
            routes: Vec::new(),
            listener,
            host: host.to_string()
        }
    }

    pub fn start(&self) {
        println!("Starting server on host: {}", self.host);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream)
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
        return "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string();
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
        let response: String = self.route_request(&request);
        let mut body: std::str::Split<'_, &str> = request.split("\r\n");
        let body_size: usize = body.clone().count();
        if let Some(body_line) = body.nth(body_size-1) {
            // This is the body
            println!("{}", body_line);
        } else {
            println!("Second line not found.");
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn add_route(&mut self, path: &'a str, html: &'a str) {
        self.routes.push(Route { path,  html});
    }   
}