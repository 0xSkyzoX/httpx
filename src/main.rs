mod parse;
mod http;

struct Resuest {
    body: &'static str,
    path: &'static str
}

struct Response {
    status: &'static str
}

fn handler<R, Q>(req: &R, res: &mut Q) -> Result<(), std::io::Error> {
    
    Ok(()) // Assuming everything went well
}


fn main() {
    let mut server: http::HttpServer<Resuest, Response> = http::HttpServer::new("127.0.0.1:5050");
    server.add_route("/app", handler);

    server.start();
    let content = parse::parser().unwrap();
    println!("{}", content);
}
