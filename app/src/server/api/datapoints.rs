use crate::db::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::value::RawValue;
use crate::server::dto::{Datapoint, Page};

#[get("/datapoints?<dataset>")]
pub fn datapoints(dataset: u32, db: &State<DbPool>) -> Json<Page<Datapoint>> {
    let conn = db.get().unwrap();
    let mut statemet = conn.prepare(r#"
            SELECT d.lat, d.lng, '{' || GROUP_CONCAT( json_quote(a.name) || ':' || json_quote(a.value), ',' ) || '}'
            FROM datapoints d
                JOIN attributes a on a.datapoint_id = d.id
            WHERE d.dataset_id = ?1
            GROUP BY d.id
            "#).unwrap();

    let rows = statemet.query([dataset]).unwrap();

    let vec1 = rows
        .mapped(|r| Ok(Datapoint {
            lat: r.get(0)?,
            lng: r.get(1)?,
            tags: RawValue::from_string(r.get::<_, String>(2)?).unwrap(),
        }))
        .map(|x|x.unwrap())
        .collect();

    return Json(Page {
        limit: 0,
        offset: 0,
        more: false,
        data: vec1,
    });
}
