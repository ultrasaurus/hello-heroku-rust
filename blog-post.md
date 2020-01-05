*DRAFT*

In an effort to understand the new Rust async/await syntax, I made a super-simple app that simply responds to all HTTP requests with **Hello!** and deployed on heroku. 

If you want to skip right to the punchline, the source code and README instructions can be found on [github.com/ultrasaurus/hello-heroku-rust](https://github.com/ultrasaurus/hello-heroku-rust)

## Rust "hello world" app

Make a new project with cargo
```
cargo new hello_rust --bin
cd hello_rust
git init
git add .
git commit -m “cargo new hello_rust —bin”

cargo run
```

output:
```
   Compiling hello_rust v0.1.0 (/Users/sallen/src/rust/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.47s
     Running `target/debug/hello_rust`
Hello, world!
```

## Heroku setup

Rust isn't officially supported by Heroku yet, but there are lots of "buildpacks" which help to deploy a Rust app.  I picked [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) -- most stars, most forks & recently updated!

We need the [heroku CLI](https://devcenter.heroku.com/articles/heroku-cli). I already had it and just did `heroku update` to sync to latest version (`7.35.1`).  Then to set up the app on heroku:

```
heroku create --buildpack emk/rust
```

output provides a unique hostname by default:
```
Creating app... done, ⬢ peaceful-gorge-05620
Setting buildpack to emk/rust... done
https://peaceful-gorge-05620.herokuapp.com/ | https://git.heroku.com/peaceful-gorge-05620.git
```

We need a Procfile so heroku knows our entrypoint
```
echo "web: ./target/release/hello_rust" >> Procfile
```

## Write the app

Add crate dependencies to `Cargo.toml` and add code to `main.rs` (and other files as with any Rust app).  The *emk/rust buildpack* takes care of building everything as part of the heroku deploy.

The following lines (in `Cargo.toml`) will add all of tokio features:

```
[dependencies]
tokio = { version = "0.2", features = ["full"] }
```

I'd rather specify only what's needed, but ran into something I couldn't debug myself ([issue#2050](https://github.com/tokio-rs/tokio/issues/2050))

The core of the app accepts the sockets connections, but doesn't read/write:

```
use std::env;
use tokio::net::TcpListener;

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
                println!("not doing anything yet");
            }
        }
    }
}
```

## Deploy on heroku


The above code will build and deploy, by simply pushing the code to heroku:
```
heroku push origin master
```

We can see what it is doing with `heroku logs --tail`: 

Here's where it starts the build and then kills the old app:
```
2020-01-05T03:45:31.000000+00:00 app[api]: Build started by user ...
2020-01-05T03:45:50.450898+00:00 heroku[web.1]: Restarting
2020-01-05T03:45:50.454311+00:00 heroku[web.1]: State changed from up to starting
2020-01-05T03:45:50.244579+00:00 app[api]: Deploy 399e1c85 by user ...
2020-01-05T03:45:50.244579+00:00 app[api]: Release v24 created by user ...
2020-01-05T03:45:50.701533+00:00 heroku[web.1]: Starting process with command `./target/release/hello_rust`
2020-01-05T03:45:51.741040+00:00 heroku[web.1]: Stopping all processes with SIGTERM
2020-01-05T03:45:51.819864+00:00 heroku[web.1]: Process exited with status 143
```

Oddly, it seems to start the app before "State changed from starting to up" but it will fail if we're not listening on the right port, so maybe that is as expected:

```
2020-01-05T03:45:52.343368+00:00 app[web.1]: listening on port 49517...
2020-01-05T03:45:53.322238+00:00 heroku[web.1]: State changed from starting to up
2020-01-05T03:45:53.303486+00:00 app[web.1]: socket connection accepted, 10.171.202.59:17201
2020-01-05T03:45:53.303545+00:00 app[web.1]: not doing anything yet
2020-01-05T03:45:53.303619+00:00 app[web.1]: listening on port 49517...
2020-01-05T03:45:53.313259+00:00 app[web.1]: socket connection accepted, 172.17.146.217:43686
2020-01-05T03:45:53.313285+00:00 app[web.1]: not doing anything yet
2020-01-05T03:45:53.313370+00:00 app[web.1]: listening on port 49517...
2020-01-05T03:46:28.000000+00:00 app[api]: Build succeeded
2020-01-05T03:46:48.251168+00:00 heroku[router]: at=error code=H13 desc="Connection closed without response" method=GET path="/" host=peaceful-gorge-05620.herokuapp.com request_id=a0d630d9-790a-47db-87af-67e680b27907 fwd="69.181.194.59" dyno=web.1 connect=1ms service=1ms status=503 bytes=0 protocol=https
```

So, the first socket connection above is some internal heroku checker, then when I attempt to go to the app URL in the browser, it fails (as expected).


## Async read and write 

I tried to keep the code clear with as little *magic* as possible.  It's a bit verbose (without even handling HTTP in any general way), but I found it helpful to see the details of read and write.

Below is an `async fn` with the additional dependencies. `tokio::io::BufReader` with the `AsyncBufReadExt` trait allows us to call `read_line` (as well as a host of other helpful methods not used here that are written on top of the basic AsyncRead).  We read the bytes from the socket line by line until we get the the end of the HTTP Request (signalled by a blank line).  So we look for two CLRFs (one at the end of the last header line and one for the blank line).

```
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

async fn process_socket(socket: TcpStream) {
    let mut buffed_socket = BufReader::new(socket);
    let mut request = String::new();
    let mut result;

    loop {
        result = buffed_socket.read_line(&mut request).await;

        if let Ok(num_bytes) = result {
            if num_bytes > 0 {
                if request.len() >= 4 {
                    let end_chars = &request[request.len() - 4..];
                    if end_chars == "\r\n\r\n" {
                        break;
                    };
                }
            }
        }
    }
    if let Err(e) = result {
        println!("failed to read from socket, err: {}", e);
        return;
    }
    println!("request: {}", request);
    let html = "<h1>Hello!</h1>";
    let response = format!(
        "HTTP/1.1 200\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );
    let write_result = buffed_socket.write_all(response.as_bytes()).await;
    if let Err(e) = write_result {
        println!("failed to write, err: {}", e);
    }
}
```

which is called from main with:

```
tokio::spawn(async move { process_socket(socket).await });
```

We need to make sure we can read/write from one socket while also listening for additional connections. `tokio::spawn` will allow the program execution to continue, while concurrently allowing our async function `process_socket` to read and write from the socket.  Because we added `#[tokio::main]` above our `async fn main` entry point, tokio will set up an executor which will wait for all of our spawned tasks to complete before exiting. 


## Background

Here's my environment info (`rustup show`):

```
stable-x86_64-apple-darwin (default)
rustc 1.39.0 (4560ea788 2019-11-04)
```

Reference docs

* https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpListener.html
* https://docs.rs/tokio/0.2.6/tokio/net/struct.TcpStream.html
* https://docs.rs/tokio/0.2.6/tokio/task/fn.spawn.html

