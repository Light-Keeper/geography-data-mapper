use crate::config::AppConfig;
use crate::db::DbPool;
use rocket::fs::FileServer;

mod api;

#[allow(unused_must_use)]
pub async fn start_server(cfg: AppConfig, pool: DbPool) {
    let api = routes![
        api::hello::hello,
        api::datasources::datasources,
        api::datapoints::datapoints
    ];

    rocket::build()
        .manage(pool)
        .mount("/", FileServer::from(cfg.static_files_dir))
        .mount("/api", api)
        .launch()
        .await;
}
