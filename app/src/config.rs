use anyhow::Context;
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
        count: usize,

        /// color of points. Separate with slash to pick a random color from the list
        #[arg(long, default_value = "yellow/blue")]
        color: String,

        /// name of datasource
        #[arg(long)]
        name: String,

        /// Country where points will be generated
        #[arg(long)]
        country: String,
    },

    /// run all pending migrations
    Migrate {
        /// clean entire database and run migrations from scratch. it will delete all the data!!!
        #[arg(long)]
        clean: bool,
    },

    // import data from external formats
    ImportGeoJSON {
        /// file with data to import
        file: String,
    },

    // import data from external formats
    ImportGeoNames {
        /// name of dataset
        #[arg(long)]
        name: String,

        /// file with data to import
        file: String,
    },
}

pub fn parse_config() -> anyhow::Result<AppConfig> {
    let cli = Cli::parse();

    Ok(AppConfig {
        sqlite_db_path: env::var("SQLITE_DB_PATH").context("SQLITE_DB_PATH not set")?,
        static_files_dir: env::var("STATIC_FILES_DIR").context("STATIC_FILES_DIR not set")?,
        cli,
    })
}
