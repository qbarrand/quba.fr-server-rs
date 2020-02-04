use {
    http::Error,
    hyper::{Request, Response, Body},
};

pub fn img(req: Request<Body>) -> Result<Response<Body>, Error> {
//    debug!("Serving images");

    let path = req.uri().path();

    debug!("Serving {}", path);
    Ok(Response::new(Body::empty()))
}