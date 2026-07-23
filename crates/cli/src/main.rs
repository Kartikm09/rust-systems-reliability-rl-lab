#![forbid(unsafe_code)]

use service::Engine;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| "demo".to_owned());
    match command.as_str() {
        "demo" => demo(),
        "serve" => serve(),
        _ => {
            eprintln!("usage: event-lab [demo|serve]");
            std::process::exit(64);
        }
    }
}

fn demo() {
    let scheduler = Arc::new(scheduler::Scheduler::new(8));
    let store = Arc::new(storage::MemoryStore::default());
    let engine = Engine::new(Arc::clone(&scheduler), store);
    match engine.ingest(b"42:4:demo") {
        Ok(id) => println!("accepted job {id}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn serve() {
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "8082".to_owned());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).expect("bind health server");
    println!("event lab listening on {port}");
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request);
        let body = "{\"status\":\"ok\"}\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes());
    }
}
