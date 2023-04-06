use anyhow::anyhow;
use geo::Contains;
use geo_types::Coord;
use geojson::GeoJson;
use rand::Rng;
use crate::db::DbPool;

pub fn generate(name: String, count: usize, color: String, country: String, pool: DbPool) -> anyhow::Result<()> {
    let mut conn = pool.get()?;

    //language=SQLite
    let query = r#"
        SELECT gf.geometry, gf.name, gf.type, gf.bbox
        FROM geo_feature_property p join geo_feature gf on gf.id = p.geo_feature_id
        WHERE p.value = ?1 AND p.key LIKE 'NAME_%'
        LIMIT  1"#;

    let (geo, can_name, tp, bbox) =
        conn.query_row(query,
                       (&country, ),
                       |row| Ok((
                           row.get::<_, String>(0)?,
                           row.get::<_, String>(1)?,
                           row.get::<_, String>(2)?,
                           row.get::<_, String>(3)?,
                       )))
            .map_err(|e| anyhow!("Failed to find county by name. {}", e))?;

    println!("Generating {} random points for {} {}", count, tp, can_name);

    let tx = conn.transaction()?;

    let dataset_id: usize = tx.query_row(r#"
        INSERT INTO datasets (name)
        VALUES (?1)
        RETURNING id"#, (name, ), |r| r.get(0))?;

    let geometry: geo_types::Geometry<f32> = serde_json::from_str::<GeoJson>(&geo)?.try_into()?;
    let mut rng = rand::thread_rng();
    let (minx, miny, maxx, maxy) = serde_json::from_str::<(f32, f32, f32, f32)>(&bbox)?;

    let points = (0..)
        .map(|_| (rng.gen_range(minx..=maxx), rng.gen_range(miny..=maxy)))
        .filter(|&(x, y)| geometry.contains(&Coord { x, y }))
        .take(count);

    for (x, y) in points {
        //language=SQLite
        let datapoint_id: usize = tx.query_row(r#"
            INSERT INTO datapoints (dataset_id, lng, lat)
            VALUES (?1, ?2, ?3)
            RETURNING id"#, (dataset_id, x, y), |r| r.get(0))?;

        tx.execute(r#"
            INSERT INTO attributes (dataset_id, datapoint_id, name, value)
            VALUES (?1, ?2, ?3, ?4)"#, (
            dataset_id, datapoint_id, "Name", format!("Datapoint {}", datapoint_id))
        )?;

        tx.execute(r#"
            INSERT INTO attributes (dataset_id, datapoint_id, name, value)
            VALUES (?1, ?2, ?3, ?4)"#, (
            dataset_id, datapoint_id, "Color", &color)
        )?;
    }

    tx.commit()?;
    Ok(())
}


