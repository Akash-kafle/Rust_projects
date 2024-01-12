use std::fs;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
   
   if buffer.starts_with(get){
    stream.read(&mut buffer).unwrap();

    let content = fs::read_to_string(
        "C:/Users/aakas/OneDrive/Desktop/folders/programming/rust/server/src/index.html",
    )
    .unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();}
    else{
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let content = fs::read_to_string(
            "C:/Users/aakas/OneDrive/Desktop/folders/programming/rust/server/src/404.html",
        )
        .unwrap();
        let response = format!("{}{}", status_line, content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
