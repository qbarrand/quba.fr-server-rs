use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use clap::{App, Arg};

mod health;
mod img;
mod sitemap;

#[macro_use]
extern crate log;
extern crate env_logger;

async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    // if not a GET, return 405
    if req.method() != &Method::GET {
        let res = Response::builder()
            .status(&StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap();

        return Ok(res)
    }

    let path = req.uri().path();

    // If an image, use the image handler
    if path.starts_with("/images/") {
        return img::img(req);
    }

    // Match the remaining routes
    let handler = match path {
        "/health" => health::health,
        "/sitemap.xml" => sitemap::sitemap,
        _ => |_| -> Result<Response<Body>, Error> {
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
    env_logger::init();

    const BIND: &str = "bind";

    let matches = App::new("quba.fr")
        .version("0.1")
        .author("Quentin Barrand <quentin@quba.fr>")
        .about("https://quba.fr server")
        .arg(Arg::with_name(BIND)
            .short("b")
            .long("bind")
            .takes_value(true)
            .default_value("127.0.0.1:8080")
            .required(true  )
        )
        .get_matches();

    let addr: SocketAddr = matches.value_of(BIND).unwrap().parse().unwrap();

    info!("Listening on {}", addr.to_string());

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
