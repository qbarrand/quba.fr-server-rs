use {
    http::Error,
    hyper::{Body, Request, Response},
    tera::{Tera, Context},
};

const NAME : &str = "sitemap";

// Only compile the template once
lazy_static! {
    static ref TEMPLATE: Tera = {
        let mut t = Tera::default();

        const TEMPLATE_STR: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://quba.fr/</loc>
        <lastmod>{{ last_mod }}</lastmod>
        <changefreq>monthly</changefreq>
        <priority>1.0</priority>
    </url>
</urlset>"#;

        info!("Compiling the sitemap template");

        t.add_raw_template(NAME, TEMPLATE_STR).unwrap();
        t
    };
}

pub fn sitemap(_: Request<Body>) -> Result<Response<Body>, Error> {
    let mut c = Context::new();
    c.insert("last_mod", "never");

    let out = TEMPLATE.render(NAME, &c).unwrap();

    Response::builder()
        .header("Content-Length", out.len())
        .body(Body::from(out))
}