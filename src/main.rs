use std::net::TcpListener;
use std::io::{Read, Write, ErrorKind};
use std::fs::File;
use std::io;
use std::error::Error;
use regex::Regex;
use std::borrow::Cow;
use std::ops::Deref;

fn main() -> Result<(), Box<dyn Error>> {
    let result = run("127.0.0.1", None);
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
                    // panic!("unrecoverable error: {}", error);
                }
                _ => {
                    panic!("unrecoverable error: {}", error);
                }
            }
            Err(Box::new(error))
        }
    };
}

fn run(domain: &str, port_no: Option<u16>) -> Result<(), io::Error> {
    let addr = if let Some(port) = port_no {
        Cow::from(format!("{}:{}", domain, port))
    } else {
        Cow::from(format!("{}:7878", domain))
    };
    // /*log/ */println!("{}", addr);

    let listener = TcpListener::bind(addr.deref()).map(|listener| {
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
        // /*log/ */ println!("{:?}", String::from_utf8(buffer.to_vec()).unwrap());

        let re = Regex::new(r"GET\s/([a-zA-Z]+)\b").unwrap();
        let req = String::from_utf8(buffer.to_vec()).unwrap();
        // /*log/ */ println!("{}", &req);

        let rel_url = match re.captures(&req).map(|captures| { /* println!("{:?}", captures); */ captures }) {
            Some(t) => { t.get(1).map_or("/", |m| m.as_str()) }
            None => { "/" }
        };
        /*log/ */println!("\t* relative_url: {}", rel_url);

        let filename = if rel_url == "/" { Cow::from("index.html") } else { Cow::from(format!("{}.html", String::from(rel_url))) /*(rel_url.to_string() + "index2.html").as_str()*/ };
        // /*log/ */println!("filename: {}", &filename.deref());
        let file = match File::open(filename.deref()) {
            Ok(file) => { Ok(file) }
            Err(_) => { File::open("404.html") }
        };
        let mut contents = String::new();
        let response: String = file?.read_to_string(&mut contents).map(|_usize| { format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents) }).map_err(|error| { panic!("error!!! {:?}", error); }).unwrap();
        // /*log/ */println!("{}", response);

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
        let result = super::run("127.0.0.1", Some(7878)).map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::AddrInUse);
        assert_eq!(expected, result);
        drop(listener);
    }

    #[test]
    #[ignore]
    fn test_file_not_found_error() {
        thread::sleep(time::Duration::from_millis(2000));
        let (sndr, rcvr) = mpsc::channel();
        let _ = thread::spawn(move || {
            let result = super::run("127.0.0.1", Some(7777));
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
        let _ = reqwest::get("http://127.0.0.1:7777/notFound");
        assert_eq!("NotFound", rcvr.recv().unwrap());
    }

    #[test]
    fn test_request_succeed_with_200() {
        thread::sleep(time::Duration::from_millis(2000));
        let _ = thread::spawn(|| {
            let _ = super::run("127.0.0.1", Some(8888));
        });
        let res = reqwest::get("http://127.0.0.1:8888");
        assert_eq!(res.unwrap().status().to_string(), "200 OK");
    }
}

