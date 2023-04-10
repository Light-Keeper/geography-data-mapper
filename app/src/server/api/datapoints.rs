use crate::db::DbPool;
use crate::server::dto::{Datapoint, Page};
use clap::builder::Str;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::params_from_iter;
use serde_json::value::RawValue;

#[derive(Debug, FromForm)]
pub struct QueryProcessor<'a> {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub order_by: Option<&'a str>,
    pub lat_max: Option<f32>,
    pub lat_min: Option<f32>,
    pub lng_max: Option<f32>,
    pub lng_min: Option<f32>,
    pub dataset: i32,
}

impl QueryProcessor<'_> {
    pub fn query(&self) -> (String, Vec<String>) {
        match self.order_by {
            None => self.query_no_ordering(),
            Some(_) => self.query_with_ordering(),
        }
    }

    fn query_no_ordering(&self) -> (String, Vec<String>) {
        //language=SQLite
        let query = format!(
            r#"
            SELECT
                d.lat,
                d.lng,
                x'7B' || GROUP_CONCAT( json_quote(a.name) || ':' || json_quote(a.value), ',' ) || x'7D'
            FROM attributes a
                 JOIN datapoints d ON d.id = a.datapoint_id
            WHERE a.dataset_id = ?1 AND {}
            GROUP BY d.id
            ORDER BY d.id
            LIMIT ?2
            OFFSET ?3
            "#,
            self.bbox_to_sql()
        );

        let dataset = self.dataset;
        let limit = self.limit.unwrap_or(100);
        let offset = self.offset.unwrap_or(0);

        (
            String::from(query),
            vec![dataset.to_string(), limit.to_string(), offset.to_string()],
        )
    }

    fn query_with_ordering(&self) -> (String, Vec<String>) {
        let dataset = self.dataset;
        let order_by = self.order_by.unwrap();
        let limit = self.limit.unwrap_or(100);
        let offset = self.offset.unwrap_or(0);

        let mut ord = order_by
            .splitn(2, ':')
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        if ord.len() < 2 {
            ord.push(String::from("ASC"))
        }

        let ord_direction = ord.pop().unwrap().to_uppercase();
        let ord_field = ord.pop().unwrap();

        if ord_direction != "ASC" && ord_direction != "DESC" {
            todo!("Properly handle invalid parameters on a public method level")
        }

        //language=SQLite
        let query = format!(
            r#"
            with ord as (
             SELECT datapoint_id, value
                    FROM attributes JOIN main.datapoints d2 on d2.id = attributes.datapoint_id
                    WHERE attributes.dataset_id = ?1 AND name = ?2 and {}
                    ORDER BY value {}
                    LIMIT ?3
            )
            SELECT
                d.lat,
                d.lng,
                x'7B' || GROUP_CONCAT( json_quote(a.name) || ':' || json_quote(a.value), ',' ) || x'7D'
            FROM attributes a
                 JOIN datapoints d ON d.id = a.datapoint_id
                 JOIN ord ON ord.datapoint_id = d.id
            GROUP BY d.id, ord.value
            ORDER BY ord.value {}
            LIMIT ?3
            OFFSET ?4
            "#,
            self.bbox_to_sql(),
            ord_direction,
            ord_direction
        );

        (
            query,
            vec![
                dataset.to_string(),
                ord_field,
                limit.to_string(),
                offset.to_string(),
            ],
        )
    }

    fn bbox_to_sql(&self) -> String {
        let x = self.lng_min.unwrap_or(-10000f32);
        let y = self.lat_min.unwrap_or(-10000f32);
        let xx = self.lng_max.unwrap_or(-10000f32);
        let yy = self.lat_max.unwrap_or(-10000f32);

        if x.min(y).min(xx).min(yy) < -1000f32 {
            return String::from("TRUE");
        }

        return format!(
            "{} <= lng AND lng <= {} AND {} <= lat AND lat <= {}",
            x, xx, y, yy
        );
    }
}

#[get("/datapoints?<qparams..>")]
pub fn datapoints(qparams: QueryProcessor, db: &State<DbPool>) -> Json<Page<Datapoint>> {
    let connection = db.get().unwrap();
    let (query, params) = qparams.query();

    let mut stmt = connection.prepare(&query).unwrap();
    let rows = stmt.query(params_from_iter(params.into_iter())).unwrap();

    let vec1 = rows
        .mapped(|r| {
            Ok(Datapoint {
                lat: r.get(0)?,
                lng: r.get(1)?,
                tags: RawValue::from_string(r.get::<_, String>(2)?).unwrap(),
            })
        })
        .map(|x| x.unwrap())
        .collect();

    return Json(Page {
        limit: 0,
        offset: 0,
        more: false,
        data: vec1,
    });
}
