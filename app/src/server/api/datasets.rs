use crate::db::DbPool;
use crate::server::dto::{Datasource, Page};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::value::RawValue;

#[get("/datasets")]
pub fn datasets(db: &State<DbPool>) -> Json<Page<Datasource>> {
    let conn = db.get().unwrap();

    //language=SQLite
    let sql = r#"SELECT id, name, metadata FROM datasets"#;

    let data = conn
        .prepare_cached(sql)
        .unwrap()
        .query([])
        .unwrap()
        .mapped(|row| {
            let id: usize = row.get(0)?;
            let name: String = row.get(1)?;
            let metadata: String = row.get(2)?;
            Ok(Datasource {
                id,
                name,
                metadata: RawValue::from_string(metadata).unwrap(),
            })
        })
        .map(|r| r.unwrap())
        .collect();

    Json(Page {
        limit: 0,
        offset: 0,
        more: false,
        data,
    })
}
