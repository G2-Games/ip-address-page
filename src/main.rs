use rocket::{outcome::Outcome, request::{self, FromRequest, Request}, response::content::RawCss};
use maud::{html, Markup};

#[macro_use] extern crate rocket;

#[get("/")]
fn ipaddr(addr: RealIP) -> String {
    addr.0
}

#[get("/pretty")]
fn pretty(addr: RealIP) -> Markup {
    html! {
        head {
            link rel="stylesheet" href="https://g2games.dev/assets/main-style.css" {}
            link rel="stylesheet" href="stylesheet.css" {}
            meta name="viewport" content="width=device-width, initial-scale=1" {}
        }
        h1 { "Your IP is:" }
        p { (addr.0) }
    }
}

#[get("/stylesheet.css")]
fn style() -> RawCss<&'static str> {
    RawCss(include_str!("style.css"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/ip", routes![
            ipaddr,
            pretty,
            style
        ])
}

struct RealIP(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RealIP {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let ip = req.remote();

        if let Some(h) = req.headers().get_one("CF-Connecting-IP") {
            Outcome::Success(RealIP(h.to_string()))
        } else if let Some(h) = req.headers().get_one("x-forwarded-for") {
            Outcome::Success(RealIP(h.to_string()))
        } else if let Some(h) = ip {
            let ip = h.ip();
            Outcome::Success(RealIP(ip.to_string()))
        } else {
            Outcome::Error((rocket::http::Status::from_code(404).unwrap(), ()))
        }
    }
}
