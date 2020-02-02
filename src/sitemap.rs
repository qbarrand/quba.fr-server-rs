use tera::{Template, Tera, Context};
use hyper::{Body, Error, Request, Response};

pub fn sitemap(_: Request<Body>) -> Result<Response<Body>, Error> {
    let mut tera = Tera::default();

    const NAME : &str = "sitemap";

    let template = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://quba.fr/</loc>
        <lastmod>{{ last_mod }}</lastmod>
        <changefreq>monthly</changefreq>
        <priority>1.0</priority>
    </url>
</urlset>"#;

    tera.add_raw_template(NAME, template).unwrap();

    let mut c = Context::new();
    c.insert("last_mod", "never");

    let out = tera.render(NAME, &c).unwrap();

    let res = Response::builder()
        .header("Content-Length", out.len())
        .body(Body::from(out))
        .unwrap();

    Ok(res)
}