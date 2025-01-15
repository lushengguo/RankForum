extern crate rankforum;

use rankforum::db;
use rankforum::field;
use rankforum::post;
use rankforum::score;
use rankforum::user;

use env_logger;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Define the address to bind the server to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a service that handles incoming requests
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    // Create the server
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
