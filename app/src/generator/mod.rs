mod geogen;

use crate::db::DbPool;
use diesel::{insert_into, sql_function, sql_types, Connection, ExpressionMethods, RunQueryDsl, SqliteConnection};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde_json::json;
use crate::generator::geogen::GeoGen;

sql_function!(fn last_insert_rowid() -> sql_types::Integer);

type Conn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub async fn generate(name: String, count: u32, color: String, country: String, pool: DbPool) {
    let conn = &mut pool.get().unwrap();
    conn.transaction::<(), diesel::result::Error, _>(
        |conn| {
            generate_in_transaction(conn, name, count, color, country); Ok(()) }
    ).unwrap()
}

fn generate_in_transaction(conn: &mut Conn, name: String, count: u32, color: String, country: String) {
    use crate::schema::datasources::dsl;

    insert_into(dsl::datasources)
        .values(dsl::name.eq(&name))
        .execute(conn)
        .expect("Failed to insert new datasource");

    let row_id: i32 = diesel::select(last_insert_rowid())
        .get_result(conn)
        .unwrap();

    use crate::schema::datapoints::dsl as dsl2;

    let generator = GeoGen::new(&country);

    for i in 0..count {
        let (lng, lat) = generator.random_point();

        insert_into(dsl2::datapoints)
            .values((
                dsl2::name.eq(format!("{}-{}", &name, i)),
                dsl2::datasource_id.eq(row_id),
                dsl2::lat.eq(lat),
                dsl2::lng.eq(lng),
                dsl2::tags.eq(json!({"color": color}).to_string()),
            ))
            .execute(conn)
            .expect("Failed to insert new datapoint");
    }
}
