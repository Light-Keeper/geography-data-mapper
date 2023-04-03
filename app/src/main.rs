mod api;
mod db;
mod models;
mod schema;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use std::env;

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    dotenv::dotenv().ok();
    let pool = db::prepare_database(&env::var("SQLITE_DB_PATH").unwrap());
    let api = routes![api::hello::hello, api::datasources::datasources,];
    rocket::build()
        .manage(pool)
        .mount(
            "/",
            FileServer::from(env::var("STATIC_FILES_DIR").unwrap() + "/"),
        )
        .mount("/api", api)
        .launch()
        .await;
}
