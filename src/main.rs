#![allow(unused_imports)]

use std::io;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn process_socket(mut socket: TcpStream) {
    loop {
        let packet_size = socket.read(&mut [0; 512]).await.expect("Failed to read stream.");
        if packet_size == 0 {
            break;
        }
        socket.write_all(b"+PONG\r\n").await.expect("Failed to write stream.");
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             loop {
    //                 let packet_size = stream.read(&mut [0; 512]).unwrap();
    //                 if packet_size == 0 {
    //                     break;
    //                 }
    //
    //                 stream.write_all(b"+PONG\r\n").unwrap();
    //             }
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}
