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
    let mut listener = TcpListener::bind(addr).await?;

    println!("listening on port {}...", port);
    loop {
        let (socket, _) = listener.accept().await?;
        let mut request = String::new();
        let mut buffed_socket = tokio::io::BufReader::new(socket);
        buffed_socket.read_line(&mut request).await?;
        println!("request: {}", request);
        buffed_socket
            .write_all(b"HTTP/1.1 200\n\n<h1>Hello!</h1>")
            .await?;
        println!("sent response\n\n\n");
    }
}
