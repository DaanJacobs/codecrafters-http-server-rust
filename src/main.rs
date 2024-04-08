pub mod http;

use std::thread;
use std::{io::Write, net::TcpListener};

use http::response::HttpResponseBuilder;

use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client_request(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client_request(stream: std::net::TcpStream) {
    let request: HttpRequest = HttpRequest::from_stream(&stream);
    let path = &request.path;

    if path == "/" {
        return_ok(stream)
    } else if path == "/user-agent" {
        return_user_agent(stream, request)
    } else if path.starts_with("/echo/") {
        return_echo(stream, request)
    } else {
        return_not_found(stream)
    }
}

fn return_ok(mut stream: std::net::TcpStream) {
    let response: HttpResponse = HttpResponseBuilder::new().build();
    stream.write_all(&response.as_bytes()).unwrap();
}

fn return_not_found(mut stream: std::net::TcpStream) {
    let response: HttpResponse = HttpResponseBuilder::new()
        .with_status(404, String::from("Not Found"))
        .build();
    stream.write_all(&response.as_bytes()).unwrap();
}

fn return_echo(mut stream: std::net::TcpStream, request: HttpRequest) {
    let path = request.path;
    let body = path.replace("/echo/", "");

    let response: HttpResponse = HttpResponseBuilder::new()
        .with_header(String::from("Content-Type"), String::from("text/plain"))
        .with_body(body)
        .build();
    stream.write_all(&response.as_bytes()).unwrap();
}

fn return_user_agent(mut stream: std::net::TcpStream, request: HttpRequest) {
    let body = request.headers.get("User-Agent").unwrap();

    let response: HttpResponse = HttpResponseBuilder::new()
        .with_header(String::from("Content-Type"), String::from("text/plain"))
        .with_body(body.to_string())
        .build();
    stream.write_all(&response.as_bytes()).unwrap();
}
