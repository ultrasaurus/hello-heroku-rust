// example code from:
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpListener.html
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpStream.html
use std::env;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::stream::StreamExt;

// use std::io::Cursor;

async fn process_socket(socket: TcpStream) {
    let mut buffed_socket = BufReader::new(socket);
    //let mut request = Vec::new();
    //let mut request;
    let mut lines = Vec::new();
    // let cursor = Cursor::new(buffed_socket);
    
    buffed_socket.lines().map(|result| 
        if let Err(e) = result {
            println!("failed to read from socket, err: {}", e);
            return;
        } else {
            let line = result.unwrap();
            lines.push();
        }
    );

        res.unwrap()
    );


    // let request_str = "foo".to_string();
    // //let request_str = String::from_utf8_lossy(&request);
    // println!("request: {}", request_str);
    // let write_result = buffed_socket
    //     .write_all(b"HTTP/1.1 200\n\n<h1>Hello!</h1>")
    //     .await;
    // if let Err(e) = write_result {
    //     println!("failed to write, err: {}", e);
    // }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get the port number to listen on (required for heroku deployment).
    let port = env::var("PORT").unwrap_or_else(|_| "1234".to_string());

    let addr = format!("0.0.0.0:{}", port);
    let mut listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on port {}...", port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}
