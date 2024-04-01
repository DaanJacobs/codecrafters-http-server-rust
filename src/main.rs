use std::{io::{self, BufRead, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buffer = io::BufReader::new(&mut stream);
                let line = buffer.lines().next().unwrap().unwrap();
                let path = line.split(' ').nth(1).unwrap();
                
                if path == "/" {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
