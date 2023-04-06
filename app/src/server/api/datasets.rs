use crate::db::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use crate::server::dto::{Datasource, Page};

#[get("/datasets")]
pub fn datasets(db: &State<DbPool>) -> Json<Page<Datasource>> {
    let conn = db.get().unwrap();

    //language=SQLite
    let sql = r#"SELECT id, name FROM datasets"#;

    let data = conn
        .prepare_cached(sql).unwrap()
        .query([]).unwrap()
        .mapped(|row| {
            let id: usize = row.get(0)?;
            let name: String = row.get(1)?;
            Ok(Datasource { id, name })
        })
        .map(|r| r.unwrap())
        .collect();

    Json(Page {
        limit: 0,
        offset: 0,
        more: false,
        data
    })
}
