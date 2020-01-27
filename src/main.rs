use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

mod health;

async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => health::health(),
        (&Method::GET, "/images") => Ok(Response::new("Hello, World".into())),
        (&Method::GET, "/sitemap.xml") => Ok(Response::new("Hello, World".into())),
        _ => {
            let res = Response::builder()
                .status(&StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();

            Ok(res)
        }
    }
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(router))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
