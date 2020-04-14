use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;

fn main() {
    run();
}

fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").map(|listener| {
        println!("listening at 7878!!!");
        listener
    }).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.map(|stream| {
            println!("connection established!!!");
            stream
        }).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer).map_err(|error| { panic!("error!!! {:?}", error) });

        let get = b"GET / HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

        let mut file = File::open(filename).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
        let mut contents = String::new();

        let response: String = file.read_to_string(&mut contents).map(|_usize| { format!("{}{}", status_line, contents) }).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();

        let _ = stream.write(response.as_bytes()).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
        let _ = stream.flush().map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
    }
}
