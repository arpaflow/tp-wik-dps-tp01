use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::env;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap();
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    println!("Server running on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 || parts[1] != "/ping" {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        return;
    }
    let headers = parse_headers(&request);
    let headers_json = headers.iter()
        .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
        .collect::<Vec<_>>()
        .join(", ");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{{{}}}", headers_json);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_headers(request: &str) -> Vec<(String, String)> {
    let headers: Vec<&str> = request.split("\r\n\r\n").collect();
    let headers = headers[0];
    let mut headers_vec = Vec::new();
    for line in headers.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            headers_vec.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    headers_vec
}