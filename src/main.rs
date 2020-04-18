use std::net::TcpListener;
use std::io::{Read, Write, ErrorKind};
use std::fs::File;
use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = run();
    return match result {
        Ok(()) => Ok(()),
        Err(error) => {
            match error.kind() {
                ErrorKind::AddrInUse => {
                    println!("* port 7878 in use!!! \n\trecoverable err : {}", error);
                    // error handling code should follow
                }
                ErrorKind::NotFound => {
                    println!("* file not found : \n\tunrecoverable err : {}", error);
                    // panic code should follow
                    panic!("unrecoverable error: {}", error);
                }
                _ => {
                    panic!("unrecoverable error: {}", error);
                }
            }
            Err(Box::new(error))
        }
    };
}

fn run() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").map(|listener| {
        println!("listening at 7878!!!");
        listener
    });

    for stream in listener?.incoming() {
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

        let file = File::open(filename);
        let mut contents = String::new();
        let response: String = file?.read_to_string(&mut contents).map(|_usize| { format!("{}{}", status_line, contents) }).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();

        let _ = stream.write(response.as_bytes()).map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
        let _ = stream.flush().map_err(|error| { panic!("error!!! {:?}", error) }).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use std::io;
    use std::sync::mpsc;
    use reqwest;

    #[test]
    fn test_address_in_use_error() {
        let listener = TcpListener::bind("127.0.0.1:7878");
        let result = super::run().map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::AddrInUse);
        assert_eq!(expected, result);
        drop(listener);
    }

    #[test]
    fn test_file_not_found_error() {
        thread::sleep(time::Duration::from_millis(2000));
        let (sndr, rcvr) = mpsc::channel();
        let _ = thread::spawn(move || {
            let result = super::run();
            return match result {
                Ok(()) => Ok(()),
                Err(error) => {
                    match error.kind() {
                        ErrorKind::AddrInUse => {
                            let _ = sndr.send("AddrInUse");
                        }
                        ErrorKind::NotFound => {
                            let _ = sndr.send("NotFound");
                        }
                        _ => {
                            let _ = sndr.send("Others");
                        }
                    }
                    Err(Box::new(error))
                }
            };
        });
        let _ = reqwest::get("http://127.0.0.1:7878");
        assert_eq!("NotFound", rcvr.recv().unwrap());
    }
}