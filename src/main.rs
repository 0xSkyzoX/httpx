mod parse;
mod http;

fn main() {
    let content = parse::parser().unwrap();
    let mut server: http::HttpServer = http::HttpServer::new("127.0.0.1:5050");
    server.add_route("/app", &content);
    server.add_route("/", "Hello World!");
    server.start();
}
