use hyper::{Request, Body, Response, Error};

pub fn sitemap(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::empty()))
}