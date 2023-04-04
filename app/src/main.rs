mod config;
mod db;
mod generator;
mod models;
mod schema;
mod server;

#[macro_use]
extern crate rocket;

use crate::config::Commands;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    let app_config = config::parse_config();
    let pool = db::prepare_database(&app_config.sqlite_db_path);

    match app_config.cli.command {
        Commands::Server => server::start_server(app_config, pool).await,

        Commands::Generate { name, count, color, country } => {
            generator::generate(name, count, color, country, pool).await
        }
    }
}
