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

// #[tokio::main]
// async fn main() {
//     let addr = "127.0.0.1:80";
//     let mut listener = TcpListener::bind(addr).await.unwrap();

//     // Here we convert the `TcpListener` to a stream of incoming connections
//     // with the `incoming` method. We then define how to process each element in
//     // the stream with the `for_each` combinator method
//     let server = async move {
//         let mut incoming = listener.incoming();
//         while let Some(socket_res) = incoming.next().await {
//             match socket_res {
//                 Ok(socket) => {
//                     println!("Accepted connection from {:?}", socket.peer_addr());
//                     socket.write_all("<h1>Hello!</h1>")
//                 }
//                 Err(err) => {
//                     // Handle error by printing to STDOUT.
//                     println!("accept error = {:?}", err);
//                 }
//             }
//         }
//     };

//     println!("Server running on localhost");

//     // Start the server and block this async fn until `server` spins down.
//     server.await;
// }
