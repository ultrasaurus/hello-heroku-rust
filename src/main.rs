// example code from:
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpListener.html
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpStream.html
use std::env;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get the port number to listen on (required for heroku deployment).
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("127.0.0.1:{}", port);
    let mut listener = TcpListener::bind(addr).await.unwrap();

    println!("listening on port {}...", port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let mut request = String::new();
        let mut buffed_socket = tokio::io::BufReader::new(socket);
        let read_result = buffed_socket.read_line(&mut request).await;
        if let Err(e) = read_result {
            println!("failed to read from socket, err: {}", e);
        } else {
            println!("request: {}", request);
            let write_result = buffed_socket
                .write_all(b"HTTP/1.1 200\n\n<h1>Hello!</h1>")
                .await;
            if let Err(e) = write_result {
                println!("failed to write, err: {}", e);
            }
        }
    }
}
