use http::Error;
use hyper::{Request, Body, Response};
use hyper::service::Service;
use std::task::{Context, Poll};
use std::future::Future;

pub struct Router {
    dir: String
}

impl Service<Request<Body>> for Router {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Box<Future<Item = Response<Body>, Error = hyper::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        unimplemented!()
    }
}