use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;
use std::path::Path;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn prepare_database(db_file_path: &str) -> DbPool {
    std::fs::create_dir_all(Path::new(db_file_path).parent().unwrap()).unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(db_file_path);
    let pool = Pool::builder().build(manager).unwrap();
    let mut connection = pool.get().unwrap();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    return pool;
}
