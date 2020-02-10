use {
    http::Error,
    std::path::Path,
    hyper::{Request, Response, Body},
};

pub fn img(dir: &Path, req: Request<Body>) -> Result<Response<Body>, Error> {
    let path = dir.join(
        Path::new(req.uri().path())
    );

    debug!("Serving {}", path.display());
    Ok(Response::new(Body::empty()))
}