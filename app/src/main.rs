#[macro_use] extern crate rocket;
use rocket::fs::FileServer;

use std::env;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    dotenv::dotenv().ok();

    rocket::build()
        .mount("/", FileServer::from(env::var("STATIC_FILES_DIR").unwrap() + "/"))
        .mount("/", routes![hello])
        .launch().await;
}