use rusqlite::Connection;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};

enum ConnectionConfig {
    File(PathBuf),
    Memory,
}

pub struct SqliteConnectionManager(ConnectionConfig);

impl SqliteConnectionManager {
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        SqliteConnectionManager(ConnectionConfig::File(path.as_ref().to_path_buf()))
    }

    #[allow(unused)]
    pub fn memory() -> Self {
        SqliteConnectionManager(ConnectionConfig::Memory)
    }
}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match self.0 {
            ConnectionConfig::File(ref path) => Connection::open(path),
            ConnectionConfig::Memory => Connection::open(":memory:"),
        }
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), Self::Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

pub fn prepare_database(db_file_path: &str) -> anyhow::Result<DbPool> {
    match Path::new(db_file_path).parent() {
        None => {}
        Some(p) => {
            std::fs::create_dir_all(p)?;
        }
    }

    let manager = SqliteConnectionManager::file(db_file_path);
    Ok(r2d2::Pool::builder().build(manager)?)
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("priv/migrations");
}

pub fn migrate(db: DbPool, clean: bool) -> anyhow::Result<()> {
    let mut conn = db.get()?;
    if clean {
        println!("Cleaning up database...");

        conn.execute_batch(
            r#"
            PRAGMA writable_schema = 1;
            DELETE FROM sqlite_master WHERE type IN ('table', 'index', 'trigger');
            PRAGMA writable_schema = 0;
            VACUUM;
            PRAGMA INTEGRITY_CHECK;
        "#,
        )?;

        println!("Database successfully cleaned");
    }

    embedded::migrations::runner().run(conn.deref_mut())?;
    println!("Successfully migrated");
    Ok(())
}
