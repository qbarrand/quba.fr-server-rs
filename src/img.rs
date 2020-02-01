use hyper::{Request, Response, Body, Error};

pub fn img(req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::empty()))
}