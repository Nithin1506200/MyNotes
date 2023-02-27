use dotenv::dotenv;
use std::fmt::format;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() {
    dotenv().ok();
    let url: String = std::env::var("url").expect("url should be set").to_owned();
    let port: String = std::env::var("port")
        .expect("port should be set")
        .to_owned();

    let listner = TcpListener::bind(format!("{}:{}", url, port)).unwrap();
    println!("served on http://{url}:{port}");
    for stream in listner.incoming() {
        match stream {
            Ok(tcpstream) => handle_connection(tcpstream),
            Err(_) => {
                println!("error")
            }
        }
    }
}
/**
 * this is to handle connection
 */
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let content = fs::read_to_string("./src/index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer));
    println!("Response {}", content);
}
