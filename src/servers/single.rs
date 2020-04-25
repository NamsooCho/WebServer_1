use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, ErrorKind};
use std::fs::File;
use std::io;
use std::error::Error;
use regex::Regex;
use std::borrow::Cow;
use std::ops::Deref;

pub fn run_server() -> Result<(), Box<dyn Error>> {
    let result = single_thread_server("127.0.0.1", None);
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

pub fn single_thread_server(domain: &str, port_no: Option<u16>) -> Result<(), io::Error> {
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
        let mut stream: TcpStream = stream.map(|stream| {
            println!("connection established!!!");
            stream
        }).map_err(|error| { panic!("error!!! {:?}", error) }).expect("connection failed");

        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer).map_err(|error| { panic!("error!!! {:?}", error) });
        // /*log/ */ println!("{:?}", String::from_utf8(buffer.to_vec()).unwrap());

        let re = Regex::new(r"GET\s/([a-zA-Z]+)\b").expect("given regular expression is not valid");
        let req = String::from_utf8(buffer.to_vec()).expect("string slice is not valid type(which is UTF-8)");
        // /*log/ */ println!("{}", &req);

        let rel_url = match re.captures(&req).map(|captures| { /* println!("{:?}", captures); */ captures }) {
            Some(t) => { t.get(1).map_or("/", |m| m.as_str()) }
            None => { "/" }
        };
        // /*log/ */println!("\t* relative_url: {}", rel_url);

        let filename = if rel_url == "/" { Cow::from("./resources/servers/index.html") } else { Cow::from(format!("./resources/servers/{}.html", String::from(rel_url))) /*(rel_url.to_string() + "index2.html").as_str()*/ };
        // /*log/ */println!("filename: {}", &filename.deref());
        let (status_line, mut file) = match File::open(filename.deref()) {
            Ok(file) => { ("HTTP/1.1 200 OK\r\n\r\n", file) }
            Err(_) => { ("HTTP/1.1 404 NOT FOUND\r\n\r\n", File::open("./resources/servers/404.html").expect("404.html is not found")) }
        };
        // /*log/ */println!("\t* status_line: {}", status_line);

        let mut contents = String::new();
        let response: String = file.read_to_string(&mut contents)
            .map(|_usize| { format!("{}{}", status_line, contents) })
            .map_err(|error| { panic!("error!!! {:?}", error); }).expect("file is not valid UTF-8");
        // /*log/ */println!("{}", response);
        //
        let _ = stream.write(response.as_bytes()).map_err(|error| { panic!("error!!! {:?}", error) }).expect("failed to write stream");
        let _ = stream.flush().map_err(|error| { panic!("error!!! {:?}", error) }).expect("failed to flush stream");
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
        let result = super::single_thread_server("127.0.0.1", Some(7878)).map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::AddrInUse);
        assert_eq!(expected, result);
        drop(listener);
    }

    #[test]
    fn test_request_succeed_with_200() {
        let _ = thread::spawn(|| {
            let _ = super::single_thread_server("127.0.0.1", Some(8888));
        });
        let res = reqwest::get("http://127.0.0.1:8888");
        assert_eq!(res.unwrap().status().to_string(), "200 OK");
    }

    #[test]
    fn test_request_fail_with_404() {
        let _ = thread::spawn(|| {
            let _ = super::single_thread_server("127.0.0.1", Some(9999));
        });
        let res = reqwest::get("http://127.0.0.1:9999/notExistUrl");
        assert_eq!(res.unwrap().status().to_string(), "404 Not Found");
    }
}

