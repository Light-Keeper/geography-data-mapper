use crate::db::DbPool;
use diesel::{insert_into, sql_function, sql_types, Connection, ExpressionMethods, RunQueryDsl};
use rand::Rng;
use serde_json::json;

sql_function!(fn last_insert_rowid() -> sql_types::Integer);

fn random_location() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let lat = rng.gen_range(-90.0..90.0);
    let lon = rng.gen_range(-180.0..180.0);
    (lat, lon)
}

pub async fn generate(name: String, count: u32, color: String, pool: DbPool) {
    use crate::schema::datasources::dsl;

    let conn = &mut pool.get().unwrap();

    conn.transaction::<(), diesel::result::Error, _>(|conn| {
        insert_into(dsl::datasources)
            .values(dsl::name.eq(&name))
            .execute(conn)
            .expect("Failed to insert new datasource");

        let row_id: i32 = diesel::select(last_insert_rowid())
            .get_result(conn)
            .unwrap();

        use crate::schema::datapoints::dsl as dsl2;

        for i in 0..count {
            let (lat, lon) = random_location();

            insert_into(dsl2::datapoints)
                .values((
                    dsl2::name.eq(format!("{}-{}", &name, i)),
                    dsl2::datasource_id.eq(row_id),
                    dsl2::lat.eq(lat),
                    dsl2::lng.eq(lon),
                    dsl2::tags.eq(json!({"color": color}).to_string()),
                ))
                .execute(conn)
                .expect("Failed to insert new datapoint");
        }
        Ok(())
    })
    .unwrap()
}
