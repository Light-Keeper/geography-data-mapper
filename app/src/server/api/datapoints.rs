use crate::db::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::value::RawValue;
use crate::server::dto::{Datapoint, Page};

fn construct_order_by_sql(order: &Vec<String>) -> String {
    let mut order_by_clauses: Vec<String> = Vec::new();

    for o in order {
        let split_param: Vec<&str> = o.split(':').collect();

        if split_param.len() != 2 {
            continue;
        }

        let column = split_param[0];
        let direction = split_param[1].to_uppercase();

        order_by_clauses.push(format!(
            "(SELECT a.value FROM attributes a WHERE a.datapoint_id = d.id AND a.name = '{}') {}",
            column, direction
        ));
    }

    if !order_by_clauses.is_empty() {
        format!("ORDER BY {}", order_by_clauses.join(", "))
    } else {
        String::new()
    }
}


#[get("/datapoints?<dataset>&<limit>&<offset>&<order_by>")]
pub fn datapoints(
    dataset: u32,
    limit: Option<u32>,
    offset: Option<u32>,
    order_by: Vec<String>,
    db: &State<DbPool>) -> Json<Page<Datapoint>> {

    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);
    let order_by_clause = construct_order_by_sql(&order_by);

    let conn = db.get().unwrap();

    let mut statemet = conn.prepare(format!(r#"
            SELECT d.lat, d.lng, x'7B' || GROUP_CONCAT( json_quote(a.name) || ':' || json_quote(a.value), ',' ) || x'7D'
            FROM datapoints d
                JOIN attributes a on a.datapoint_id = d.id
            WHERE d.dataset_id = ?1
            GROUP BY d.id
            {order_by_clause}
            limit {limit}
            offset {offset}
        "#).as_str()).unwrap();

    dbg!(&statemet);
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
