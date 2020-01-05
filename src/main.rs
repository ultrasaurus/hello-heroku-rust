// example code from:
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpListener.html
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpStream.html
// https://docs.rs/tokio/0.2.6/tokio/task/fn.spawn.html
use std::env;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    // Get the port number to listen on (required for heroku deployment).
    let port = env::var("PORT").unwrap_or_else(|_| "1234".to_string());

    let addr = format!("0.0.0.0:{}", port);
    let mut listener = TcpListener::bind(addr).await.unwrap();

    loop {
        println!("listening on port {}...", port);
        let result = listener.accept().await;
        match result {
            Err(e) => println!("listen.accept() failed, err: {:?}", e),
            Ok(listen) => {
                let (socket, addr) = listen;
                println!("socket connection accepted, {}", addr);
                // Process each socket concurrently.
                tokio::spawn(async move {
                    let mut buffed_socket = tokio::io::BufReader::new(socket);
                    let mut request = String::new();
                    let mut result;
                    loop {
                        result = buffed_socket.read_line(&mut request).await;
                        if let Ok(num_bytes) = result {
                            if num_bytes > 0 && request.len() >= 4 {
                                let end_chars = &request[request.len() - 4..];
                                if end_chars == "\r\n\r\n" {
                                    break;
                                };
                            }
                        }
                    }
                    if let Err(e) = result {
                        println!("failed to read from socket, err: {}", e);
                        return;
                    }
                    let html = "<h1>Hello!</h1>";
                    println!("request: {}", request);
                    let response = format!(
                        "HTTP/1.1 200\r\nContent-Length: {}\r\n\r\n{}",
                        html.len(),
                        html
                    );
                    let write_result = buffed_socket.write_all(response.as_bytes()).await;
                    if let Err(e) = write_result {
                        println!("failed to write, err: {}", e);
                    }
                });
            }
        }
    }
}
