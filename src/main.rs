use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let max_threads = 2;
    
    let thing = Arc::new((Mutex::new(max_threads), Condvar::new()));
    let mut count = 0;

    for stream in listener.incoming() {
        count += 1;

        println!("\r\n{} - starting loop", count);
        {
            let &(ref num_threads_mutex, ref cond) = &*thing;
            let mut num_threads = num_threads_mutex.lock().unwrap();
            let mut can_go = *num_threads > 0;
            println!("{} - num_threads_1: {}", count, num_threads);
            println!("{} - can_go_1: {}", count, can_go);
            if can_go {
                *num_threads -= 1;                    
            }

            while !can_go {
                println!("{} - Waiting", count);
                num_threads = cond.wait(num_threads).unwrap();
                println!("{} - Awake", count);

                can_go = *num_threads > 0;
                println!("{} - num_threads_2: {}", count, num_threads);
                println!("{} - can_go_2: {}", count, can_go);
                if can_go {
                    *num_threads -= 1;                    
                }
            }
        }

        let stream = stream.unwrap();        
        let thing2 = thing.clone();        

        let inner_count = count;
        thread::spawn(move || {
            
            println!("{} Handling collection", inner_count);
            handle_connection(stream);
            println!("{} Done!", inner_count);

            let &(ref num_threads_mutex, ref cond) = &*thing2;

            let mut num_threads = num_threads_mutex.lock().unwrap();
            *num_threads += 1;
            cond.notify_one();

        });
        

    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (file_path, status, status_message) = if buffer.starts_with(get) {
        ("hello.html", 200, "OK")
    } else if buffer.starts_with(sleep) { 
        thread::sleep(Duration::from_secs(30));
        ("hello.html", 200, "OK")
    } else {
        ("404.html", 404, "NOT FOUND")
    };

    let body = fs::read_to_string(file_path).unwrap();
    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", status, status_message, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

