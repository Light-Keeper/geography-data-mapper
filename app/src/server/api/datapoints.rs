use r2d2::PooledConnection;
use crate::db::{DbPool, SqliteConnectionManager};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::value::RawValue;
use crate::server::dto::{Datapoint, Page};
use rocket::form::Form;
use rusqlite::{params, Params, params_from_iter, Row, Rows, Statement, ToSql};
use serde::de::Error;

#[derive(Debug, FromForm)]
pub struct QueryProcessor<'a> {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub order_by: Option<&'a str>,
    pub from_lng: Option<f32>,
    pub to_lng: Option<f32>,
    pub from_lat: Option<f32>,
    pub to_lat: Option<f32>,
    pub dataset: i32,
}

impl QueryProcessor<'_> {
    pub fn query(&self) -> (String, Vec<&dyn ToSql>) {
        //language=SQLite
        let query = r#"
        SELECT
            d.lat,
            d.lng,
            x'7B' || GROUP_CONCAT( json_quote(a.name) || ':' || json_quote(a.value), ',' ) || x'7D'
        FROM attributes a
             JOIN datapoints d ON d.id = a.datapoint_id
             JOIN (
                SELECT datapoint_id, value
                FROM attributes
                WHERE dataset_id = ?1 AND name = ?2
            ) ord ON ord.datapoint_id = d.id
        WHERE a.dataset_id = ?1
        GROUP BY d.id, ord.value
        ORDER BY ord.value DESC
        LIMIT ?3
        OFFSET ?4
        "#;


        let dataset = &self.dataset;
        let order_by = self.order_by.unwrap();
        let limit = self.limit.unwrap_or(100);
        let offset = self.offset.unwrap_or(0);

        (
            String::from(query),
            vec![
                dataset as &dyn ToSql,
                order_by as &dyn ToSql
            ]
        )
    }
}

#[get("/datapoints?<qparams..>")]
pub fn datapoints(qparams: QueryProcessor, db: &State<DbPool>) -> Json<Page<Datapoint>> {
    let connection = db.get().unwrap();
    let (query, params) = qparams.query();

    let mut stmt = connection.prepare(&query).unwrap();
    let rows = stmt.query(params.as_slice()).unwrap();

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
