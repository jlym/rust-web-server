use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

use web_server::Semaphore;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();    
    let max_number_of_threads = 2;
    let semaphore = Semaphore::new(max_number_of_threads);

    for stream in listener.incoming() {
        semaphore.enter();      
        let thread_semaphore = semaphore.clone();
        let stream = stream.unwrap();
        thread::spawn(move || {            
            handle_connection(stream);

            thread_semaphore.exit();
        });        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (file_path, status, status_message) = if buffer.starts_with(get) {
        ("hello.html", 200, "OK")
    } else if buffer.starts_with(sleep) { 
        thread::sleep(Duration::from_secs(5));
        ("hello.html", 200, "OK")
    } else {
        ("404.html", 404, "NOT FOUND")
    };

    let body = fs::read_to_string(file_path).unwrap();
    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_message, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

