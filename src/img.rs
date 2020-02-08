use {
    http::Error,
    hyper::{Request, Response, Body},
};
use std::path::Path;

pub fn img(req: Request<Body>) -> Result<Response<Body>, Error> {
//    let path = dir.join(
//        Path::new(req.uri().path())
//    );
//
//    debug!("Serving {}", path());
    Ok(Response::new(Body::empty()))
}