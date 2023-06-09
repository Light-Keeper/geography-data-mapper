use crate::config::AppConfig;
use crate::db::DbPool;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{Request, Response};

mod api;
mod dto;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[allow(unused_must_use)]
pub async fn start_server(cfg: AppConfig, pool: DbPool) {
    let api = routes![
        api::hello::hello,
        api::datasets::datasets,
        api::datapoints::datapoints
    ];

    rocket::build()
        .attach(Cors)
        .manage(pool)
        .mount("/", FileServer::from(cfg.static_files_dir))
        .mount("/api", api)
        .launch()
        .await;
}
