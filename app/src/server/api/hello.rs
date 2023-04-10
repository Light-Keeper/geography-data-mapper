use crate::db::DbPool;
use rocket::State;

#[get("/hello")]
pub fn hello(_db: &State<DbPool>) -> String {
    format!("Hello, world! SQLite version: {}", "123")
}
