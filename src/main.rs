// example code from:
// https://hyper.rs/guides/server/hello-world/
// https://github.com/emk/rust-buildpack-example-actix/blob/master/src/main.rs
// https://crates.io/crates/pretty_env_logger
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(
        "<HTML><H1>Hello World!</H1><HTML>",
    )))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // Get the port number to listen on (required for heroku deployment).
    let port = env::var("PORT")
        .unwrap_or_else(|_| "1234".to_string())
        .parse()
        .expect("PORT must be a number");

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on {}", addr);

    server.await?;

    Ok(())
}
