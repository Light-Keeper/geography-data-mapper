use crate::db::DbPool;
use crate::models::Datasource;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/datasources")]
pub fn datasources(db: &State<DbPool>) -> Json<Vec<Datasource>> {
    let mut connection = db.get().unwrap();

    let all_datasources: Vec<_> = schema::datasources::table
        .order(schema::datasources::id.asc())
        .load::<Datasource>(&mut connection)
        .expect("Error loading datasources");

    return Json(all_datasources);
}
