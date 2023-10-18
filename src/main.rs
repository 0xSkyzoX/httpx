mod parse;
mod http;

fn main() {
    let content = parse::parser().unwrap();
    let mut server = http::HttpServer::new("0.0.0.0:5500");
    server.add_route("/", &content);
    server.start()
}
