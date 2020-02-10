use {
    http::Error,
    hyper::service::{make_service_fn, service_fn},
    hyper::{Body, Method, Request, Response, Server, StatusCode},
    std::convert::Infallible,
    std::net::SocketAddr,
    std::path::Path,
    clap::{App, Arg},
};

mod health;
mod img;
mod sitemap;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

async fn router(dir: &Path, req: Request<Body>) -> Result<Response<Body>, Error> {
    // if not a GET, return 405
    if req.method() != &Method::GET {
        return Response::builder()
            .status(&StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
    }

    let path = req.uri().path();

    // If an image, use the image handler
    if path.starts_with("/images/") {
        return img::img(dir, req);
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

#[tokio::main]
async fn main() {
    env_logger::init();

    const BIND: &'static str = "bind";
    const DIR: &'static str = "dir";

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

    let addr_str = matches.value_of(BIND).unwrap();

    let addr: SocketAddr = addr_str.parse().unwrap_or_else(
        |_| panic!("{}: could not parse address", addr_str)
    );

    let dir_str = matches.value_of(DIR).unwrap();
    let dir = Path::new(dir_str);

    info!("Listening on {}", addr.to_string());

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(move |_conn| {
        async {
            let router = |req| router(dir, req);

            // service_fn converts our router into a `Service`
            Ok::<_, Infallible>(service_fn(router))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
