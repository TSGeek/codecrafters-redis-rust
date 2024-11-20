#![allow(unused_imports)]

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                while stream.read(&mut [0; 1]).unwrap() == 1 {
                    stream.write_all(b"+PONG\r\n").unwrap()
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
