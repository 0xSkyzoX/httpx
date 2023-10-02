use std::io::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Route<R, Q> {
    path: &'static str, // Make sure path is a static string
    func: fn(req: &R, res: &mut Q) -> Result<(), Error>,
}

pub struct HttpServer<R, Q> {
    routes: Vec<Route<R, Q>>,
    listener: TcpListener,
    host: String
}

impl<R, Q> HttpServer<R, Q> {
    pub fn new(host: &str) -> Self {

        let listener = TcpListener::bind(host).unwrap();

        HttpServer {
            routes: Vec::new(),
            listener,
            host: host.to_string()
        }
    }

    pub fn start(&self) {
        println!("Starting server on host: {}", self.host);
        let mut buffer: [u8; 1024] = [0; 1024];
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connection!");
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    // fn handle_connection(&self, mut stream: TcpStream) {
    //     let mut buffer: [u8; 1024] = [0; 1024];
    //     stream.read(&mut buffer).unwrap();

    //     let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
    //     let response: String = self.route_request(&request);
    //     let mut body: std::str::Split<'_, &str> = request.split("\r\n");
    //     let body_size: usize = body.clone().count();
    //     if let Some(body_line) = body.nth(body_size-1) {
    //         // This is the body
    //         println!("{}", body_line);
    //     } else {
    //         println!("Second line not found.");
    //     }

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    pub fn add_route(&mut self, path: &'static str, func: fn(req: &R, res: &mut Q) -> Result<(), Error>) {
        self.routes.push(Route { path, func });
    }   
}