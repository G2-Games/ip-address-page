use maud::{html, Markup};
use axum::{
    extract::Request, http::{HeaderMap, HeaderName, HeaderValue, StatusCode}, routing::get, Router
};

#[tokio::main]
async fn main() {
    let ip = Router::new()
        .route("/", get(get_ip))
        .route("/pretty", get(get_ip_pretty))
        .route("/stylesheet.css", get(stylesheet));

    let app = Router::new()
        .nest("/ip", ip);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8990").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_ip(req: Request) -> Result<String, StatusCode> {
    let ip_addr = ip_from_header(req.headers());

    if let Some(ip) = ip_addr {
        Ok(ip.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_ip_pretty(req: Request) -> Result<Markup, StatusCode> {
    let ip_addr = ip_from_header(req.headers());

    Ok(html! {
        head {
            link rel="stylesheet" href="https://g2games.dev/assets/main-style.css" {}
            link rel="stylesheet" href="stylesheet.css" {}
            meta name="viewport" content="width=device-width, initial-scale=1" {}
        }
        h1 { "Your IP is:" }
        p { (ip_addr.unwrap_or("Unknown")) }
    })
}

fn ip_from_header(header: &HeaderMap<HeaderValue>) -> Option<&str> {
    if let Some(h) = header.get(HeaderName::from_static("cf-connecting-ip")) {
        Some(h.to_str().unwrap())
    } else if let Some(h) = header.get(HeaderName::from_static("x-forwarded-for")) {
        Some(h.to_str().unwrap())
    } else if let Some(h) = header.get(HeaderName::from_static("x-real-ip")) {
        Some(h.to_str().unwrap())
    } else {
        None
    }
}

async fn stylesheet() -> String {
    include_str!("style.css").to_string()
}
