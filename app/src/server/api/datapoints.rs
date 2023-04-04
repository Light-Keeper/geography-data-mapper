use crate::db::DbPool;
use crate::models::Datapoint;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/datapoints?<datasource_id>")]
pub fn datapoints(datasource_id: Option<u32>, db: &State<DbPool>) -> Json<Vec<Datapoint>> {
    use crate::schema::datapoints::dsl;

    let mut conn = db.get().unwrap();

    let v = if let Some(id) = datasource_id {
        dsl::datapoints.filter(dsl::datasource_id.eq(id as i32)).load(&mut conn)
    } else {
        dsl::datapoints.load(&mut conn)
    };


    return Json(v.unwrap());
}
