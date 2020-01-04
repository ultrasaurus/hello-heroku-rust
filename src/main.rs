// example code from:
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpListener.html
// https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpStream.html
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        println!("waiting for a request...");
        let (socket, _) = listener.accept().await?;
        println!("accepted socket!");
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
