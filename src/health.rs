use {
    domain::core::iana::{Class, Rtype},
    domain::resolv::Resolver,
    http::Error,
    hyper::{Body, Response, Request},
    tokio_core::reactor::Core,
};

pub fn health(_: Request<Body>) -> Result<Response<Body>, Error> {
//    let mut core = Core::new().unwrap();
//    let resolv = Resolver::new(&core.handle());
//
//    let name = DNameBuf::from_str("ping.quba.fr.").unwrap();
//    let soa = resolv.query((name, Rtype::Txt, Class::In));
//
//    for record in v4.answer().unwrap() {
//        println!("{}", record.unwrap());
//    }

    Ok(Response::new(Body::empty()))
}
