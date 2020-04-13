use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(tcp_listener) => {
            println!("* listener is up: \n\t{:?}", &tcp_listener);
            tcp_listener
        }
        Err(err) => {
            println!("failed:{:?}", err);
            panic!("error");
        }
    };

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(tcp_stream) => {
                println!("* connection established!: \n\t{:?}", &tcp_stream);
                tcp_stream
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };

        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(buffer_size) => {
                println!("* read stream \n\tbuffer size: {:?}", &buffer_size);
                buffer_size
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };

        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

        let mut file = match File::open(filename) {
            Ok(file) => {
                println!("* get handler for local file\n\tfile: {:?}", &file);
                file
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };
        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(buffer_size) => {
                println!("* read buffer from local file into string container\n\tbuffer size: {:?}", &buffer_size);
                buffer_size
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };

        let response = format!("{}{}", status_line, contents);

        match stream.write(response.as_bytes()) {
            Ok(buffer_size) => {
                println!("* write response from string into stream\n\tbuffer size: {:?}", &buffer_size);
                buffer_size
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };
        match stream.flush() {
            Ok(t) => {
                println!("* emptied buffer {:?}", &t);
                t
            }
            Err(err) => {
                println!("failed:{:?}", err);
                panic!("error");
            }
        };
    }
    /* (Result.and_then)/
    // Result.and_then 으로 처리를 이어 나가려면 컨텍스트 객체가 반환되어야 함.
        pub fn and_then<U, F>(self, op: F) -> Result<U, E>
        where
        F: FnOnce(T) -> Result<U, E>,
    // (sample)
    fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
    fn err(x: u32) -> Result<u32, u32> { Err(x) }

    assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
    assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
    */
}

