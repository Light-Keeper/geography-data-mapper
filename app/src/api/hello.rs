use crate::db::DbPool;
use diesel::prelude::*;
use rocket::State;

#[get("/hello")]
pub fn hello(db: &State<DbPool>) -> String {
    let mut connection = db.get().unwrap();

    let version: String = diesel::select(diesel::dsl::sql::<diesel::sql_types::Text>(
        "sqlite_version()",
    ))
    .get_result(&mut connection)
    .unwrap();

    return format!("Hello, world! SQLite version: {}", version);
}
