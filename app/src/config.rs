use clap::{Parser, Subcommand};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub sqlite_db_path: String,
    pub static_files_dir: String,
    pub cli: Cli,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// starts server
    Server,

    /// generates fake data
    Generate {
        /// number of points to generate
        #[arg(long, default_value = "100")]
        count: u32,

        /// color of points. Separate with slash to pick a random color from the list
        #[arg(long, default_value = "yellow/blue")]
        color: String,

        /// name of datasource
        #[arg(long)]
        name: String,
    },
}

pub fn parse_config() -> AppConfig {
    let cli = Cli::parse();

    AppConfig {
        sqlite_db_path: env::var("SQLITE_DB_PATH").expect("SQLITE_DB_PATH not set"),
        static_files_dir: env::var("STATIC_FILES_DIR").expect("STATIC_FILES_DIR not set"),
        cli,
    }
}
