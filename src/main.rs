use {
    http::Error,
    hyper::service::{make_service_fn, service_fn},
    hyper::{Body, Method, Request, Response, Server, StatusCode},
    std::convert::Infallible,
    std::net::SocketAddr,
    clap::{App, Arg},
};
use std::path::Path;
use hyper::service::Service;

mod health;
mod img;
mod router;
mod sitemap;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    // if not a GET, return 405
    if req.method() != &Method::GET {
        return Response::builder()
            .status(&StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
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
            Response::builder()
                .status(&StatusCode::NOT_FOUND)
                .body(Body::empty())
        }
    };

    handler(req)
}

fn test(dir: String) -> Service {

    let f = async fn() {

    };

    service_fn(f)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    const BIND: &str = "bind";
    const DIR: &str = "dir";

    let matches = App::new("quba.fr")
        .version("0.1")
        .author("Quentin Barrand <quentin@quba.fr>")
        .about("https://quba.fr server")
        .arg(Arg::with_name(BIND)
            .short("b")
            .long(BIND)
            .takes_value(true)
            .default_value("127.0.0.1:8080")
            .required(true  )
        )
        .arg(Arg::with_name(DIR)
            .short("d")
            .long(DIR)
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let addr: SocketAddr = matches.value_of(BIND).unwrap().parse().unwrap();

    info!("Listening on {}", addr.to_string());

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| {
        async {
            // service_fn converts our function into a `Service`
//            Ok::<_, Infallible>(service_fn(router))
            Ok::<_, Infallible>(router::Router {
                dir: String::from("/test")
            })
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
