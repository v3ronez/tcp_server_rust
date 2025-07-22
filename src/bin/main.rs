use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
    thread,
};

use tcp_server::ThreadPool;

fn main() {
    let listenner = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(5);
    for stream in listenner.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Connection establishied!");
        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down.");
}
fn handle_connection(mut stream: std::net::TcpStream) -> () {
    // this is create on stack
    let mut buffer = [0; 512];
    // this on the heap
    // let mut buffer = Vec::new();
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "h.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
