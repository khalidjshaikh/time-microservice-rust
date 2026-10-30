use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use time_microservice_rust::{build_root_json, VERSION};

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        404 => "Not Found",
        _ => "Error",
    }
}

fn handle_connection(stream: &mut std::net::TcpStream) {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("/");

    let (status, body) = if path == "/" {
        (200u16, build_root_json())
    } else if path == "/healthz" || path == "/readyz" {
        (200u16, "{\"status\":\"ok\"}".to_string())
    } else {
        (404u16, "{\"error\":\"not found\"}".to_string())
    };

    let resp = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        status_text(status),
        body.len(),
        body
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).expect("failed to bind");
    println!("time-microservice {} listening on :{}", VERSION, port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                thread::spawn(move || handle_connection(&mut s));
            }
            Err(e) => {
                eprintln!("connection error: {}", e);
            }
        }
    }
}
