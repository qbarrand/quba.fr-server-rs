use hyper::{Body, Error, Response};
use domain::bits::{DNameBuf};
use domain::core::iana::{Rtype, Class};
use domain::resolv::Resolver;
use tokio::core::reactor::Core;

pub fn health() -> Result<Response<Body>, Error> {
    let mut core = Core::new().unwrap();
    let resolv = Resolver::new(&core.handle());

    let name = DNameBuf::from_str("ping.quba.fr.").unwrap();
    let soa = resolv.query((name, Rtype::Txt, Class::In));

    for record in v4.answer().unwrap() {
        println!("{}", record.unwrap());
    }

    Ok(Response::new(Body::empty()))
}