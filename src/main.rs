use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

mod health;
mod img;
mod sitemap;

async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    // if not a GET, return 405
    if req.method() != &Method::GET {
        let res = Response::builder()
            .status(&StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap();

        return Ok(res)
    }

    let handler = match req.uri().path() {
        "/health" => health::health,
        "/images" => img::img,
        "/sitemap.xml" => sitemap::sitemap,
        _ => |r: Request<Body>| -> Result<Response<Body>, Error> {
            let res = Response::builder()
                .status(&StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();

            Ok(res)
        }
    };

    handler(req)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| {
        async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(router))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
