use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listenner = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listenner.incoming() {
        let stream = stream.unwrap();
        println!("Connection establishied!");
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: std::net::TcpStream) -> () {
    // this is create on stack
    let mut buffer = [0; 512];
    // this on the heap
    // let mut buffer = Vec::new();
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let mut file = File::open("h.html").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return ();
    }
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
