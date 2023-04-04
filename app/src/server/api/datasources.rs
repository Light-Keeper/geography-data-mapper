use crate::db::DbPool;
use crate::models::Datasource;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/datasources")]
pub fn datasources(db: &State<DbPool>) -> Json<Vec<Datasource>> {
    use crate::schema::datasources::dsl::*;

    let mut connection = db.get().unwrap();

    let all_datasources: Vec<_> = datasources
        .order(id.asc())
        .load::<Datasource>(&mut connection)
        .expect("Error loading datasources");

    return Json(all_datasources);
}
